//! Collects EVE-Data from different sources and inserts them into the database
//! for later usage

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

use caph_collector::{Character, Market, Sde, Time};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Mutex};
use tracing::Level;

/// Env variable for the database URL
const DATABASE_URL: &str = "DATABASE_URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        //.pretty()
        .with_max_level(Level::ERROR)
        .init();

    let pg_addr = std::env::var(DATABASE_URL)
        .expect("The ENV 'DATABASE_URL' does not exist");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;

    let pool_copy = pool.clone();
    let sde = tokio::task::spawn(async move {
        let mut sde = Sde::new(pool_copy);

        let mut last_ts = 0;
        let mut last_iso = String::new();

        let mut error = None;

        loop {
            let timed = std::time::Instant::now();
            tracing::info!("SDE task started.");
            if let Err(e) = sde.run().await {
                tracing::error!("Error running SDE task {:?}", e);
                error = Some(e.to_string());
            } else {
                tracing::info!("SDE task done.");
            }

            let time = Time::default();
            let next_ts = time.datetime_next_sde().timestamp();
            let next_iso = time.datetime_next_sde();
            let duration = timed.elapsed().as_secs();

            last_ts = next_ts;
            last_iso = next_iso.to_string();
            tokio::time::sleep(time.duration_next_sde()).await;
        }
    });

    let pool_copy = pool.clone();
    let character = tokio::task::spawn(async move {
        let mut character = Character::new(pool_copy);

        let mut last_ts = 0;
        let mut last_iso = String::new();

        let mut error = None;

        loop {
            let timed = std::time::Instant::now();
            if let Err(e) = character.task().await {
                tracing::error!("Error running character task {:?}", e);
                error = Some(e.to_string());
            } else {
                tracing::info!("Character task done.");
            }

            let time = Time::default();
            let next_ts = time.datetime_next_character().timestamp();
            let next_iso = time.datetime_next_character();
            let duration = timed.elapsed().as_secs();

            error = None;
            last_ts = next_ts;
            last_iso = next_iso.to_string();
            tokio::time::sleep(time.duration_next_character()).await;
        }
    });

    let pool_copy = pool.clone();
    let market = tokio::task::spawn(async move {
        let mut market = Market::new(pool_copy);

        let mut last_ts = 0;
        let mut last_iso = String::new();

        let mut error = None;

        loop {
            let timed = std::time::Instant::now();

            if let Err(e) = market.task().await {
                tracing::error!("Error running market task {:?}", e);
                error = Some(e.to_string());
            } else {
                tracing::info!("market task done.");
            }

            let time = Time::default();
            let next_ts = time.datetime_next_market().timestamp();
            let next_iso = time.datetime_next_market();
            let duration = timed.elapsed().as_secs();

            error = None;
            last_ts = next_ts;
            last_iso = next_iso.to_string();
            tokio::time::sleep(time.duration_next_market()).await;
        }
    });

    let _ = tokio::join!(
        character,
        market,
        sde,
    );

    Ok(())
}
