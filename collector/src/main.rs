//! Collects EVE-Data from different sources and inserts them into the database
//! for later usage

use caph_collector::{Character, Sde, duration_next_sde_download, duration_to_next_10_minute};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing::{instrument, Level};

/// Env variable for the database URL
const PG_ADDR: &str = "DATABASE_URL";

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(Level::INFO)
        .init();

    let pg_addr = std::env::var(PG_ADDR).unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;

    let pool_copy = pool.clone();
    let sde = tokio::task::spawn(async {
        let mut sde = Sde::new(pool_copy);

        loop {
            tracing::info!("SDE task started.");
            if let Err(e) = sde.run().await {
                tracing::error!("Error running SDE task {:?}", e);
            }
            tracing::info!("SDE task done.");

            let next_run = duration_next_sde_download();
            tokio::time::sleep(next_run).await;
        }
    });

    let pool_copy = pool.clone();
    let character = tokio::task::spawn(async {
        let mut character = Character::new(pool_copy);

        loop {
            tracing::info!("Character task started.");
            if let Err(e) = character.task().await {
                tracing::error!("Error running character task {:?}", e);
            }
            tracing::info!("Character task done.");

            let next_run = duration_to_next_10_minute()
                .unwrap_or_else(|_| Duration::from_secs(30 * 60));
            tokio::time::sleep(next_run).await; // Run on the next 30 minute interval
        }
    });

    let _ = tokio::join!(
        character,
        sde,
    );

    Ok(())
}
