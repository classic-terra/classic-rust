package testenv

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"

	// helpers

	// tendermint
	abci "github.com/tendermint/tendermint/abci/types"
	"github.com/tendermint/tendermint/libs/log"
	tmtypes "github.com/tendermint/tendermint/proto/tendermint/types"
	dbm "github.com/tendermint/tm-db"

	// cosmos-sdk
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	"github.com/cosmos/cosmos-sdk/server"
	"github.com/cosmos/cosmos-sdk/simapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/errors"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	"github.com/cosmos/cosmos-sdk/x/staking"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"

	// wasmd
	"github.com/CosmWasm/wasmd/x/wasm"

	// classic
	"github.com/classic-terra/core/v2/app"
	coretypes "github.com/classic-terra/core/v2/types"
	markettypes "github.com/classic-terra/core/v2/x/market/types"
	oracletypes "github.com/classic-terra/core/v2/x/oracle/types"
	treasurytypes "github.com/classic-terra/core/v2/x/treasury/types"
)

type TestEnv struct {
	App                *app.TerraApp
	Ctx                sdk.Context
	ParamTypesRegistry ParamTypeRegistry
	ValPrivs           []*secp256k1.PrivKey
}

// DebugAppOptions is a stub implementing AppOptions
type DebugAppOptions struct{}

// Get implements AppOptions
func (ao DebugAppOptions) Get(o string) interface{} {
	if o == server.FlagTrace {
		return true
	}
	return nil
}

func SetupTerraApp() *app.TerraApp {
	db := dbm.NewMemDB()
	var emptyWasmOpts []wasm.Option
	encCfg := app.MakeEncodingConfig()

	appInstance := app.NewTerraApp(
		log.NewNopLogger(),
		db,
		nil,
		true,
		map[int64]bool{},
		app.DefaultNodeHome,
		5,
		encCfg,
		DebugAppOptions{},
		emptyWasmOpts,
	)

	// Genesis setup
	genesisState := app.NewDefaultGenesisState()

	// setup oracle genesis to have some exchange prices
	exchangeRates := []oracletypes.ExchangeRateTuple{
		{
			// the dream of 1 luna to be worth 1 usd some day
			Denom:        coretypes.MicroUSDDenom,
			ExchangeRate: sdk.MustNewDecFromStr("1.0"),
		},
		{
			// 1 uluna = 0.7 usdr
			Denom:        coretypes.MicroSDRDenom,
			ExchangeRate: sdk.MustNewDecFromStr("0.7"),
		},
	}

	oracleGen := oracletypes.GenesisState{
		Params:                        oracletypes.DefaultParams(),
		FeederDelegations:             []oracletypes.FeederDelegation{},
		ExchangeRates:                 exchangeRates,
		MissCounters:                  []oracletypes.MissCounter{},
		AggregateExchangeRatePrevotes: []oracletypes.AggregateExchangeRatePrevote{},
		AggregateExchangeRateVotes:    []oracletypes.AggregateExchangeRateVote{},
		TobinTaxes:                    []oracletypes.TobinTax{},
	}
	genesisState[oracletypes.ModuleName] = encCfg.Marshaler.MustMarshalJSON(&oracleGen)

	stateBytes, err := json.MarshalIndent(genesisState, "", " ")

	requireNoErr(err)

	concensusParams := simapp.DefaultConsensusParams
	concensusParams.Block = &abci.BlockParams{
		MaxBytes: 22020096,
		MaxGas:   -1,
	}

	// replace sdk.DefaultDenom with "uosmo", a bit of a hack, needs improvement
	stateBytes = []byte(strings.Replace(string(stateBytes), "\"stake\"", "\"uluna\"", -1))

	appInstance.InitChain(
		abci.RequestInitChain{
			Validators:      []abci.ValidatorUpdate{},
			ConsensusParams: concensusParams,
			AppStateBytes:   stateBytes,
		},
	)

	return appInstance
}

func (env *TestEnv) BeginNewBlock(executeNextEpoch bool, timeIncreaseSeconds uint64) {
	var valAddr []byte

	validators := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	if len(validators) >= 1 {
		valAddrFancy, err := validators[0].GetConsAddr()
		requireNoErr(err)
		valAddr = valAddrFancy.Bytes()
	} else {
		valPriv, valAddrFancy := env.setupValidator(stakingtypes.Bonded)
		validator, _ := env.App.StakingKeeper.GetValidator(env.Ctx, valAddrFancy)
		valAddr2, _ := validator.GetConsAddr()
		valAddr = valAddr2.Bytes()

		env.ValPrivs = append(env.ValPrivs, valPriv)
		err := simapp.FundAccount(env.App.BankKeeper, env.Ctx, valAddrFancy.Bytes(), sdk.NewCoins(sdk.NewInt64Coin("uosmo", 9223372036854775807)))
		if err != nil {
			panic(errors.Wrapf(err, "Failed to fund account"))
		}
	}

	env.beginNewBlockWithProposer(executeNextEpoch, valAddr, timeIncreaseSeconds)
}

