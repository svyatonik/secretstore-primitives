use std::collections::BTreeSet;
use crate::{requester::Requester, RequestSignature, ServerKeyId, CommonPoint, EncryptedPoint, KeyServerId, MessageHash};

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
	StoreDocumentKey(ServerKeyId, Requester, CommonPoint, EncryptedPoint),

	// === Document key retrieval tasks ===

	/// Retrieve document key (server_key_id, requester).
	RetrieveDocumentKey(ServerKeyId, Requester),
	/// Retrieve document key (server_key_id, requester).
	RetrieveShadowDocumentKey(ServerKeyId, Requester),

	// === Signing tasks ===

	/// Generate Schnorr signature for the message (server_key_id, requester, message).
	SchnorrSignMessage(ServerKeyId, Requester, MessageHash),
	/// Generate ECDSA signature for the message.
	EcdsaSignMessage(ServerKeyId, Requester, MessageHash),

	// === Administrative tasks ===

	/// Change servers set (old_set_signature, new_set_signature, new_set).
	ChangeServersSet(RequestSignature, RequestSignature, BTreeSet<KeyServerId>),
}
