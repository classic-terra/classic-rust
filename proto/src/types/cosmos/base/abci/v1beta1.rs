use osmosis_std_derive::CosmwasmExt;
/// NOTE: The following type is not implemented due to current limitations of code generator
/// which currently has issue with tendermint_proto.
/// This will be fixed in the upcoming release.
#[allow(dead_code)]
struct TxResponse {}
/// ABCIMessageLog defines a structure containing an indexed tx ABCI message log.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.abci.v1beta1.ABCIMessageLog")]
pub struct AbciMessageLog {
    #[prost(uint32, tag = "1")]
    pub msg_index: u32,
    #[prost(string, tag = "2")]
    pub log: ::prost::alloc::string::String,
    /// Events contains a slice of Event objects that were emitted during some
    /// execution.
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<StringEvent>,
}
/// StringEvent defines en Event object wrapper where all the attributes
/// contain key/value pairs that are strings instead of raw bytes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.abci.v1beta1.StringEvent")]
pub struct StringEvent {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
}
/// Attribute defines an attribute wrapper where the key and value are
/// strings instead of raw bytes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.abci.v1beta1.Attribute")]
pub struct Attribute {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// GasInfo defines tx execution gas context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.abci.v1beta1.GasInfo")]
pub struct GasInfo {
    /// GasWanted is the maximum units of work we allow this tx to perform.
    #[prost(uint64, tag = "1")]
    pub gas_wanted: u64,
    /// GasUsed is the amount of gas actually consumed.
    #[prost(uint64, tag = "2")]
    pub gas_used: u64,
}
/// NOTE: The following type is not implemented due to current limitations of code generator
/// which currently has issue with tendermint_proto.
/// This will be fixed in the upcoming release.
#[allow(dead_code)]
struct Result {}
/// SimulationResponse defines the response generated when a transaction is
/// successfully simulated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.abci.v1beta1.SimulationResponse")]
pub struct SimulationResponse {
    #[prost(message, optional, tag = "1")]
    pub gas_info: ::core::option::Option<GasInfo>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<Result>,
}
/// MsgData defines the data returned in a Result object during message
/// execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.abci.v1beta1.MsgData")]
#[deprecated]
pub struct MsgData {
    #[prost(string, tag = "1")]
    pub msg_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// TxMsgData defines a list of MsgData. A transaction will have a MsgData object
/// for each message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.abci.v1beta1.TxMsgData")]
pub struct TxMsgData {
    /// data field is deprecated and not populated.
    #[deprecated]
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<MsgData>,
    /// msg_responses contains the Msg handler responses packed into Anys.
    ///
    /// Since: cosmos-sdk 0.46
    #[prost(message, repeated, tag = "2")]
    pub msg_responses: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// SearchTxsResult defines a structure for querying txs pageable
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.abci.v1beta1.SearchTxsResult")]
pub struct SearchTxsResult {
    /// Count of all txs
    #[prost(uint64, tag = "1")]
    pub total_count: u64,
    /// Count of txs in current page
    #[prost(uint64, tag = "2")]
    pub count: u64,
    /// Index of current page, start from 1
    #[prost(uint64, tag = "3")]
    pub page_number: u64,
    /// Count of total pages
    #[prost(uint64, tag = "4")]
    pub page_total: u64,
    /// Max count txs per page
    #[prost(uint64, tag = "5")]
    pub limit: u64,
    /// List of txs in current page
    #[prost(message, repeated, tag = "6")]
    pub txs: ::prost::alloc::vec::Vec<TxResponse>,
}
