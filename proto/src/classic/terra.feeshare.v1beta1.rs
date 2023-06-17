// @generated
/// FeeShare defines an instance that organizes fee distribution conditions for
/// the owner of a given smart contract
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeShare {
    /// contract_address is the bech32 address of a registered contract in string
    /// form
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// deployer_address is the bech32 address of message sender. It must be the
    /// same as the contracts admin address.
    #[prost(string, tag = "2")]
    pub deployer_address: ::prost::alloc::string::String,
    /// withdrawer_address is the bech32 address of account receiving the
    /// transaction fees.
    #[prost(string, tag = "3")]
    pub withdrawer_address: ::prost::alloc::string::String,
}
/// Params defines the feeshare module params
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// enable_feeshare defines a parameter to enable the feeshare module
    #[prost(bool, tag = "1")]
    pub enable_fee_share: bool,
    /// developer_shares defines the proportion of the transaction fees to be
    /// distributed to the registered contract owner
    #[prost(string, tag = "2")]
    pub developer_shares: ::prost::alloc::string::String,
    /// allowed_denoms defines the list of denoms that are allowed to be paid to
    /// the contract withdraw addresses. If said denom is not in the list, the fees
    /// will ONLY be sent to the community pool.
    /// If this list is empty, all denoms are allowed.
    #[prost(string, repeated, tag = "3")]
    pub allowed_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GenesisState defines the module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params are the feeshare module parameters
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// FeeShare is a slice of active registered contracts for fee distribution
    #[prost(message, repeated, tag = "2")]
    pub fee_share: ::prost::alloc::vec::Vec<FeeShare>,
}
/// QueryFeeSharesRequest is the request type for the Query/FeeShares RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeSharesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryFeeSharesResponse is the response type for the Query/FeeShares RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeSharesResponse {
    /// FeeShare is a slice of all stored Reveneue
    #[prost(message, repeated, tag = "1")]
    pub feeshare: ::prost::alloc::vec::Vec<FeeShare>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryFeeShareRequest is the request type for the Query/FeeShare RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeShareRequest {
    /// contract_address of a registered contract in bech32 format
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryFeeShareResponse is the response type for the Query/FeeShare RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeShareResponse {
    /// FeeShare is a stored Reveneue for the queried contract
    #[prost(message, optional, tag = "1")]
    pub feeshare: ::core::option::Option<FeeShare>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params is the returned FeeShare parameter
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryDeployerFeeSharesRequest is the request type for the
/// Query/DeployerFeeShares RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDeployerFeeSharesRequest {
    /// deployer_address in bech32 format
    #[prost(string, tag = "1")]
    pub deployer_address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryDeployerFeeSharesResponse is the response type for the
/// Query/DeployerFeeShares RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDeployerFeeSharesResponse {
    /// contract_addresses is the slice of registered contract addresses for a
    /// deployer
    #[prost(string, repeated, tag = "1")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryWithdrawerFeeSharesRequest is the request type for the
/// Query/WithdrawerFeeShares RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawerFeeSharesRequest {
    /// withdrawer_address in bech32 format
    #[prost(string, tag = "1")]
    pub withdrawer_address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryWithdrawerFeeSharesResponse is the response type for the
/// Query/WithdrawerFeeShares RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawerFeeSharesResponse {
    /// contract_addresses is the slice of registered contract addresses for a
    /// withdrawer
    #[prost(string, repeated, tag = "1")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgRegisterFeeShare defines a message that registers a FeeShare
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterFeeShare {
    /// contract_address in bech32 format
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// deployer_address is the bech32 address of message sender. It must be the
    /// same the contract's admin address
    #[prost(string, tag = "2")]
    pub deployer_address: ::prost::alloc::string::String,
    /// withdrawer_address is the bech32 address of account receiving the
    /// transaction fees
    #[prost(string, tag = "3")]
    pub withdrawer_address: ::prost::alloc::string::String,
}
/// MsgRegisterFeeShareResponse defines the MsgRegisterFeeShare response type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterFeeShareResponse {}
/// MsgUpdateFeeShare defines a message that updates the withdrawer address for a
/// registered FeeShare
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateFeeShare {
    /// contract_address in bech32 format
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// deployer_address is the bech32 address of message sender. It must be the
    /// same the contract's admin address
    #[prost(string, tag = "2")]
    pub deployer_address: ::prost::alloc::string::String,
    /// withdrawer_address is the bech32 address of account receiving the
    /// transaction fees
    #[prost(string, tag = "3")]
    pub withdrawer_address: ::prost::alloc::string::String,
}
/// MsgUpdateFeeShareResponse defines the MsgUpdateFeeShare response type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateFeeShareResponse {}
/// MsgCancelFeeShare defines a message that cancels a registered FeeShare
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelFeeShare {
    /// contract_address in bech32 format
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// deployer_address is the bech32 address of message sender. It must be the
    /// same the contract's admin address
    #[prost(string, tag = "2")]
    pub deployer_address: ::prost::alloc::string::String,
}
/// MsgCancelFeeShareResponse defines the MsgCancelFeeShare response type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelFeeShareResponse {}
include!("terra.feeshare.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
