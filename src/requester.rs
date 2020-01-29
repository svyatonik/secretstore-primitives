use parity_crypto::publickey::{Address, Public, public_to_address, recover};
use crate::{RequestSignature, ServerKeyId};

/// Requester identification data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Requester {
	/// Requested with server key id signature.
	Signature(RequestSignature),
	/// Requested with public key.
	Public(Public),
	/// Requested with verified address.
	Address(Address),
}

impl Requester {
	pub fn public(&self, server_key_id: &ServerKeyId) -> Result<Public, String> {
		match *self {
			Requester::Signature(ref signature) => recover(signature, server_key_id)
				.map_err(|e| format!("bad signature: {}", e)),
			Requester::Public(ref public) => Ok(public.clone()),
			Requester::Address(_) => Err("cannot recover public from address".into()),
		}
	}

	pub fn address(&self, server_key_id: &ServerKeyId) -> Result<Address, String> {
		self.public(server_key_id)
			.map(|p| public_to_address(&p))
	}
}

impl From<RequestSignature> for Requester {
	fn from(signature: RequestSignature) -> Requester {
		Requester::Signature(signature)
	}
}

impl From<Public> for Requester {
	fn from(public: Public) -> Requester {
		Requester::Public(public)
	}
}

impl From<Address> for Requester {
	fn from(address: Address) -> Requester {
		Requester::Address(address)
	}
}

impl std::fmt::Display for Requester {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "{:?}", self)
	}
}
