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

use std::collections::BTreeSet;
use ethereum_types::H256;
use parity_crypto::publickey::{Public, Signature};
use crate::{requester::Requester, ServerKeyId, KeyServerPublic};

/// Service contract task.
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceTask {
	// === Server key related tasks ===

	/// Generate server key (server_key_id, author, threshold).
	GenerateServerKey(ServerKeyId, Requester, usize),
	/// Retrieve server key (server_key_id, requester).
	RetrieveServerKey(ServerKeyId, Option<Requester>),

	// === Document key store tasks ===

	/// Generate document key (server_key_id, author, threshold).
	GenerateDocumentKey(ServerKeyId, Requester, usize),
	/// Store document key (server_key_id, author, common_point, encrypted_point).
	StoreDocumentKey(ServerKeyId, Requester, Public, Public),

	// === Document key retrieval tasks ===

	/// Retrieve document key (server_key_id, requester).
	RetrieveDocumentKey(ServerKeyId, Requester),
	/// Retrieve document key (server_key_id, requester).
	RetrieveShadowDocumentKey(ServerKeyId, Requester),

	// === Signing tasks ===

	/// Generate Schnorr signature for the message (server_key_id, requester, message).
	SchnorrSignMessage(ServerKeyId, Requester, H256),
	/// Generate ECDSA signature for the message.
	EcdsaSignMessage(ServerKeyId, Requester, H256),

	// === Administrative tasks ===

	/// Change servers set (old_set_signature, new_set_signature, new_set).
	ChangeServersSet(Signature, Signature, BTreeSet<KeyServerPublic>),
}