func (env *TestEnv) GetValidatorAddresses() []string {
	validators := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	var addresses []string
	for _, validator := range validators {
		addresses = append(addresses, validator.OperatorAddress)
	}

	return addresses
}

// beginNewBlockWithProposer begins a new block with a proposer.
func (env *TestEnv) beginNewBlockWithProposer(executeNextEpoch bool, proposer sdk.ValAddress, timeIncreaseSeconds uint64) {
	validator, found := env.App.StakingKeeper.GetValidator(env.Ctx, proposer)

	if !found {
		panic("validator not found")
	}

	valConsAddr, err := validator.GetConsAddr()
	requireNoErr(err)

	valAddr := valConsAddr.Bytes()

	newBlockTime := env.Ctx.BlockTime().Add(time.Duration(timeIncreaseSeconds) * time.Second)

	header := tmtypes.Header{ChainID: "columbus-5", Height: env.Ctx.BlockHeight() + 1, Time: newBlockTime}
	newCtx := env.Ctx.WithBlockTime(newBlockTime).WithBlockHeight(env.Ctx.BlockHeight() + 1)
	env.Ctx = newCtx
	lastCommitInfo := abci.LastCommitInfo{
		Votes: []abci.VoteInfo{{
			Validator:       abci.Validator{Address: valAddr, Power: 1000},
			SignedLastBlock: true,
		}},
	}
	reqBeginBlock := abci.RequestBeginBlock{Header: header, LastCommitInfo: lastCommitInfo}

	env.App.BeginBlock(reqBeginBlock)
	env.Ctx = env.App.NewContext(false, reqBeginBlock.Header)
}

func (env *TestEnv) setupValidator(bondStatus stakingtypes.BondStatus) (*secp256k1.PrivKey, sdk.ValAddress) {
	valPriv := secp256k1.GenPrivKey()
	valPub := valPriv.PubKey()
	valAddr := sdk.ValAddress(valPub.Address())
	bondDenom := env.App.StakingKeeper.GetParams(env.Ctx).BondDenom
	selfBond := sdk.NewCoins(sdk.Coin{Amount: sdk.NewInt(100), Denom: bondDenom})

	err := simapp.FundAccount(env.App.BankKeeper, env.Ctx, sdk.AccAddress(valPub.Address()), selfBond)
	requireNoErr(err)

	stakingHandler := staking.NewHandler(env.App.StakingKeeper)
	stakingCoin := sdk.NewCoin(bondDenom, selfBond[0].Amount)
	ZeroCommission := stakingtypes.NewCommissionRates(sdk.ZeroDec(), sdk.ZeroDec(), sdk.ZeroDec())
	msg, err := stakingtypes.NewMsgCreateValidator(valAddr, valPub, stakingCoin, stakingtypes.Description{}, ZeroCommission, sdk.OneInt())
	requireNoErr(err)
	res, err := stakingHandler(env.Ctx, msg)
	requireNoErr(err)
	requireNoNil("staking handler", res)

	env.App.BankKeeper.SendCoinsFromModuleToModule(env.Ctx, stakingtypes.NotBondedPoolName, stakingtypes.BondedPoolName, sdk.NewCoins(stakingCoin))

	val, found := env.App.StakingKeeper.GetValidator(env.Ctx, valAddr)
	requierTrue("validator found", found)

	val = val.UpdateStatus(bondStatus)
	env.App.StakingKeeper.SetValidator(env.Ctx, val)

	consAddr, err := val.GetConsAddr()
	requireNoErr(err)

	signingInfo := slashingtypes.NewValidatorSigningInfo(
		consAddr,
		env.Ctx.BlockHeight(),
		0,
		time.Unix(0, 0),
		false,
		0,
	)
	env.App.SlashingKeeper.SetValidatorSigningInfo(env.Ctx, consAddr, signingInfo)

	return valPriv, valAddr
}

func (env *TestEnv) SetupParamTypes() {
	pReg := env.ParamTypesRegistry

	pReg.RegisterParamSet(&oracletypes.Params{})
	pReg.RegisterParamSet(&treasurytypes.Params{})
	pReg.RegisterParamSet(&markettypes.Params{})
}

func requireNoErr(err error) {
	if err != nil {
		panic(err)
	}
}

func requireNoNil(name string, nilable any) {
	if nilable == nil {
		panic(fmt.Sprintf("%s must not be nil", name))
	}
}

func requierTrue(name string, b bool) {
	if !b {
		panic(fmt.Sprintf("%s must be true", name))
	}
}
