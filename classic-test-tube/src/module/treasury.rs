use classic_rust::types::terra::treasury::v1beta1::*;
use classic_core_test_tube::fn_query;

use classic_core_test_tube::module::Module;
use classic_core_test_tube::runner::Runner;

pub struct Treasury<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Treasury<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Treasury<'a, R>
where
    R: Runner<'a>,
{
    fn_query! {
        pub query_tax_cap ["/terra.treasury.v1beta1.Query/TaxCap"]: QueryTaxCapRequest => QueryTaxCapResponse
    }

    fn_query! {
        pub query_tax_rate ["/terra.treasury.v1beta1.Query/TaxRate"]: QueryTaxRateRequest => QueryTaxRateResponse
    }
}
