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
    clippy::missing_safety_doc
)]
#![warn(
    clippy::await_holding_lock,
    clippy::get_unwrap,
    clippy::map_unwrap_or,
    clippy::unwrap_in_result,
    clippy::unwrap_used
)]
#![allow(clippy::redundant_field_names)]
#![feature(stmt_expr_attributes)]

/// Module for handling characters
mod character;
/// Module containing clients to the EVE-API
mod client;
/// Module for handling corporations
mod corporation;
/// Module containing possible errors
mod error;
/// Module containing all macros
mod macros;
/// Module for all universe api requests
mod universe;

/// Collects all services under one import
pub mod services {
    pub use crate::character::EveCharacterService;
    pub use crate::corporation::EveCorporationService;
}

pub use self::character::*;
pub use self::client::*;
pub use self::corporation::*;
pub use self::error::*;
pub use self::universe::*;

use serde::{Deserialize, Deserializer, Serialize};

eve_id!(AllianceId, i32, u32);
eve_id!(CategoryId, i32, u32);
eve_id!(CharacterId, i32, u32);
eve_id!(CorporationId, i32, u32);
eve_id!(GroupId, i32, u32);
eve_id!(ItemId, i64, u64);
eve_id!(JobId, i32, u32);
eve_id!(LocationId, i64, u64);
eve_id!(StationId, i64, u64);
eve_id!(SystemId, i64, u64);
eve_id!(TypeId, i32, u32);
eve_id!(RegionId, i32, u32);

/// Represents an asset
#[derive(Debug, Deserialize)]
pub struct AssetEntry {
    /// Unique Id of the item
    pub item_id: ItemId,
    /// Flag of the location, eg. MedSlot6, Deliveries, Wallet
    pub location_flag: String,
    /// Either a id of a structurte, container or ship
    pub location_id: LocationId,
    /// Stored quantity
    pub quantity: i32,
    /// [TypeId] of the item
    pub type_id: TypeId,

    /// True if the item is a copy
    #[serde(default)]
    pub is_blueprint_copy: bool,
}

/// Represents a single character blueprint
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlueprintEntry {
    /// Unique ID of the asset
    pub item_id: ItemId,
    /// Id of the location the asset is located in
    pub location_id: LocationId,
    /// Material efficiency of the blueprint, max 10
    pub material_efficiency: i32,
    /// Time efficiency of the blueprint, max 20
    pub time_efficiency: i32,
    /// A range of numbers with a minimum of -2 and no maximum value where -1
    /// is an original and -2 is a copy. It can be a positive integer if it is
    /// a stack of blueprint originals fresh from the market (e.g. no
    /// activities performed on them yet).
    pub quantity: i32,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original
    pub runs: i32,
    /// Type id of the asset
    pub type_id: TypeId,
}

/// Represents a transaction entry
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndustryJobEntry {
    /// Activity of the job
    #[serde(deserialize_with = "IndustryActivity::from")]
    #[serde(rename = "activity_id")]
    pub activity: IndustryActivity,
    /// ID of the blueprint
    pub blueprint_type_id: TypeId,
    /// Station where the blueprint is located, can also be containers
    pub blueprint_location_id: LocationId,
    /// Current status of the job
    /// active, cancelled, delivered, paused, ready, reverted
    pub status: String,
    /// When the job was submitted
    pub start_date: String,
    /// When the job is doen
    pub end_date: String,
    /// Cost of the job
    pub cost: Option<f32>,
    /// Number of runs
    pub runs: u32,
    /// Unique id for the job
    pub job_id: u32,
    /// CharacterId of the character that strted the job
    pub installer_id: CharacterId,
    /// [CorporationId] of the corporation that owns this job
    #[serde(default)]
    pub corporation_id: Option<CorporationId>,
}

/// List of all industry activities
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum IndustryActivity {
    /// Manufacturing of things
    Manufacturing,
    /// Researching time efficiency
    MaterialEfficiencyResearch,
    /// Researching material efficiency
    TimeEfficiencyResearch,
    /// Making blueprint copies
    Copying,
    /// The process of creating a more advanced item based on an existing item
    Invention,
    /// The process of combining raw and intermediate materials to create advanced components
    Reactions,
    /// No matches were found
    Unknown,
}

impl IndustryActivity {
    fn from<'de, D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match u8::deserialize(d)? {
            1 => Self::Manufacturing,
            3 => Self::MaterialEfficiencyResearch,
            4 => Self::TimeEfficiencyResearch,
            5 => Self::Copying,
            8 => Self::Invention,
            9 => Self::Reactions,
            _ => Self::Unknown,
        })
    }
}
