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
	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
	tmtypes "github.com/tendermint/tendermint/types"
	dbm "github.com/tendermint/tm-db"

	// cosmos-sdk
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	cryptocodec "github.com/cosmos/cosmos-sdk/crypto/codec"
	"github.com/cosmos/cosmos-sdk/crypto/keys/ed25519"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	"github.com/cosmos/cosmos-sdk/server"
	"github.com/cosmos/cosmos-sdk/simapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/errors"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	banktest "github.com/cosmos/cosmos-sdk/x/bank/testutil"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	stakingkeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"

	// wasmd
	"github.com/CosmWasm/wasmd/x/wasm"

	// classic
	"github.com/classic-terra/core/v2/app"
	appparams "github.com/classic-terra/core/v2/app/params"
	apptesting "github.com/classic-terra/core/v2/app/testing"
	coretypes "github.com/classic-terra/core/v2/types"
	fork "github.com/classic-terra/core/v2/types/fork"
	markettypes "github.com/classic-terra/core/v2/x/market/types"
	oracletypes "github.com/classic-terra/core/v2/x/oracle/types"
	treasurytypes "github.com/classic-terra/core/v2/x/treasury/types"
)

type TestEnv struct {
	App                *app.TerraApp
	Ctx                sdk.Context
	ParamTypesRegistry ParamTypeRegistry
	ValPrivs           []*secp256k1.PrivKey
	NodeHome           string
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

func SetupTerraApp(nodeHome string) *app.TerraApp {
	db := dbm.NewMemDB()
	var emptyWasmOpts []wasm.Option
	encCfg := app.MakeEncodingConfig()

	appInstance := app.NewTerraApp(
		log.NewNopLogger(),
		db,
		nil,
		true,
		map[int64]bool{},
		nodeHome,
		5,
		encCfg,
		DebugAppOptions{},
		emptyWasmOpts,
	)

	// Genesis setup
	genesisState := app.NewDefaultGenesisState()

	// setup validator genesis
	senderPrivKey := secp256k1.GenPrivKey()
	acc := authtypes.NewBaseAccount(senderPrivKey.PubKey().Address().Bytes(), senderPrivKey.PubKey(), 0, 0)
	balance := banktypes.Balance{
		Address: acc.GetAddress().String(),
		Coins:   sdk.NewCoins(sdk.NewCoin(appparams.BondDenom, sdk.NewInt(100000000000000))),
	}

	// create validator set with single validator
	privVal := ed25519.GenPrivKey()
	pubKey, err := cryptocodec.ToTmPubKeyInterface(privVal.PubKey())
	requireNoErr(err)
	validator := tmtypes.NewValidator(pubKey, 1)
	valSet := tmtypes.NewValidatorSet([]*tmtypes.Validator{validator})
	genesisState = genesisStateWithValSet(appInstance, genesisState, valSet, []authtypes.GenesisAccount{acc}, balance)

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

	// setup treasury genesis to have some tax cap
	treasuryGen := treasurytypes.GenesisState{
		Params:       treasurytypes.DefaultParams(),
		TaxRate:      treasurytypes.DefaultTaxRate,
		RewardWeight: treasurytypes.DefaultRewardWeight,
		TaxCaps: []treasurytypes.TaxCap{
			{
				// this is mainnet value
				Denom:  coretypes.MicroLunaDenom,
				TaxCap: sdk.NewInt(60000000000000000),
			},
			{
				Denom:  coretypes.MicroUSDDenom,
				TaxCap: sdk.NewInt(79835432561680706),
			},
			{
				Denom:  coretypes.MicroSDRDenom,
				TaxCap: sdk.NewInt(1000000),
			},
		},
		TaxProceeds:          sdk.Coins{},
		EpochInitialIssuance: sdk.Coins{},
		EpochStates:          []treasurytypes.EpochState{},
	}
	genesisState[treasurytypes.ModuleName] = encCfg.Marshaler.MustMarshalJSON(&treasuryGen)

	stateBytes, err := json.MarshalIndent(genesisState, "", " ")

	requireNoErr(err)

	concensusParams := simapp.DefaultConsensusParams
	concensusParams.Block = &abci.BlockParams{
		MaxBytes: 22020096,
		MaxGas:   -1,
	}

	// replace sdk.DefaultDenom with "uluna", a bit of a hack, needs improvement
	stateBytes = []byte(strings.Replace(string(stateBytes), "\"stake\"", "\"uluna\"", -1))

	appInstance.InitChain(
		abci.RequestInitChain{
			ChainId:         apptesting.SimAppChainID,
			Validators:      []abci.ValidatorUpdate{},
			InitialHeight:   fork.VersionMapEnableHeight,
			ConsensusParams: concensusParams,
			AppStateBytes:   stateBytes,
		},
	)

	appInstance.Commit()
	appInstance.BeginBlock(abci.RequestBeginBlock{Header: tmproto.Header{
		ChainID:            apptesting.SimAppChainID,
		Height:             appInstance.LastBlockHeight() + 1,
		AppHash:            appInstance.LastCommitID().Hash,
		ValidatorsHash:     valSet.Hash(),
		NextValidatorsHash: valSet.Hash(),
	}})

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
		err := banktest.FundAccount(env.App.BankKeeper, env.Ctx, valAddrFancy.Bytes(), sdk.NewCoins(sdk.NewInt64Coin("uluna", 9223372036854775807)))
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

	header := tmproto.Header{ChainID: apptesting.SimAppChainID, Height: env.Ctx.BlockHeight() + 1, Time: newBlockTime}
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

	err := banktest.FundAccount(env.App.BankKeeper, env.Ctx, sdk.AccAddress(valPub.Address()), selfBond)
	requireNoErr(err)

	stakingMsgServer := stakingkeeper.NewMsgServerImpl(env.App.StakingKeeper)
	stakingCoin := sdk.NewCoin(bondDenom, selfBond[0].Amount)
	ZeroCommission := stakingtypes.NewCommissionRates(sdk.ZeroDec(), sdk.ZeroDec(), sdk.ZeroDec())
	msg, err := stakingtypes.NewMsgCreateValidator(valAddr, valPub, stakingCoin, stakingtypes.Description{}, ZeroCommission, sdk.OneInt())
	requireNoErr(err)
	res, err := stakingMsgServer.CreateValidator(sdk.WrapSDKContext(env.Ctx), msg)
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

func genesisStateWithValSet(
	app *app.TerraApp, genesisState app.GenesisState,
	valSet *tmtypes.ValidatorSet, genAccs []authtypes.GenesisAccount,
	balances ...banktypes.Balance,
) app.GenesisState {
	// set genesis accounts
	authGenesis := authtypes.NewGenesisState(authtypes.DefaultParams(), genAccs)
	genesisState[authtypes.ModuleName] = app.AppCodec().MustMarshalJSON(authGenesis)

	validators := make([]stakingtypes.Validator, 0, len(valSet.Validators))
	delegations := make([]stakingtypes.Delegation, 0, len(valSet.Validators))
	validatorSigningInfos := make([]slashingtypes.SigningInfo, 0, len(valSet.Validators))
	missedBlocks := make([]slashingtypes.ValidatorMissedBlocks, 0, len(valSet.Validators))

	bondAmt := sdk.DefaultPowerReduction

	for _, val := range valSet.Validators {
		pk, err := cryptocodec.FromTmPubKeyInterface(val.PubKey)
		requireNoErr(err)
		pkAny, err := codectypes.NewAnyWithValue(pk)
		requireNoErr(err)
		validator := stakingtypes.Validator{
			OperatorAddress:   sdk.ValAddress(val.Address).String(),
			ConsensusPubkey:   pkAny,
			Jailed:            false,
			Status:            stakingtypes.Bonded,
			Tokens:            bondAmt,
			DelegatorShares:   sdk.OneDec(),
			Description:       stakingtypes.Description{},
			UnbondingHeight:   int64(0),
			UnbondingTime:     time.Unix(0, 0).UTC(),
			Commission:        stakingtypes.NewCommission(sdk.ZeroDec(), sdk.ZeroDec(), sdk.ZeroDec()),
			MinSelfDelegation: sdk.ZeroInt(),
		}

		slashingInfo := slashingtypes.SigningInfo{
			Address: sdk.GetConsAddress(pk).String(),
			ValidatorSigningInfo: slashingtypes.ValidatorSigningInfo{
				Address:             sdk.GetConsAddress(pk).String(),
				StartHeight:         fork.VersionMapEnableHeight,
				IndexOffset:         0,
				JailedUntil:         time.Unix(0, 0),
				Tombstoned:          false,
				MissedBlocksCounter: 0,
			},
		}

		validatorMissedBlock := slashingtypes.ValidatorMissedBlocks{
			Address:      sdk.GetConsAddress(pk).String(),
			MissedBlocks: []slashingtypes.MissedBlock{},
		}

		validators = append(validators, validator)
		delegations = append(delegations, stakingtypes.NewDelegation(genAccs[0].GetAddress(), val.Address.Bytes(), sdk.OneDec()))
		validatorSigningInfos = append(validatorSigningInfos, slashingInfo)
		missedBlocks = append(missedBlocks, validatorMissedBlock)
	}
	// set validators and delegations
	defaultStParams := stakingtypes.DefaultParams()
	stParams := stakingtypes.NewParams(
		defaultStParams.UnbondingTime,
		defaultStParams.MaxValidators,
		defaultStParams.MaxEntries,
		defaultStParams.HistoricalEntries,
		appparams.BondDenom,
		defaultStParams.MinCommissionRate,
	)

	// set validators and delegations
	stakingGenesis := stakingtypes.NewGenesisState(stParams, validators, delegations)
	genesisState[stakingtypes.ModuleName] = app.AppCodec().MustMarshalJSON(stakingGenesis)

	totalSupply := sdk.NewCoins()
	for _, b := range balances {
		// add genesis acc tokens to total supply
		totalSupply = totalSupply.Add(b.Coins...)
	}

	for range delegations {
		// add delegated tokens to total supply
		totalSupply = totalSupply.Add(sdk.NewCoin(appparams.BondDenom, bondAmt))
	}

	// add bonded amount to bonded pool module account
	balances = append(balances, banktypes.Balance{
		Address: authtypes.NewModuleAddress(stakingtypes.BondedPoolName).String(),
		Coins:   sdk.Coins{sdk.NewCoin(appparams.BondDenom, bondAmt)},
	})

	// update total supply
	bankGenesis := banktypes.NewGenesisState(
		banktypes.DefaultGenesisState().Params,
		balances,
		totalSupply,
		[]banktypes.Metadata{},
	)

	genesisState[banktypes.ModuleName] = app.AppCodec().MustMarshalJSON(bankGenesis)

	// set slashing info of validator
	slashingGenesis := slashingtypes.NewGenesisState(
		slashingtypes.DefaultParams(),
		validatorSigningInfos,
		missedBlocks,
	)

	genesisState[slashingtypes.ModuleName] = app.AppCodec().MustMarshalJSON(slashingGenesis)

	return genesisState
}
