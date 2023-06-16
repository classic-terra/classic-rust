/// proposal request structure for adding burn tax exemption address(es)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddBurnTaxExemptionAddressProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// proposal request structure for removing burn tax exemption address(es)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBurnTaxExemptionAddressProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the parameters for the oracle module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub tax_policy: ::core::option::Option<PolicyConstraints>,
    #[prost(message, optional, tag = "2")]
    pub reward_policy: ::core::option::Option<PolicyConstraints>,
    #[prost(string, tag = "3")]
    pub seigniorage_burden_target: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub mining_increment: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub window_short: u64,
    #[prost(uint64, tag = "6")]
    pub window_long: u64,
    #[prost(uint64, tag = "7")]
    pub window_probation: u64,
    #[prost(string, tag = "8")]
    pub burn_tax_split: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub min_initial_deposit_ratio: ::prost::alloc::string::String,
}
/// PolicyConstraints - defines policy constraints can be applied in tax & reward policies
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyConstraints {
    #[prost(string, tag = "1")]
    pub rate_min: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub rate_max: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub cap: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub change_rate_max: ::prost::alloc::string::String,
}
/// EpochTaxProceeds represents the tax amount
/// collected at the current epoch
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochTaxProceeds {
    #[prost(message, repeated, tag = "1")]
    pub tax_proceeds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// EpochInitialIssuance represents initial issuance
/// of the currrent epoch
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochInitialIssuance {
    #[prost(message, repeated, tag = "1")]
    pub issuance: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryTaxRateRequest is the request type for the Query/TaxRate RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxRateRequest {}
/// QueryTaxRateResponse is response type for the
/// Query/TaxRate RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxRateResponse {
    #[prost(string, tag = "1")]
    pub tax_rate: ::prost::alloc::string::String,
}
/// QueryTaxCapRequest is the request type for the Query/TaxCap RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxCapRequest {
    /// denom defines the denomination to query for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryTaxCapResponse is response type for the
/// Query/TaxCap RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxCapResponse {
    #[prost(string, tag = "1")]
    pub tax_cap: ::prost::alloc::string::String,
}
/// QueryTaxCapsRequest is the request type for the Query/TaxCaps RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxCapsRequest {}
/// QueryTaxCapsResponseItem is response item type for the
/// Query/TaxCaps RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxCapsResponseItem {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tax_cap: ::prost::alloc::string::String,
}
/// QueryTaxCapsResponse is response type for the
/// Query/TaxCaps RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxCapsResponse {
    #[prost(message, repeated, tag = "1")]
    pub tax_caps: ::prost::alloc::vec::Vec<QueryTaxCapsResponseItem>,
}
/// QueryRewardWeightRequest is the request type for the Query/RewardWeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardWeightRequest {}
/// QueryRewardWeightResponse is response type for the
/// Query/RewardWeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardWeightResponse {
    #[prost(string, tag = "1")]
    pub reward_weight: ::prost::alloc::string::String,
}
/// QueryTaxProceedsRequest is the request type for the Query/TaxProceeds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxProceedsRequest {}
/// QueryTaxProceedsResponse is response type for the
/// Query/TaxProceeds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxProceedsResponse {
    #[prost(message, repeated, tag = "1")]
    pub tax_proceeds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QuerySeigniorageProceedsRequest is the request type for the Query/SeigniorageProceeds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySeigniorageProceedsRequest {}
