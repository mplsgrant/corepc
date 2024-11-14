// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Generating ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use bitcoin::BlockHash;
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `generate`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Generate(pub Vec<BlockHash>);

impl Generate {
    /// Returns the number of blocks generated.
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns true if 0 blocks were generated.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

/// Models the result of JSON-RPC method `generatetoaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GenerateToAddress(pub Vec<BlockHash>);

impl GenerateToAddress {
    /// Returns the number of blocks generated.
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns true if 0 blocks were generated.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}
