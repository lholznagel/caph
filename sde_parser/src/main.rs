//! Parses parts of the EVE provided SDE-File into SQL-Statements for the main
//! application.

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

/// Module for creating the blueprints SQL-Code
mod blueprints;
/// Module for creating the items SQL-Code
mod items;

use std::io::prelude::*;
use std::{path::Path, fs::File};
use tracing_subscriber::EnvFilter;

/// Folder that contains the input file
pub const FOLDER_INPUT: &str  = "input";
/// Folder for all SQL files
pub const FOLDER_OUTPUT: &str = "output";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let current_dir = std::env::current_dir()?;
    if !Path::new(
            &format!(
                "{}/{}/blueprints.yaml",
                current_dir.to_str().unwrap_or_default(),
                FOLDER_INPUT
            )
        ).exists() {

        tracing::error!(
            "File 'blueprints.yaml' is not in {}/blueprints.yaml",
            FOLDER_INPUT
        );
    }

    let blueprints = blueprints::run()?;
    let mut fs = File::create(format!(
            "{}/{}/blueprints.sql",
            current_dir.to_str().unwrap_or_default(),
            FOLDER_OUTPUT
    ))?;
    fs.write_all(blueprints.as_bytes())?;

    let items = items::run()?;
    let mut fs = File::create(format!(
        "{}/{}/items.sql",
        current_dir.to_str().unwrap_or_default(),
        FOLDER_OUTPUT
    ))?;
    fs.write_all(items.as_bytes())?;

    Ok(())
}

