//! Exposes the EVE-API and EVE-SDE as a single library without making a
//! difference between those two.
//!
//! All SDE-Data are cached in the library.
//! Some but not all API results are cached, read the manual
//!
//! For EVE-API-Authentication an EVE-Auth-Client is exposed.

#![forbid(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
)]
#![warn(
    clippy::await_holding_lock,
    clippy::get_unwrap,
    clippy::map_unwrap_or,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
)]
#![allow(
    clippy::redundant_field_names
)]
#![feature(stmt_expr_attributes)]

/// Module for handling assets
mod asset;
/// Module for handling blueprints
mod blueprint;
/// Module for handling characters
mod character;
/// Module containing clients to the EVE-API
mod client;
/// Module containing possible errors
mod error;
/// Module containing all macros
mod macros;
/// Module for handling market requests
mod market;
/// Module for handling reprocessing information
mod reprocess;
/// Module for handling schematics
mod schematic;
/// Module for handling stations
mod station;
/// Module for handling systems
mod system;
/// Module for handling unique names
mod unique_name;
/// Module contains the wrapper for managing the SDE.zip
mod zip;

/// Collects all services under one import
pub mod services {
    pub use crate::asset::ConnectAssetService;
    pub use crate::blueprint::ConnectBlueprintService;
    pub use crate::character::ConnectCharacterService;
    pub use crate::market::ConnectMarketService;
    pub use crate::reprocess::ConnectReprocessService;
    pub use crate::schematic::ConnectSchematicService;
    pub use crate::station::ConnectStationService;
    pub use crate::system::ConnectSystemService;
    pub use crate::unique_name::ConnectUniqueNameService;
}

pub use self::asset::*;
pub use self::blueprint::*;
pub use self::character::*;
pub use self::client::*;
pub use self::error::*;
pub use self::market::*;
pub use self::reprocess::*;
pub use self::schematic::*;
pub use self::station::*;
pub use self::system::*;
pub use self::unique_name::*;
pub use self::zip::*;

use serde::{Deserialize, Serialize};

eve_id!(AllianceId,    i32);
eve_id!(CategoryId,    i32);
eve_id!(CharacterId,   i32);
eve_id!(CorporationId, i32);
eve_id!(GroupId,       i32);
eve_id!(ItemId,        i64);
eve_id!(JobId,         i32);
eve_id!(LocationId,    i64);
eve_id!(StationId,     i64);
eve_id!(SystemId,      i64);
eve_id!(TypeId,        i32);
eve_id!(RegionId,      i32);
