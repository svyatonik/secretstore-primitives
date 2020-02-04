// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity Secret Store.

// Parity Secret Store is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Secret Store is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Secret Store.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::BTreeMap;
use ethereum_types::H256;
use parity_crypto::publickey::{Address, Public, Secret};
use crate::{error::Error, KeyServerId, ServerKeyId};

/// Encrypted key share, stored by key storage on the single key server.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DocumentKeyShare {
	/// Author of the entry.
	pub author: Address,
	/// Decryption threshold (at least threshold + 1 nodes are required to decrypt data).
	pub threshold: usize,
	/// Server public key.
	pub public: Public,
	/// Common (shared) encryption point.
	pub common_point: Option<Public>,
	/// Encrypted point.
	pub encrypted_point: Option<Public>,
	/// Key share versions.
	pub versions: Vec<DocumentKeyShareVersion>,
}

/// Versioned portion of document key share.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentKeyShareVersion {
	/// Version hash (Keccak(time + id_numbers)).
	pub hash: H256,
	/// Nodes ids numbers.
	pub id_numbers: BTreeMap<KeyServerId, Secret>,
	/// Node secret share.
	pub secret_share: Secret,
}


/// Secret Store key storage.
pub trait KeyStorage: Send + Sync {
	/// Insert new key.
	fn insert(&self, key_id: ServerKeyId, key: DocumentKeyShare) -> Result<(), Error>;
	/// Update document encryption key
	fn update(&self, document: ServerKeyId, key: DocumentKeyShare) -> Result<(), Error>;
	/// Get document encryption key
	fn get(&self, document: &ServerKeyId) -> Result<Option<DocumentKeyShare>, Error>;
	/// Remove document encryption key
	fn remove(&self, document: &ServerKeyId) -> Result<(), Error>;
	/// Clears the database
	fn clear(&self) -> Result<(), Error>;
	/// Check if storage contains document encryption key
	fn contains(&self, document: &ServerKeyId) -> bool;
	/// Iterate through storage
	fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=(ServerKeyId, DocumentKeyShare)> + 'a>;
}