//! Core library containing services that manage application data

#![forbid(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
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

/// Macros that are used across the library
mod macros;
/// Wrapper for the market EVE-API
mod market;
/// Wrapper for handling projects
mod project;

pub use self::market::*;
pub use self::project::*;
