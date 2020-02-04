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
pub struct KeyShare {
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
	pub versions: Vec<KeyShareVersion>,
}

/// Versioned portion of key share.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyShareVersion {
	/// Version hash (Keccak(time + id_numbers)).
	pub hash: H256,
	/// Nodes ids numbers.
	pub id_numbers: BTreeMap<KeyServerId, Secret>,
	/// Node secret share.
	pub secret_share: Secret,
}


/// Secret Store key storage.
pub trait KeyStorage: Send + Sync {
	/// Insert new key share.
	fn insert(&self, key_id: ServerKeyId, key: KeyShare) -> Result<(), Error>;
	/// Update existing key share.
	fn update(&self, key_id: ServerKeyId, key: KeyShare) -> Result<(), Error>;
	/// Get existing key share.
	fn get(&self, key_id: &ServerKeyId) -> Result<Option<KeyShare>, Error>;
	/// Remove key share.
	fn remove(&self, key_id: &ServerKeyId) -> Result<(), Error>;
	/// Clears the database.
	fn clear(&self) -> Result<(), Error>;
	/// Check if storage contains encryption key
	fn contains(&self, key_id: &ServerKeyId) -> bool;
	/// Iterate through storage.
	fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=(ServerKeyId, KeyShare)> + 'a>;
}
