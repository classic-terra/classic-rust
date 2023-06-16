//! Registry of type URLs associated with various protobuf types defined in
//! this crate.

// TODO(tarcieri): leverage first-class support for type URLs in prost?
// See: https://github.com/tokio-rs/prost/issues/299

use crate::{classic, traits::TypeUrl};

impl TypeUrl for classic::market::MsgSwap {
    const TYPE_URL: &'static str = "/terra.market.v1beta1.MsgSwap";
}

impl TypeUrl for classic::market::MsgSwapResponse {
    const TYPE_URL: &'static str = "/terra.market.v1beta1.MsgSwapResponse";
}

impl TypeUrl for classic::market::MsgSwapSend {
    const TYPE_URL: &'static str = "/terra.market.v1beta1.MsgSwapSend";
}

impl TypeUrl for classic::market::MsgSwapSendResponse {
    const TYPE_URL: &'static str = "/terra.market.v1beta1.MsgSwapSendResponse";
}

impl TypeUrl for classic::oracle::MsgAggregateExchangeRatePrevote {
    const TYPE_URL: &'static str = "/terra.oracle.v1beta1.MsgAggregateExchangeRatePrevote";
}

impl TypeUrl for classic::oracle::MsgAggregateExchangeRatePrevoteResponse {
    const TYPE_URL: &'static str = "/terra.oracle.v1beta1.MsgAggregateExchangeRatePrevoteResponse";
}

impl TypeUrl for classic::oracle::MsgAggregateExchangeRateVote {
    const TYPE_URL: &'static str = "/terra.oracle.v1beta1.MsgAggregateExchangeRateVote";
}

impl TypeUrl for classic::oracle::MsgAggregateExchangeRateVoteResponse {
    const TYPE_URL: &'static str = "/terra.oracle.v1beta1.MsgAggregateExchangeRateVoteResponse";
}

impl TypeUrl for classic::oracle::MsgDelegateFeedConsent {
    const TYPE_URL: &'static str = "/terra.oracle.v1beta1.MsgDelegateFeedConsent";
}

impl TypeUrl for classic::oracle::MsgDelegateFeedConsentResponse {
    const TYPE_URL: &'static str = "/terra.oracle.v1beta1.MsgDelegateFeedConsentResponse";
}
