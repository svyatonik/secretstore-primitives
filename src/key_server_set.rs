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
use std::net::SocketAddr;
use ethereum_types::H256;
use crate::NodeId;

///
pub type MigrationId = H256;

/// Key Server Set state.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct KeyServerSetSnapshot {
	/// Current set of key servers.
	pub current_set: BTreeMap<NodeId, SocketAddr>,
	/// New set of key servers.
	pub new_set: BTreeMap<NodeId, SocketAddr>,
	/// Current migration data.
	pub migration: Option<KeyServerSetMigration>,
}

/// Key server set migration.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct KeyServerSetMigration {
	/// Migration id.
	pub id: MigrationId,
	/// Migration set of key servers. It is the new_set at the moment of migration start.
	pub set: BTreeMap<NodeId, SocketAddr>,
	/// Master node of the migration process.
	pub master: NodeId,
	/// Is migration confirmed by this node?
	pub is_confirmed: bool,
}

/// Key Server Set
pub trait KeyServerSet: Send + Sync {
	/// Is this node currently isolated from the set?
	fn is_isolated(&self) -> bool;
	/// Get server set state.
	fn snapshot(&self) -> KeyServerSetSnapshot;
	/// Start migration.
	fn start_migration(&self, migration_id: MigrationId);
	/// Confirm migration.
	fn confirm_migration(&self, migration_id: MigrationId);
}
