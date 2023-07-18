use cosmrs::Any;

use cosmwasm_std::Coin;

use prost::Message;
use test_tube::account::SigningAccount;

use test_tube::runner::result::{RunnerExecuteResult, RunnerResult};
use test_tube::runner::Runner;
use test_tube::BaseApp;

const FEE_DENOM: &str = "uluna";
const TERRA_ADDRESS_PREFIX: &str = "terra";
const CHAIN_ID: &str = "columbus-5";
const DEFAULT_GAS_ADJUSTMENT: f64 = 1.2;

#[derive(Debug, PartialEq)]
pub struct TerraTestApp {
    inner: BaseApp,
}

impl Default for TerraTestApp {
    fn default() -> Self {
        TerraTestApp::new()
    }
}

impl TerraTestApp {
    pub fn new() -> Self {
        Self {
            inner: BaseApp::new(
                FEE_DENOM,
                CHAIN_ID,
                TERRA_ADDRESS_PREFIX,
                DEFAULT_GAS_ADJUSTMENT,
            ),
        }
    }

    /// Get the current block time in nanoseconds
    pub fn get_block_time_nanos(&self) -> i64 {
        self.inner.get_block_time_nanos()
    }

    /// Get the current block time in seconds
    pub fn get_block_time_seconds(&self) -> i64 {
        self.inner.get_block_time_nanos() / 1_000_000_000i64
    }

    /// Get the current block height
    pub fn get_block_height(&self) -> i64 {
        self.inner.get_block_height()
    }

    /// Get the first validator address
    pub fn get_first_validator_address(&self) -> RunnerResult<String> {
        self.inner.get_first_validator_address()
    }

    /// Get the first validator signing account
    pub fn get_first_validator_signing_account(&self) -> RunnerResult<SigningAccount> {
        self.inner.get_first_validator_signing_account()
    }

    /// Increase the time of the blockchain by the given number of seconds.
    pub fn increase_time(&self, seconds: u64) {
        self.inner.increase_time(seconds)
    }

    /// Initialize account with initial balance of any coins.
    /// This function mints new coins and send to newly created account
    pub fn init_account(&self, coins: &[Coin]) -> RunnerResult<SigningAccount> {
        self.inner.init_account(coins)
    }
    /// Convinience function to create multiple accounts with the same
    /// Initial coins balance
    pub fn init_accounts(&self, coins: &[Coin], count: u64) -> RunnerResult<Vec<SigningAccount>> {
        self.inner.init_accounts(coins, count)
    }

    /// Simulate transaction execution and return gas info
    pub fn simulate_tx<I>(
        &self,
        msgs: I,
        signer: &SigningAccount,
    ) -> RunnerResult<cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        self.inner.simulate_tx(msgs, signer)
    }

    /// Set parameter set for a given subspace.
    pub fn set_param_set(&self, subspace: &str, pset: impl Into<Any>) -> RunnerResult<()> {
        self.inner.set_param_set(subspace, pset)
    }

    /// Get parameter set for a given subspace.
    pub fn get_param_set<P: Message + Default>(
        &self,
        subspace: &str,
        type_url: &str,
    ) -> RunnerResult<P> {
        self.inner.get_param_set(subspace, type_url)
    }
}

