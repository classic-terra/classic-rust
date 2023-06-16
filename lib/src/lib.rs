#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    clippy::derive_partial_eq_without_eq
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod traits;
mod type_urls;

pub use cosmos_sdk_proto as cosmos;
pub use prost;
pub use prost_types::Any;
pub use tendermint_proto as tendermint;

/// The version (commit hash) of Terra Classic core used when generating this library.
pub const CLASSIC_VERSION: &str = include_str!("classic/CLASSIC_COMMIT");

/// Alliance protobuf definitions.
pub mod classic {

    /// Market module on Classic
    pub mod market {
        include!("classic/terra.market.v1beta1.rs");
    }

    /// Oracle module on Classic
    pub mod oracle {
        include!("classic/terra.oracle.v1beta1.rs");
    }

    /// Treasury module on Classic
    pub mod treasury {
        include!("classic/terra.treasury.v1beta1.rs");
    }
}
