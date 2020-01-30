// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use ethereum_types::H256;
use parity_crypto::publickey::{Public, Signature};
use crate::{
	KeyServerId, KeyServerPublic, ServerKeyId,
	error::Error,
	requester::Requester,
};

/// Server key generation artifacts.
pub struct ServerKeyGenerationArtifacts {
	/// Public portion of generated server key.
	pub key: Public,
}

/// Server key retrieval artifacts.
pub struct ServerKeyRetrievalArtifacts {
	/// Public portion of retrieved server key.
	pub key: Public,
	/// Threshold that has been used to generate server key.
	pub threshold: usize,
}

/// Server key (SK) generator.
pub trait ServerKeyGenerator {
	/// SK generation future.
	type GenerateKeyFuture: Future<Output = Result<ServerKeyGenerationArtifacts, Error>> + Send;
	/// SK restore future.
	type RestoreKeyFuture: Future<Output = Result<ServerKeyRetrievalArtifacts, Error>> + Send;

	/// Generate new SK.
	/// `key_id` is the caller-provided identifier of generated SK.
	/// `author` is the author of key entry.
	/// `threshold + 1` is the minimal number of nodes, required to restore private key.
	/// Result is a public portion of SK.
	fn generate_key(
		&self,
		key_id: ServerKeyId,
		author: Requester,
		threshold: usize,
	) -> Self::GenerateKeyFuture;
	/// Retrieve public portion of previously generated SK.
	/// `key_id` is identifier of previously generated SK.
	/// `author` is the same author, that has created the server key.
	/// If `author` is `None`, then author-check is omitted.
	fn restore_key_public(
		&self,
		key_id: ServerKeyId,
		author: Option<Requester>,
	) -> Self::RestoreKeyFuture;
}

/// Dcument key generation artifacts.
pub struct DocumentKeyGenerationArtifacts {
	/// Generated document key. UNENCRYPTED.
	pub document_key: Public,
}

/// Document key retrieval artifacts.
pub struct DocumentKeyRetrievalArtifacts {
	/// Restored document key. UNENCRYPTED.
	pub document_key: Public,
}

/// Document key common retrieval artifacts.
///
/// This data isn't enough to recover document key and could only be used for
/// establishing consensus over `common_point` and `threshold`.
pub struct DocumentKeyCommonRetrievalArtifacts {
	/// The common point of portion of encrypted document keys. Common point is
	/// shared among all key servers that aware of the given document key.
	pub common_point: Public,
	/// Threshold that has been used to generate associated server key.
	pub threshold: usize,
}

/// Document key shadow retrieval artifacts.
///
/// The data is enough to decrypt document key by the owner of corresponding
/// requester key.
pub struct DocumentKeyShadowRetrievalArtifacts {
	/// The common point of portion of encrypted document keys. Common point is
	/// shared among all key servers that aware of the given document key.
	pub common_point: Public,
	/// Threshold that has been used to generate associated server key.
	pub threshold: usize,
	/// Partially decrypted document key.
	pub encrypted_document_key: Public,
	/// Key servers that has participated in decryption session along with their
	/// shadow coefficients. Shadow coefficients are encrypted with requester public
	/// key. After decryption, they can be used to finally decrypt document key.
	pub participants_coefficients: BTreeMap<KeyServerId, Vec<u8>>,
}

/// Document key (DK) server.
pub trait DocumentKeyServer: ServerKeyGenerator {
	/// DK store future.
	type StoreDocumentKeyFuture: Future<Output = Result<(), Error>> + Send;
	/// DK generation future.
	type GenerateDocumentKeyFuture: Future<Output = Result<DocumentKeyGenerationArtifacts, Error>> + Send;
	/// DK restore future.
	type RestoreDocumentKeyFuture: Future<Output = Result<DocumentKeyRetrievalArtifacts, Error>> + Send;
	/// DK common part restore future.
	type RestoreDocumentKeyCommonFuture: Future<Output = Result<DocumentKeyCommonRetrievalArtifacts, Error>> + Send;
	/// DK shadow restore future.
	type RestoreDocumentKeyShadowFuture: Future<Output = Result<DocumentKeyShadowRetrievalArtifacts, Error>> + Send;