/// QuerySeigniorageProceedsResponse is response type for the
/// Query/SeigniorageProceeds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySeigniorageProceedsResponse {
    #[prost(string, tag = "1")]
    pub seigniorage_proceeds: ::prost::alloc::string::String,
}
/// QueryIndicatorsRequest is the request type for the Query/Indicators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIndicatorsRequest {}
/// QueryIndicatorsResponse is response type for the
/// Query/Indicators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIndicatorsResponse {
    #[prost(string, tag = "1")]
    pub trl_year: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub trl_month: ::prost::alloc::string::String,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryBurnTaxExemptionListRequest is the request type for the Query/BurnTaxExemptionList RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBurnTaxExemptionListRequest {
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryBurnTaxExemptionListResponse is response type for the Query/BurnTaxExemptionList RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBurnTaxExemptionListResponse {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service.
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
            D: TryInto<tonic::transport::Endpoint>,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// TaxRate return the current tax rate
        pub async fn tax_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTaxRateRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTaxRateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.treasury.v1beta1.Query/TaxRate");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("terra.treasury.v1beta1.Query", "TaxRate"));
            self.inner.unary(req, path, codec).await
        }
        /// TaxCap returns the tax cap of a denom
        pub async fn tax_cap(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTaxCapRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTaxCapResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/terra.treasury.v1beta1.Query/TaxCap");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("terra.treasury.v1beta1.Query", "TaxCap"));
            self.inner.unary(req, path, codec).await
        }
        /// TaxCaps returns the all tax caps
        pub async fn tax_caps(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTaxCapsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTaxCapsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.treasury.v1beta1.Query/TaxCaps");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("terra.treasury.v1beta1.Query", "TaxCaps"));
            self.inner.unary(req, path, codec).await
        }
        /// RewardWeight return the current reward weight
        pub async fn reward_weight(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRewardWeightRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryRewardWeightResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.treasury.v1beta1.Query/RewardWeight");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.treasury.v1beta1.Query",
                "RewardWeight",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// SeigniorageProceeds return the current seigniorage proceeds
        pub async fn seigniorage_proceeds(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySeigniorageProceedsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySeigniorageProceedsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/terra.treasury.v1beta1.Query/SeigniorageProceeds",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.treasury.v1beta1.Query",
                "SeigniorageProceeds",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// TaxProceeds return the current tax proceeds
        pub async fn tax_proceeds(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTaxProceedsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTaxProceedsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.treasury.v1beta1.Query/TaxProceeds");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.treasury.v1beta1.Query",
                "TaxProceeds",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Indicators return the current trl informations
        pub async fn indicators(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryIndicatorsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryIndicatorsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.treasury.v1beta1.Query/Indicators");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.treasury.v1beta1.Query",
                "Indicators",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// BurnTaxExemptionList returns all registered burn tax exemption addresses
        pub async fn burn_tax_exemption_list(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBurnTaxExemptionListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBurnTaxExemptionListResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/terra.treasury.v1beta1.Query/BurnTaxExemptionList",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.treasury.v1beta1.Query",
                "BurnTaxExemptionList",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Params queries all parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/terra.treasury.v1beta1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("terra.treasury.v1beta1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// TaxRate return the current tax rate
        async fn tax_rate(
            &self,
            request: tonic::Request<super::QueryTaxRateRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTaxRateResponse>, tonic::Status>;
        /// TaxCap returns the tax cap of a denom
        async fn tax_cap(
            &self,
            request: tonic::Request<super::QueryTaxCapRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTaxCapResponse>, tonic::Status>;
        /// TaxCaps returns the all tax caps
        async fn tax_caps(
            &self,
            request: tonic::Request<super::QueryTaxCapsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTaxCapsResponse>, tonic::Status>;
        /// RewardWeight return the current reward weight
        async fn reward_weight(
            &self,
            request: tonic::Request<super::QueryRewardWeightRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryRewardWeightResponse>, tonic::Status>;
        /// SeigniorageProceeds return the current seigniorage proceeds
        async fn seigniorage_proceeds(
            &self,
            request: tonic::Request<super::QuerySeigniorageProceedsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySeigniorageProceedsResponse>,
            tonic::Status,
        >;
        /// TaxProceeds return the current tax proceeds
        async fn tax_proceeds(
            &self,
            request: tonic::Request<super::QueryTaxProceedsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTaxProceedsResponse>, tonic::Status>;
        /// Indicators return the current trl informations
        async fn indicators(
            &self,
            request: tonic::Request<super::QueryIndicatorsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryIndicatorsResponse>, tonic::Status>;
        /// BurnTaxExemptionList returns all registered burn tax exemption addresses
        async fn burn_tax_exemption_list(
            &self,
            request: tonic::Request<super::QueryBurnTaxExemptionListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBurnTaxExemptionListResponse>,
            tonic::Status,
        >;
        /// Params queries all parameters.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
    }
    /// Query defines the gRPC querier service.
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/terra.treasury.v1beta1.Query/TaxRate" => {
                    #[allow(non_camel_case_types)]
                    struct TaxRateSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTaxRateRequest> for TaxRateSvc<T> {
                        type Response = super::QueryTaxRateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTaxRateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).tax_rate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TaxRateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/terra.treasury.v1beta1.Query/TaxCap" => {
                    #[allow(non_camel_case_types)]
                    struct TaxCapSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTaxCapRequest> for TaxCapSvc<T> {
                        type Response = super::QueryTaxCapResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTaxCapRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).tax_cap(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TaxCapSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/terra.treasury.v1beta1.Query/TaxCaps" => {
                    #[allow(non_camel_case_types)]
                    struct TaxCapsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTaxCapsRequest> for TaxCapsSvc<T> {
                        type Response = super::QueryTaxCapsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTaxCapsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).tax_caps(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TaxCapsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/terra.treasury.v1beta1.Query/RewardWeight" => {
                    #[allow(non_camel_case_types)]
                    struct RewardWeightSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryRewardWeightRequest> for RewardWeightSvc<T> {
                        type Response = super::QueryRewardWeightResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRewardWeightRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).reward_weight(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RewardWeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/terra.treasury.v1beta1.Query/SeigniorageProceeds" => {
                    #[allow(non_camel_case_types)]
                    struct SeigniorageProceedsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QuerySeigniorageProceedsRequest>
                        for SeigniorageProceedsSvc<T>
                    {
                        type Response = super::QuerySeigniorageProceedsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySeigniorageProceedsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).seigniorage_proceeds(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SeigniorageProceedsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/terra.treasury.v1beta1.Query/TaxProceeds" => {
                    #[allow(non_camel_case_types)]
                    struct TaxProceedsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTaxProceedsRequest> for TaxProceedsSvc<T> {
                        type Response = super::QueryTaxProceedsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTaxProceedsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).tax_proceeds(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TaxProceedsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/terra.treasury.v1beta1.Query/Indicators" => {
                    #[allow(non_camel_case_types)]
                    struct IndicatorsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryIndicatorsRequest> for IndicatorsSvc<T> {
                        type Response = super::QueryIndicatorsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryIndicatorsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).indicators(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IndicatorsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/terra.treasury.v1beta1.Query/BurnTaxExemptionList" => {
                    #[allow(non_camel_case_types)]
                    struct BurnTaxExemptionListSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryBurnTaxExemptionListRequest>
                        for BurnTaxExemptionListSvc<T>
                    {
                        type Response = super::QueryBurnTaxExemptionListResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryBurnTaxExemptionListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).burn_tax_exemption_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BurnTaxExemptionListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/terra.treasury.v1beta1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "terra.treasury.v1beta1.Query";
    }
}
/// GenesisState defines the oracle module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, tag = "2")]
    pub tax_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub reward_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub tax_caps: ::prost::alloc::vec::Vec<TaxCap>,
    #[prost(message, repeated, tag = "5")]
    pub tax_proceeds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "6")]
    pub epoch_initial_issuance:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "7")]
    pub epoch_states: ::prost::alloc::vec::Vec<EpochState>,
}
/// TaxCap is the max tax amount can be charged for the given denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxCap {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tax_cap: ::prost::alloc::string::String,
}
/// EpochState is the record for each epoch state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochState {
    #[prost(uint64, tag = "1")]
    pub epoch: u64,
    #[prost(string, tag = "2")]
    pub tax_reward: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub seigniorage_reward: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub total_staked_luna: ::prost::alloc::string::String,
}