impl<'a> Runner<'a> for TerraTestApp {
    fn execute_multiple<M, R>(
        &self,
        msgs: &[(M, &str)],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default,
    {
        self.inner.execute_multiple(msgs, signer)
    }

    fn query<Q, R>(&self, path: &str, q: &Q) -> RunnerResult<R>
    where
        Q: ::prost::Message,
        R: ::prost::Message + Default,
    {
        self.inner.query(path, q)
    }

    fn execute_multiple_raw<R>(
        &self,
        msgs: Vec<cosmrs::Any>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        R: prost::Message + Default,
    {
        self.inner.execute_multiple_raw(msgs, signer)
    }
}

#[cfg(test)]
mod tests {
    use classic_rust::types::terra::treasury::v1beta1::QueryTaxCapRequest;
    use prost::Message;
    use std::option::Option::None;

    use cosmrs::proto::cosmos::bank::v1beta1::QueryAllBalancesRequest;
    use cosmwasm_std::{coins, Coin};

    use crate::module::Wasm;
    use crate::runner::app::TerraTestApp;
    use crate::{Bank, Treasury};
    use test_tube::account::{Account, FeeSetting};
    use test_tube::module::Module;
    use test_tube::runner::*;

    #[test]
    fn test_init_accounts() {
        let app = TerraTestApp::default();
        let accounts = app
            .init_accounts(&coins(100_000_000_000, "uosmo"), 3)
            .unwrap();

        assert!(accounts.get(0).is_some());
        assert!(accounts.get(1).is_some());
        assert!(accounts.get(2).is_some());
        assert!(accounts.get(3).is_none());
    }

    #[test]
    fn test_get_and_set_block_timestamp() {
        let app = TerraTestApp::default();

        let block_time_nanos = app.get_block_time_nanos();
        let block_time_seconds = app.get_block_time_seconds();

        app.increase_time(10u64);

        assert_eq!(
            app.get_block_time_nanos(),
            block_time_nanos + 10_000_000_000
        );
        assert_eq!(app.get_block_time_seconds(), block_time_seconds + 10);
    }

    #[test]
    fn test_get_block_height() {
        let app = TerraTestApp::default();

        assert_eq!(app.get_block_height(), 11543150i64);

        app.increase_time(10u64);

        assert_eq!(app.get_block_height(), 11543151i64);
    }

    #[test]
    fn test_execute_swap() {
        use classic_rust::types::terra::market::v1beta1::MsgSwap;
        use classic_rust::types::cosmos::base::v1beta1::Coin;
        use crate::module::Bank;
        use crate::module::Market;

        let app = TerraTestApp::default();
        let bank = Bank::new(&app);
        let market = Market::new(&app);

        let acc = app
            .init_account(&coins(100_000_000_000_000, "uluna"))
            .unwrap();

        // the price of 1 uluna = 1 uusd, so trader is expected to receive 1000uusd 
        let msg = MsgSwap {
            trader: acc.address(),
            ask_denom: String::from("uusd"),
            offer_coin: Option::Some(Coin{amount: String::from("1000000"), denom: String::from("uluna")})
        };

        let res = market.swap(msg, &acc).unwrap();

        let swap_coin = &res
            .data
            .swap_coin
            .unwrap();

        let swap_fee = &res
            .data
            .swap_fee
            .unwrap();

        assert_eq!(swap_coin.denom.as_str(), "uusd");
        assert_eq!(swap_fee.denom.as_str(), "uusd");

        let swap_amount : u64 = swap_coin.amount.parse().unwrap();
        let fee_amount : u64 = swap_fee.amount.parse().unwrap();
        let total_amount : u64 = swap_amount + fee_amount;

        assert_eq!(
            total_amount,
            1000000
        );

        let query = &QueryAllBalancesRequest{
            address: acc.address(),
            pagination: None
        };

        // check that the account has updated swap coin amount
        let res = bank.query_all_balances(query).unwrap();
        res.balances.iter().filter(|coin| coin.denom == "uusd").for_each(|coin| {
            assert_eq!(coin.amount, swap_coin.amount);
        });

        // execute on more time to excercise account sequence
        let msg = MsgSwap {
            trader: acc.address(),
            ask_denom: String::from("uusd"),
            offer_coin: Option::Some(Coin{amount: String::from("1000000"), denom: String::from("uluna")})
        };

        let res = market.swap(msg, &acc).unwrap();
        
        let additional_swap_coin = &res
            .data
            .swap_coin
            .unwrap();
        
        let accumlated_uusd = swap_amount + additional_swap_coin.amount.parse::<u64>().unwrap();
        
        // check that the account has updated swap coin amount
        let res = bank.query_all_balances(query).unwrap();
        res.balances.iter().filter(|coin| coin.denom == "uusd").for_each(|coin| {
            assert_eq!(coin.amount.parse::<u64>().unwrap(), accumlated_uusd);
        });
        
    }

    #[test]
    fn test_query_tax_cap() {
        let app = TerraTestApp::default();
        let treasury = Treasury::new(&app);

        let tax_cap = treasury.query_tax_cap(&QueryTaxCapRequest{
            denom: String::from("uluna")
        }).unwrap();

        assert_eq!(tax_cap.tax_cap, String::from("60000000000000000"));
    }

    #[test]
    fn test_query_print_all_module_addresses() {
        use cosmrs::proto::cosmos::auth::v1beta1::{QueryAccountsRequest, QueryAccountsResponse, ModuleAccount};

        let app = TerraTestApp::default();

        let query = &QueryAccountsRequest {
            pagination: None,
        };

        let modules: Vec<ModuleAccount> = app
            .query::<QueryAccountsRequest, QueryAccountsResponse>("/cosmos.auth.v1beta1.Query/Accounts", query)
            .unwrap()
            .accounts
            .into_iter()
            .map(|account| {
                let value_slice = account.value.as_slice();
                ModuleAccount::decode::<&[u8]>(value_slice)
            })
            .filter_map(Result::ok)
            .collect();
        
        modules.iter()
            .for_each(|module| {
                println!("name: {}, address: {}", module.clone().name, module.clone().base_account.unwrap().address)
            });
        
        assert_eq!(modules.len(), 10);
    }

    #[test]
    fn test_wasm_execute_and_query() {
        use cw1_whitelist::msg::*;

        let app = TerraTestApp::default();
        let accs = app
            .init_accounts(
                &[
                    Coin::new(1_000_000_000_000, "uluna"),
                    Coin::new(1_000_000_000_000, "uusd"),
                ],
                2,
            )
            .unwrap();
        let admin = &accs[0];
        let new_admin = &accs[1];

        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, admin)
            .unwrap()
            .data
            .code_id;
        assert_eq!(code_id, 1);

        // initialize admins and check if the state is correct
        let init_admins = vec![admin.address()];
        let contract_addr = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    admins: init_admins.clone(),
                    mutable: true,
                },
                Some(&admin.address()),
                None,
                &[],
                admin,
            )
            .unwrap()
            .data
            .address;
        let admin_list = wasm
            .query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {})
            .unwrap();
        assert_eq!(admin_list.admins, init_admins);
        assert!(admin_list.mutable);

        // update admin and check again
        let new_admins = vec![new_admin.address()];
        wasm.execute::<ExecuteMsg>(
            &contract_addr,
            &ExecuteMsg::UpdateAdmins {
                admins: new_admins.clone(),
            },
            &[],
            admin,
        )
        .unwrap();

        let admin_list = wasm
            .query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {})
            .unwrap();

        assert_eq!(admin_list.admins, new_admins);
        assert!(admin_list.mutable);
    }

    #[test]
    fn test_custom_fee() {
        let app = TerraTestApp::default();
        let initial_balance = 1_000_000_000_000;
        let alice = app.init_account(&coins(initial_balance, "uluna")).unwrap();
        let bob = app.init_account(&coins(initial_balance, "uluna")).unwrap();

        let amount = Coin::new(1_000_000, "uluna");
        let gas_limit = 100_000_000;

        // use FeeSetting::Auto by default, so should not equal newly custom fee setting
        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let res = wasm.store_code(&wasm_byte_code, None, &alice).unwrap();

        assert_ne!(res.gas_info.gas_wanted, gas_limit);

        //update fee setting
        let bob = bob.with_fee_setting(FeeSetting::Custom {
            amount: amount.clone(),
            gas_limit,
        });
        let res = wasm.store_code(&wasm_byte_code, None, &bob).unwrap();

        let bob_balance = Bank::new(&app)
            .query_all_balances(&QueryAllBalancesRequest {
                address: bob.address(),
                pagination: None,
            })
            .unwrap()
            .balances
            .into_iter()
            .find(|c| c.denom == "uluna")
            .unwrap()
            .amount
            .parse::<u128>()
            .unwrap();

        assert_eq!(res.gas_info.gas_wanted, gas_limit);
        assert_eq!(bob_balance, initial_balance - amount.amount.u128());
    }

    #[test]
    fn test_param_set() {
        use cosmrs::Any;
        use classic_rust::types::terra::treasury::v1beta1::*;

        let app = TerraTestApp::new();

        let treasury_params = app.get_param_set::<Params>(classic_rust::TREASURY_MODULE, Params::TYPE_URL).unwrap();

        // sdk.Dec is being handled weirdly, this is due to sdk.Dec being handled by amino in sdk instead of proto in rust.
        // 0000000000000000000 (19 zeros) () (0.000000000000000000 in sdk.Dec)
        // 1000000000000000000 (1.0 in sdk.Dec)
        // 2357 (0.000000000000002357 in sdk.Dec)
        let in_pset = Params {
            tax_policy: treasury_params.tax_policy,
            reward_policy : treasury_params.reward_policy,
            seigniorage_burden_target: treasury_params.seigniorage_burden_target,
            mining_increment: treasury_params.mining_increment,
            window_short: treasury_params.window_short,
            window_long: treasury_params.window_long,
            window_probation: treasury_params.window_probation,
            burn_tax_split: String::from("50000000000000000"), //0.05
            min_initial_deposit_ratio: treasury_params.min_initial_deposit_ratio,
        };
        app.set_param_set(
            classic_rust::TREASURY_MODULE,
            Any {
                type_url: Params::TYPE_URL.to_string(),
                value: in_pset.encode_to_vec(),
            },
        ).unwrap();

        let out_pset = app.get_param_set::<Params>(classic_rust::TREASURY_MODULE, Params::TYPE_URL).unwrap();
        assert_eq!(out_pset.burn_tax_split, String::from("50000000000000000"));
    }
}