	/// Store externally generated DK.
	/// `key_id` is identifier of previously generated SK.
	/// `author` is the same author, that has created the server key.
	/// `common_point` is a result of `k * T` expression, where `T` is generation point
	/// and `k` is random scalar in EC field.
	/// `encrypted_document_key` is a result of `M + k * y` expression, where `M` is unencrypted document key (point on EC),
	///   `k` is the same scalar used in `common_point` calculation and `y` is previously generated public part of SK.
	fn store_document_key(
		&self,
		key_id: ServerKeyId,
		author: Requester,
		common_point: Public,
		encrypted_document_key: Public,
	) -> Self::StoreDocumentKeyFuture;
	/// Generate and store both SK and DK. This is a shortcut for consequent calls of `generate_key` and `store_document_key`.
	/// The only difference is that DK is generated by DocumentKeyServer (which might be considered unsafe).
	/// `key_id` is the caller-provided identifier of generated SK.
	/// `author` is the author of server && document key entry.
	/// `threshold + 1` is the minimal number of nodes, required to restore private key.
	/// Result is a DK, encrypted with caller public key.
	fn generate_document_key(
		&self,
		key_id: ServerKeyId,
		author: Requester,
		threshold: usize,
	) -> Self::GenerateDocumentKeyFuture;
	/// Restore previously stored DK.
	/// DK is decrypted on the key server (which might be considered unsafe), and then encrypted with caller public key.
	/// `key_id` is identifier of previously generated SK.
	/// `requester` is the one who requests access to document key. Caller must be on ACL for this function to succeed.
	/// Result is a DK, encrypted with caller public key.
	fn restore_document_key(
		&self,
		key_id: ServerKeyId,
		requester: Requester,
	) -> Self::RestoreDocumentKeyFuture;
	/// Restore portion of DK that is the same among all key servers.
	fn restore_document_key_common(
		&self,
		key_id: ServerKeyId,
		requester: Requester,
	) -> Self::RestoreDocumentKeyCommonFuture;
	/// Restore previously stored DK.
	/// To decrypt DK on client:
	/// 1) use requestor secret key to decrypt secret coefficients from result.decrypt_shadows
	/// 2) calculate decrypt_shadows_sum = sum of all secrets from (1)
	/// 3) calculate decrypt_shadow_point: decrypt_shadows_sum * result.common_point
	/// 4) calculate decrypted_secret: result.decrypted_secret + decrypt_shadow_point
	/// Result is a DK shadow.
	fn restore_document_key_shadow(
		&self,
		key_id: ServerKeyId,
		requester: Requester,
	) -> Self::RestoreDocumentKeyShadowFuture;
}

/// Schnorr signing artifacts.
pub struct SchnorrSigningArtifacts {
	/// C portion of Schnorr signature.
	pub signature_c: H256,
	/// S portion of Schnorr signature.
	pub signature_s: H256,
}

/// ECDSA signing artifacts.
pub struct EcdsaSigningArtifacts {
	/// ECDSA signature.
	pub signature: Signature,
}

/// Message signer.
pub trait MessageSigner: ServerKeyGenerator {
	/// Schnorr signing future.
	type SignMessageSchnorrFuture: Future<Output = Result<SchnorrSigningArtifacts, Error>> + Send;
	/// ECDSA signing future.
	type SignMessageECDSAFuture: Future<Output = Result<EcdsaSigningArtifacts, Error>> + Send;

	/// Generate Schnorr signature for message with previously generated SK.
	/// `key_id` is the caller-provided identifier of generated SK.
	/// `requester` is the one who requests access to server key private.
	/// `message` is the message to be signed.
	/// Result is a signed message, encrypted with caller public key.
	fn sign_message_schnorr(
		&self,
		key_id: ServerKeyId,
		requester: Requester,
		message: H256,
	) -> Self::SignMessageSchnorrFuture;
	/// Generate ECDSA signature for message with previously generated SK.
	/// WARNING: only possible when SK was generated using t <= 2 * N.
	/// `key_id` is the caller-provided identifier of generated SK.
	/// `signature` is `key_id`, signed with caller public key.
	/// `message` is the hash of message to be signed.
	/// Result is a signed message, encrypted with caller public key.
	fn sign_message_ecdsa(
		&self,
		key_id: ServerKeyId,
		signature: Requester,
		message: H256,
	) -> Self::SignMessageECDSAFuture;
}

/// Administrative sessions server.
pub trait AdminSessionsServer {
	/// Change servers set future.
	type ChangeServersSetFuture: Future<Output = Result<(), Error>> + Send;

	/// Change servers set so that nodes in new_servers_set became owners of shares for all keys.
	/// And old nodes (i.e. cluster nodes except new_servers_set) have clear databases.
	/// WARNING: newly generated keys will be distributed among all cluster nodes. So this session
	/// must be followed with cluster nodes change (either via contract, or config files).
	fn change_servers_set(
		&self,
		old_set_signature: Signature,
		new_set_signature: Signature,
		new_servers_set: BTreeSet<KeyServerPublic>,
	) -> Self::ChangeServersSetFuture;
}

/// Key server.
pub trait KeyServer: AdminSessionsServer + DocumentKeyServer + MessageSigner + Send + Sync + 'static {
}
