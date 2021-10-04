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

use caph_collector::{Character, ProcessStatus, Sde, Status, TaskState, Time, start_server};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Mutex};
use tracing::Level;

/// Env variable for the database URL
const DATABASE_URL: &str = "DATABASE_URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(Level::INFO)
        .init();

    let pg_addr = std::env::var(DATABASE_URL)
        .expect("The ENV 'DATABASE_URL' does not exist");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;

    let state = Arc::new(Mutex::new(TaskState::default()));

    let state_copy = state.clone();
    let pool_copy = pool.clone();
    let sde = tokio::task::spawn(async move {
        let mut sde = Sde::new(pool_copy);

        let mut last_ts = 0;
        let mut last_iso = String::new();

        let mut status;
        let mut error = None;

        loop {
            let timed = std::time::Instant::now();
            tracing::info!("SDE task started.");
            if let Err(e) = sde.run().await {
                tracing::error!("Error running SDE task {:?}", e);
                status = ProcessStatus::Error;
                error = Some(e.to_string());
            } else {
                tracing::info!("SDE task done.");
                status = ProcessStatus::Ok;
            }

            let time = Time::new();
            let next_ts = time.datetime_next_sde().timestamp();
            let next_iso = time.datetime_next_sde();
            let duration = timed.elapsed().as_secs();

            let state = Status::new(
                last_ts,
                last_iso,

                next_ts,
                next_iso.to_string(),

                duration,
                status,

                error.clone()
            );

            last_ts = next_ts;
            last_iso = next_iso.to_string();
            {
                state_copy
                    .lock()
                    .unwrap()
                    .sde_status(state);
            }
            tokio::time::sleep(time.duration_next_sde()).await;
        }
    });

    let state_copy = state.clone();
    let pool_copy = pool.clone();
    let character = tokio::task::spawn(async move {
        let mut character = Character::new(pool_copy);

        let mut last_ts = 0;
        let mut last_iso = String::new();

        let mut status;
        let mut error = None;

        loop {
            let timed = std::time::Instant::now();
            tracing::info!("Character task started.");
            if let Err(e) = character.task().await {
                tracing::error!("Error running character task {:?}", e);
                status = ProcessStatus::Error;
                error = Some(e.to_string());
            } else {
                tracing::info!("Character task done.");
                status = ProcessStatus::Ok;
            }

            let time = Time::new();
            let next_ts = time.datetime_next_character().timestamp();
            let next_iso = time.datetime_next_character();
            let duration = timed.elapsed().as_secs();

            let state = Status::new(
                last_ts,
                last_iso,

                next_ts,
                next_iso.to_string(),

                duration,
                status,

                error.clone()
            );

            last_ts = next_ts;
            last_iso = next_iso.to_string();
            {
                state_copy
                    .lock()
                    .unwrap()
                    .character_status(state);
            }
            tokio::time::sleep(time.duration_next_character()).await;
        }
    });

    let pool_copy = pool.clone();
    let state_copy = state.clone();
    let server = tokio::task::spawn(async {
        if let Err(_) = start_server(pool_copy, state_copy).await {
            tracing::error!("Failed to start server");
        }
    });

    let _ = tokio::join!(
        character,
        sde,

        server
    );

    Ok(())
}
