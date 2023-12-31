use classic_rust::types::terra::oracle::v1beta1::*;
use classic_core_test_tube::fn_query;

use classic_core_test_tube::module::Module;
use classic_core_test_tube::runner::Runner;

pub struct Oracle<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Oracle<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Oracle<'a, R>
where
    R: Runner<'a>,
{
    fn_query! {
        pub query_exchange_rates ["/terra.oracle.v1beta1.Query/ExchangeRates"]: QueryExchangeRateRequest => QueryExchangeRateResponse
    }
}
