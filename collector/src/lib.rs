//! Library for collection EVE-Data from the EVE-API and SDE

#![forbid(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
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

/// Handles all character stuff
mod character;
/// Collection of all errors
mod error;
/// Handles all EVE-SDE stuff
mod sde;
/// Exposes an API interface
mod server;
/// Helper for time
mod time;

pub use self::character::*;
pub use self::error::*;
pub use self::sde::*;
pub use self::server::*;
pub use self::time::*;
