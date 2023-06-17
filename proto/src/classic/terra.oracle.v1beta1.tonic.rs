// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn exchange_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExchangeRateRequest>,
        ) -> Result<tonic::Response<super::QueryExchangeRateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/ExchangeRate");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exchange_rates(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExchangeRatesRequest>,
        ) -> Result<tonic::Response<super::QueryExchangeRatesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/ExchangeRates");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn tobin_tax(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTobinTaxRequest>,
        ) -> Result<tonic::Response<super::QueryTobinTaxResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/TobinTax");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn tobin_taxes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTobinTaxesRequest>,
        ) -> Result<tonic::Response<super::QueryTobinTaxesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/TobinTaxes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn actives(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryActivesRequest>,
        ) -> Result<tonic::Response<super::QueryActivesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/Actives");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn vote_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVoteTargetsRequest>,
        ) -> Result<tonic::Response<super::QueryVoteTargetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/VoteTargets");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn feeder_delegation(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFeederDelegationRequest>,
        ) -> Result<tonic::Response<super::QueryFeederDelegationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/terra.oracle.v1beta1.Query/FeederDelegation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn miss_counter(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMissCounterRequest>,
        ) -> Result<tonic::Response<super::QueryMissCounterResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/MissCounter");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn aggregate_prevote(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregatePrevoteRequest>,
        ) -> Result<tonic::Response<super::QueryAggregatePrevoteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/terra.oracle.v1beta1.Query/AggregatePrevote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn aggregate_prevotes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregatePrevotesRequest>,
        ) -> Result<tonic::Response<super::QueryAggregatePrevotesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/terra.oracle.v1beta1.Query/AggregatePrevotes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn aggregate_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateVoteRequest>,
        ) -> Result<tonic::Response<super::QueryAggregateVoteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/AggregateVote");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn aggregate_votes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateVotesRequest>,
        ) -> Result<tonic::Response<super::QueryAggregateVotesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/AggregateVotes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn aggregate_exchange_rate_prevote(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAggregateExchangeRatePrevote>,
        ) -> Result<tonic::Response<super::MsgAggregateExchangeRatePrevoteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/terra.oracle.v1beta1.Msg/AggregateExchangeRatePrevote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn aggregate_exchange_rate_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAggregateExchangeRateVote>,
        ) -> Result<tonic::Response<super::MsgAggregateExchangeRateVoteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/terra.oracle.v1beta1.Msg/AggregateExchangeRateVote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delegate_feed_consent(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDelegateFeedConsent>,
        ) -> Result<tonic::Response<super::MsgDelegateFeedConsentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/terra.oracle.v1beta1.Msg/DelegateFeedConsent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
