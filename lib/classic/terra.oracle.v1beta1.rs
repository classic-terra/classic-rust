/// MsgAggregateExchangeRatePrevote represents a message to submit
/// aggregate exchange rate prevote.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRatePrevote {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub feeder: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator: ::prost::alloc::string::String,
}
/// MsgAggregateExchangeRatePrevoteResponse defines the Msg/AggregateExchangeRatePrevote response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRatePrevoteResponse {}
/// MsgAggregateExchangeRateVote represents a message to submit
/// aggregate exchange rate vote.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRateVote {
    #[prost(string, tag = "1")]
    pub salt: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exchange_rates: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub feeder: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub validator: ::prost::alloc::string::String,
}
/// MsgAggregateExchangeRateVoteResponse defines the Msg/AggregateExchangeRateVote response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRateVoteResponse {}
/// MsgDelegateFeedConsent represents a message to
/// delegate oracle voting rights to another address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelegateFeedConsent {
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub delegate: ::prost::alloc::string::String,
}
/// MsgDelegateFeedConsentResponse defines the Msg/DelegateFeedConsent response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelegateFeedConsentResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the oracle Msg service.
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
            D: TryInto<tonic::transport::Endpoint>,
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
        /// AggregateExchangeRatePrevote defines a method for submitting
        /// aggregate exchange rate prevote
        pub async fn aggregate_exchange_rate_prevote(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAggregateExchangeRatePrevote>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAggregateExchangeRatePrevoteResponse>,
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
                "/terra.oracle.v1beta1.Msg/AggregateExchangeRatePrevote",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.oracle.v1beta1.Msg",
                "AggregateExchangeRatePrevote",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// AggregateExchangeRateVote defines a method for submitting
        /// aggregate exchange rate vote
        pub async fn aggregate_exchange_rate_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAggregateExchangeRateVote>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAggregateExchangeRateVoteResponse>,
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
                "/terra.oracle.v1beta1.Msg/AggregateExchangeRateVote",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.oracle.v1beta1.Msg",
                "AggregateExchangeRateVote",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DelegateFeedConsent defines a method for setting the feeder delegation
        pub async fn delegate_feed_consent(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDelegateFeedConsent>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDelegateFeedConsentResponse>,
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
                "/terra.oracle.v1beta1.Msg/DelegateFeedConsent",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.oracle.v1beta1.Msg",
                "DelegateFeedConsent",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        /// AggregateExchangeRatePrevote defines a method for submitting
        /// aggregate exchange rate prevote
        async fn aggregate_exchange_rate_prevote(
            &self,
            request: tonic::Request<super::MsgAggregateExchangeRatePrevote>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAggregateExchangeRatePrevoteResponse>,
            tonic::Status,
        >;
        /// AggregateExchangeRateVote defines a method for submitting
        /// aggregate exchange rate vote
        async fn aggregate_exchange_rate_vote(
            &self,
            request: tonic::Request<super::MsgAggregateExchangeRateVote>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAggregateExchangeRateVoteResponse>,
            tonic::Status,
        >;
        /// DelegateFeedConsent defines a method for setting the feeder delegation
        async fn delegate_feed_consent(
            &self,
            request: tonic::Request<super::MsgDelegateFeedConsent>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDelegateFeedConsentResponse>,
            tonic::Status,
        >;
    }
    /// Msg defines the oracle Msg service.
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
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
                "/terra.oracle.v1beta1.Msg/AggregateExchangeRatePrevote" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateExchangeRatePrevoteSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAggregateExchangeRatePrevote>
                        for AggregateExchangeRatePrevoteSvc<T>
                    {
                        type Response = super::MsgAggregateExchangeRatePrevoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAggregateExchangeRatePrevote>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).aggregate_exchange_rate_prevote(request).await
                            };
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
                        let method = AggregateExchangeRatePrevoteSvc(inner);
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
                "/terra.oracle.v1beta1.Msg/AggregateExchangeRateVote" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateExchangeRateVoteSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAggregateExchangeRateVote>
                        for AggregateExchangeRateVoteSvc<T>
                    {
                        type Response = super::MsgAggregateExchangeRateVoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAggregateExchangeRateVote>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).aggregate_exchange_rate_vote(request).await };
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
                        let method = AggregateExchangeRateVoteSvc(inner);
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
                "/terra.oracle.v1beta1.Msg/DelegateFeedConsent" => {
                    #[allow(non_camel_case_types)]
                    struct DelegateFeedConsentSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDelegateFeedConsent>
                        for DelegateFeedConsentSvc<T>
                    {
                        type Response = super::MsgDelegateFeedConsentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDelegateFeedConsent>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delegate_feed_consent(request).await };
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
                        let method = DelegateFeedConsentSvc(inner);
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
    impl<T: Msg> Clone for MsgServer<T> {
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
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "terra.oracle.v1beta1.Msg";
    }
}
/// Params defines the parameters for the oracle module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub vote_period: u64,
    #[prost(string, tag = "2")]
    pub vote_threshold: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub reward_band: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub reward_distribution_window: u64,
    #[prost(message, repeated, tag = "5")]
    pub whitelist: ::prost::alloc::vec::Vec<Denom>,
    #[prost(string, tag = "6")]
    pub slash_fraction: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub slash_window: u64,
    #[prost(string, tag = "8")]
    pub min_valid_per_window: ::prost::alloc::string::String,
}
/// Denom - the object to hold configurations of each denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Denom {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tobin_tax: ::prost::alloc::string::String,
}
/// struct for aggregate prevoting on the ExchangeRateVote.
/// The purpose of aggregate prevote is to hide vote exchange rates with hash
/// which is formatted as hex string in SHA256("{salt}:{exchange rate}{denom},...,{exchange rate}{denom}:{voter}")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateExchangeRatePrevote {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub submit_block: u64,
}
/// MsgAggregateExchangeRateVote - struct for voting on
/// the exchange rates of Luna denominated in various Terra assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateExchangeRateVote {
    #[prost(message, repeated, tag = "1")]
    pub exchange_rate_tuples: ::prost::alloc::vec::Vec<ExchangeRateTuple>,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
}
/// ExchangeRateTuple - struct to store interpreted exchange rates data to store
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeRateTuple {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exchange_rate: ::prost::alloc::string::String,
}
/// QueryExchangeRateRequest is the request type for the Query/ExchangeRate RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeRateRequest {
    /// denom defines the denomination to query for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryExchangeRateResponse is response type for the
