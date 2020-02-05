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

use std::{
	collections::{BTreeMap, BTreeSet},
	sync::Arc,
};
use crate::{error::Error, KeyServerId};

/// Network transport.
pub trait NetworkTransport {
	/// Type of address we need to know to connect remote key servers.
	type Address;

	/// Set key servers we need to connect to.
	fn set_key_servers_set(&self, set: BTreeMap<KeyServerId, Self::Address>);
	/// Are we connected to all required nodes?
	fn is_fully_connected(&self) -> bool;
	/// Get connections snapshot.
	fn snapshot(&self) -> Arc<dyn NetworkSnapshot>;
}

/// Network connections snapshot.
pub trait NetworkSnapshot {
	/// Returns IDs of all nodes that were connected when snapshot has been created.
	fn nodes(&self) -> BTreeSet<KeyServerId>;
	/// Broadcast message to all other nodes.
	fn broadcast(&self, message: Vec<u8>) -> Result<(), Error>;
	/// Send message to given node.
	fn send(&self, to: &KeyServerId, message: Vec<u8>) -> Result<(), Error>;
}
