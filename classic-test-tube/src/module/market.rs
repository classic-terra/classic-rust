use classic_rust::types::terra::market::v1beta1::*;
use test_tube::{fn_execute, fn_query};

use test_tube::module::Module;
use test_tube::runner::Runner;

pub struct Market<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Market<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Market<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub swap: MsgSwap[MsgSwap::TYPE_URL] => MsgSwapResponse
    }

    fn_execute! {
        pub swap_send: MsgSwapSend[MsgSwapSend::TYPE_URL] => MsgSwapSendResponse
    }

    fn_query! {
        pub query_swap ["/terra.market.v1beta1.Query/Swap"]: QuerySwapRequest => QuerySwapResponse
    }

    fn_query! {
        pub query_pool_delta ["/terra.market.v1beta1.Query/TerraPoolDelta"]: QueryTerraPoolDeltaRequest => QueryTerraPoolDeltaResponse
    }
}