/// Query/ExchangeRate RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeRateResponse {
    /// exchange_rate defines the exchange rate of Luna denominated in various Terra
    #[prost(string, tag = "1")]
    pub exchange_rate: ::prost::alloc::string::String,
}
/// QueryExchangeRatesRequest is the request type for the Query/ExchangeRates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeRatesRequest {}
/// QueryExchangeRatesResponse is response type for the
/// Query/ExchangeRates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeRatesResponse {
    /// exchange_rates defines a list of the exchange rate for all whitelisted denoms.
    #[prost(message, repeated, tag = "1")]
    pub exchange_rates:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// QueryTobinTaxRequest is the request type for the Query/TobinTax RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTobinTaxRequest {
    /// denom defines the denomination to query for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryTobinTaxResponse is response type for the
/// Query/TobinTax RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTobinTaxResponse {
    /// tobin_taxe defines the tobin tax of a denom
    #[prost(string, tag = "1")]
    pub tobin_tax: ::prost::alloc::string::String,
}
/// QueryTobinTaxesRequest is the request type for the Query/TobinTaxes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTobinTaxesRequest {}
/// QueryTobinTaxesResponse is response type for the
/// Query/TobinTaxes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTobinTaxesResponse {
    /// tobin_taxes defines a list of the tobin tax of all whitelisted denoms
    #[prost(message, repeated, tag = "1")]
    pub tobin_taxes: ::prost::alloc::vec::Vec<Denom>,
}
/// QueryActivesRequest is the request type for the Query/Actives RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActivesRequest {}
/// QueryActivesResponse is response type for the
/// Query/Actives RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActivesResponse {
    /// actives defines a list of the denomination which oracle prices aggreed upon.
    #[prost(string, repeated, tag = "1")]
    pub actives: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryVoteTargetsRequest is the request type for the Query/VoteTargets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVoteTargetsRequest {}
/// QueryVoteTargetsResponse is response type for the
/// Query/VoteTargets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVoteTargetsResponse {
    /// vote_targets defines a list of the denomination in which everyone
    /// should vote in the current vote period.
    #[prost(string, repeated, tag = "1")]
    pub vote_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryFeederDelegationRequest is the request type for the Query/FeederDelegation RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeederDelegationRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryFeederDelegationResponse is response type for the
