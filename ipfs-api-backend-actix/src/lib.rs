// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Connect to an IPFS API using a client implemented with Actix.
//!
//! # Example
//!
//! ```rust
//!  use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
//!  use ipfs_api_backend_actix::response::VersionResponse;
//!
//!  async fn example() -> Result<VersionResponse, ipfs_api_backend_actix::Error> {
//!     let client = IpfsClient::default();
//!
//!     client.version().await
//! }
//! ```

extern crate actix_multipart_rfc7578 as multipart;

mod backend;
mod error;

pub use crate::{backend::ActixBackend as IpfsClient, error::Error};
pub use ipfs_api_prelude::{
    request::{self, KeyType, Logger, LoggingLevel, ObjectTemplate},
    response, ApiError, BackendWithGlobalOptions, GlobalOptions, IpfsApi, TryFromUri,
};
pub use multipart::client::multipart::Form;
