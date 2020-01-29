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

pub mod acl_storage;
pub mod error;
pub mod key_server;
pub mod key_server_set;
pub mod requester;
pub mod serialization;
pub mod service;

/// Node id.
pub type NodeId = parity_crypto::publickey::Public;
///

pub type KeyServerId = NodeId;
///
pub type RequesterId = ethereum_types::Address;
/// Server key id. When key is used to encrypt document, it could be document contents hash.
pub type ServerKeyId = ethereum_types::H256;


///
pub type CommonPoint = parity_crypto::publickey::Public;
///
pub type EncryptedPoint = parity_crypto::publickey::Public;

pub type ServerKey = parity_crypto::publickey::Public;

pub type DecryptedSecret = parity_crypto::publickey::Public;

pub type DocumentKeyShadow = Vec<u8>;

pub type EncryptedDocumentKey = Vec<u8>;

pub struct DocumentKeyCommon {
	pub threshold: usize,
	pub common_point: CommonPoint,
}

///
pub type MessageHash = ethereum_types::H256;

pub type RequestSignature = parity_crypto::publickey::Signature;

pub type EncryptedMessageSignature = Vec<u8>;

/// Shadow decryption result.
#[derive(Clone, Debug, PartialEq)]
pub struct EncryptedDocumentKeyShadow {
	/// Decrypted secret point. It is partially decrypted if shadow decryption was requested.
	pub decrypted_secret: parity_crypto::publickey::Public,
	/// Shared common point.
	pub common_point: Option<parity_crypto::publickey::Public>,
	/// If shadow decryption was requested: shadow decryption coefficients, encrypted with requestor public.
	pub decrypt_shadows: Option<Vec<Vec<u8>>>,
}
/*
/// Link
pub trait KeyServerLink: Send + Sync {
	/// Spawn auxiliary background task.
	fn spawn_task(&self, Box<Future<Output = ()> + Send + Sync>);
	/// Send service task.
	fn send_service_task(&self, task: ServiceTask) -> Box<Future<Output = Result<ServiceResponse, Error>> + Send + Sync>;
}

*/