/// Query/FeederDelegation RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeederDelegationResponse {
    /// feeder_addr defines the feeder delegation of a validator
    #[prost(string, tag = "1")]
    pub feeder_addr: ::prost::alloc::string::String,
}
/// QueryMissCounterRequest is the request type for the Query/MissCounter RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMissCounterRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryMissCounterResponse is response type for the
/// Query/MissCounter RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMissCounterResponse {
    /// miss_counter defines the oracle miss counter of a validator
    #[prost(uint64, tag = "1")]
    pub miss_counter: u64,
}
/// QueryAggregatePrevoteRequest is the request type for the Query/AggregatePrevote RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevoteRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryAggregatePrevoteResponse is response type for the
/// Query/AggregatePrevote RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevoteResponse {
    /// aggregate_prevote defines oracle aggregate prevote submitted by a validator in the current vote period
    #[prost(message, optional, tag = "1")]
    pub aggregate_prevote: ::core::option::Option<AggregateExchangeRatePrevote>,
}
/// QueryAggregatePrevotesRequest is the request type for the Query/AggregatePrevotes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevotesRequest {}
/// QueryAggregatePrevotesResponse is response type for the
/// Query/AggregatePrevotes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevotesResponse {
    /// aggregate_prevotes defines all oracle aggregate prevotes submitted in the current vote period
    #[prost(message, repeated, tag = "1")]
    pub aggregate_prevotes: ::prost::alloc::vec::Vec<AggregateExchangeRatePrevote>,
}
/// QueryAggregateVoteRequest is the request type for the Query/AggregateVote RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVoteRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryAggregateVoteResponse is response type for the
/// Query/AggregateVote RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVoteResponse {
    /// aggregate_vote defines oracle aggregate vote submitted by a validator in the current vote period
    #[prost(message, optional, tag = "1")]
    pub aggregate_vote: ::core::option::Option<AggregateExchangeRateVote>,
}
/// QueryAggregateVotesRequest is the request type for the Query/AggregateVotes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVotesRequest {}
/// QueryAggregateVotesResponse is response type for the
/// Query/AggregateVotes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVotesResponse {
    /// aggregate_votes defines all oracle aggregate votes submitted in the current vote period
    #[prost(message, repeated, tag = "1")]
    pub aggregate_votes: ::prost::alloc::vec::Vec<AggregateExchangeRateVote>,
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
        /// ExchangeRate returns exchange rate of a denom
        pub async fn exchange_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExchangeRateRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryExchangeRateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/ExchangeRate");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.oracle.v1beta1.Query",
                "ExchangeRate",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ExchangeRates returns exchange rates of all denoms
        pub async fn exchange_rates(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExchangeRatesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryExchangeRatesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/ExchangeRates");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.oracle.v1beta1.Query",
                "ExchangeRates",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// TobinTax returns tobin tax of a denom
        pub async fn tobin_tax(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTobinTaxRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTobinTaxResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/TobinTax");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("terra.oracle.v1beta1.Query", "TobinTax"));
            self.inner.unary(req, path, codec).await
        }
        /// TobinTaxes returns tobin taxes of all denoms
        pub async fn tobin_taxes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTobinTaxesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTobinTaxesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/TobinTaxes");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("terra.oracle.v1beta1.Query", "TobinTaxes"));
            self.inner.unary(req, path, codec).await
        }
        /// Actives returns all active denoms
        pub async fn actives(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryActivesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryActivesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/Actives");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("terra.oracle.v1beta1.Query", "Actives"));
            self.inner.unary(req, path, codec).await
        }
        /// VoteTargets returns all vote target denoms
        pub async fn vote_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVoteTargetsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryVoteTargetsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/VoteTargets");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("terra.oracle.v1beta1.Query", "VoteTargets"));
            self.inner.unary(req, path, codec).await
        }
        /// FeederDelegation returns feeder delegation of a validator
        pub async fn feeder_delegation(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFeederDelegationRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryFeederDelegationResponse>, tonic::Status>
        {
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
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.oracle.v1beta1.Query",
                "FeederDelegation",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// MissCounter returns oracle miss counter of a validator
        pub async fn miss_counter(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMissCounterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryMissCounterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/MissCounter");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("terra.oracle.v1beta1.Query", "MissCounter"));
            self.inner.unary(req, path, codec).await
        }
        /// AggregatePrevote returns an aggregate prevote of a validator
        pub async fn aggregate_prevote(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregatePrevoteRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAggregatePrevoteResponse>, tonic::Status>
        {
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
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.oracle.v1beta1.Query",
                "AggregatePrevote",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// AggregatePrevotes returns aggregate prevotes of all validators
        pub async fn aggregate_prevotes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregatePrevotesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAggregatePrevotesResponse>,
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
                "/terra.oracle.v1beta1.Query/AggregatePrevotes",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.oracle.v1beta1.Query",
                "AggregatePrevotes",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// AggregateVote returns an aggregate vote of a validator
        pub async fn aggregate_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateVoteRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAggregateVoteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/AggregateVote");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.oracle.v1beta1.Query",
                "AggregateVote",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// AggregateVotes returns aggregate votes of all validators
        pub async fn aggregate_votes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateVotesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAggregateVotesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/AggregateVotes");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "terra.oracle.v1beta1.Query",
                "AggregateVotes",
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
            let path = http::uri::PathAndQuery::from_static("/terra.oracle.v1beta1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("terra.oracle.v1beta1.Query", "Params"));
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
        /// ExchangeRate returns exchange rate of a denom
        async fn exchange_rate(
            &self,
            request: tonic::Request<super::QueryExchangeRateRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryExchangeRateResponse>, tonic::Status>;
        /// ExchangeRates returns exchange rates of all denoms
        async fn exchange_rates(
            &self,
            request: tonic::Request<super::QueryExchangeRatesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryExchangeRatesResponse>, tonic::Status>;
        /// TobinTax returns tobin tax of a denom
        async fn tobin_tax(
            &self,
            request: tonic::Request<super::QueryTobinTaxRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTobinTaxResponse>, tonic::Status>;
        /// TobinTaxes returns tobin taxes of all denoms
        async fn tobin_taxes(
            &self,
            request: tonic::Request<super::QueryTobinTaxesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTobinTaxesResponse>, tonic::Status>;
        /// Actives returns all active denoms
        async fn actives(
            &self,
            request: tonic::Request<super::QueryActivesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryActivesResponse>, tonic::Status>;
        /// VoteTargets returns all vote target denoms
        async fn vote_targets(
            &self,
            request: tonic::Request<super::QueryVoteTargetsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryVoteTargetsResponse>, tonic::Status>;
        /// FeederDelegation returns feeder delegation of a validator
        async fn feeder_delegation(
            &self,
            request: tonic::Request<super::QueryFeederDelegationRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryFeederDelegationResponse>, tonic::Status>;
        /// MissCounter returns oracle miss counter of a validator
        async fn miss_counter(
            &self,
            request: tonic::Request<super::QueryMissCounterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryMissCounterResponse>, tonic::Status>;
        /// AggregatePrevote returns an aggregate prevote of a validator
        async fn aggregate_prevote(
            &self,
            request: tonic::Request<super::QueryAggregatePrevoteRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAggregatePrevoteResponse>, tonic::Status>;
        /// AggregatePrevotes returns aggregate prevotes of all validators
        async fn aggregate_prevotes(
            &self,
            request: tonic::Request<super::QueryAggregatePrevotesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAggregatePrevotesResponse>,
            tonic::Status,
        >;
        /// AggregateVote returns an aggregate vote of a validator
        async fn aggregate_vote(
            &self,
            request: tonic::Request<super::QueryAggregateVoteRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAggregateVoteResponse>, tonic::Status>;
        /// AggregateVotes returns aggregate votes of all validators
        async fn aggregate_votes(
            &self,
            request: tonic::Request<super::QueryAggregateVotesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAggregateVotesResponse>, tonic::Status>;
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
                "/terra.oracle.v1beta1.Query/ExchangeRate" => {
                    #[allow(non_camel_case_types)]
                    struct ExchangeRateSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryExchangeRateRequest> for ExchangeRateSvc<T> {
                        type Response = super::QueryExchangeRateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryExchangeRateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).exchange_rate(request).await };
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
                        let method = ExchangeRateSvc(inner);
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
                "/terra.oracle.v1beta1.Query/ExchangeRates" => {
                    #[allow(non_camel_case_types)]
                    struct ExchangeRatesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryExchangeRatesRequest>
                        for ExchangeRatesSvc<T>
                    {
                        type Response = super::QueryExchangeRatesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryExchangeRatesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).exchange_rates(request).await };
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
                        let method = ExchangeRatesSvc(inner);
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
                "/terra.oracle.v1beta1.Query/TobinTax" => {
                    #[allow(non_camel_case_types)]
                    struct TobinTaxSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTobinTaxRequest> for TobinTaxSvc<T> {
                        type Response = super::QueryTobinTaxResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTobinTaxRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).tobin_tax(request).await };
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
                        let method = TobinTaxSvc(inner);
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
                "/terra.oracle.v1beta1.Query/TobinTaxes" => {
                    #[allow(non_camel_case_types)]
                    struct TobinTaxesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTobinTaxesRequest> for TobinTaxesSvc<T> {
                        type Response = super::QueryTobinTaxesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTobinTaxesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).tobin_taxes(request).await };
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
                        let method = TobinTaxesSvc(inner);
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
                "/terra.oracle.v1beta1.Query/Actives" => {
                    #[allow(non_camel_case_types)]
                    struct ActivesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryActivesRequest> for ActivesSvc<T> {
                        type Response = super::QueryActivesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryActivesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).actives(request).await };
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
                        let method = ActivesSvc(inner);
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
                "/terra.oracle.v1beta1.Query/VoteTargets" => {
                    #[allow(non_camel_case_types)]
                    struct VoteTargetsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryVoteTargetsRequest> for VoteTargetsSvc<T> {
                        type Response = super::QueryVoteTargetsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVoteTargetsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).vote_targets(request).await };
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
                        let method = VoteTargetsSvc(inner);
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
                "/terra.oracle.v1beta1.Query/FeederDelegation" => {
                    #[allow(non_camel_case_types)]
                    struct FeederDelegationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryFeederDelegationRequest>
                        for FeederDelegationSvc<T>
                    {
                        type Response = super::QueryFeederDelegationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFeederDelegationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).feeder_delegation(request).await };
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
                        let method = FeederDelegationSvc(inner);
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
                "/terra.oracle.v1beta1.Query/MissCounter" => {
                    #[allow(non_camel_case_types)]
                    struct MissCounterSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryMissCounterRequest> for MissCounterSvc<T> {
                        type Response = super::QueryMissCounterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryMissCounterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).miss_counter(request).await };
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
                        let method = MissCounterSvc(inner);
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
                "/terra.oracle.v1beta1.Query/AggregatePrevote" => {
                    #[allow(non_camel_case_types)]
                    struct AggregatePrevoteSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAggregatePrevoteRequest>
                        for AggregatePrevoteSvc<T>
                    {
                        type Response = super::QueryAggregatePrevoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAggregatePrevoteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).aggregate_prevote(request).await };
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
                        let method = AggregatePrevoteSvc(inner);
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
                "/terra.oracle.v1beta1.Query/AggregatePrevotes" => {
                    #[allow(non_camel_case_types)]
                    struct AggregatePrevotesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAggregatePrevotesRequest>
                        for AggregatePrevotesSvc<T>
                    {
                        type Response = super::QueryAggregatePrevotesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAggregatePrevotesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).aggregate_prevotes(request).await };
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
                        let method = AggregatePrevotesSvc(inner);
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
                "/terra.oracle.v1beta1.Query/AggregateVote" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateVoteSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAggregateVoteRequest>
                        for AggregateVoteSvc<T>
                    {
                        type Response = super::QueryAggregateVoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAggregateVoteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).aggregate_vote(request).await };
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
                        let method = AggregateVoteSvc(inner);
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
                "/terra.oracle.v1beta1.Query/AggregateVotes" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateVotesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAggregateVotesRequest>
                        for AggregateVotesSvc<T>
                    {
                        type Response = super::QueryAggregateVotesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAggregateVotesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).aggregate_votes(request).await };
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
                        let method = AggregateVotesSvc(inner);
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
                "/terra.oracle.v1beta1.Query/Params" => {
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
        const NAME: &'static str = "terra.oracle.v1beta1.Query";
    }
}
/// GenesisState defines the oracle module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub feeder_delegations: ::prost::alloc::vec::Vec<FeederDelegation>,
    #[prost(message, repeated, tag = "3")]
    pub exchange_rates: ::prost::alloc::vec::Vec<ExchangeRateTuple>,
    #[prost(message, repeated, tag = "4")]
    pub miss_counters: ::prost::alloc::vec::Vec<MissCounter>,
    #[prost(message, repeated, tag = "5")]
    pub aggregate_exchange_rate_prevotes: ::prost::alloc::vec::Vec<AggregateExchangeRatePrevote>,
    #[prost(message, repeated, tag = "6")]
    pub aggregate_exchange_rate_votes: ::prost::alloc::vec::Vec<AggregateExchangeRateVote>,
    #[prost(message, repeated, tag = "7")]
    pub tobin_taxes: ::prost::alloc::vec::Vec<TobinTax>,
}
/// FeederDelegation is the address for where oracle feeder authority are
/// delegated to. By default this struct is only used at genesis to feed in
/// default feeder addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeederDelegation {
    #[prost(string, tag = "1")]
    pub feeder_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MissCounter defines an miss counter and validator address pair used in
/// oracle module's genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissCounter {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub miss_counter: u64,
}
/// TobinTax defines an denom and tobin_tax pair used in
/// oracle module's genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TobinTax {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tobin_tax: ::prost::alloc::string::String,
}
