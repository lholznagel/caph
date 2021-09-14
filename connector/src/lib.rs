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

/// Module for handling blueprints
mod blueprint;
/// Module containing clients to the EVE-API
mod client;
/// Module containing possible errors
mod error;
/// Module contains the wrapper for managing the SDE.zip
mod zip;

pub use self::blueprint::*;
pub use self::client::*;
pub use self::error::*;
pub use self::zip::*;