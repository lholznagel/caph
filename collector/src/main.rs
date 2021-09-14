//! Collects EVE-Data from different sources and inserts them into the database
//! for later usage.

mod character;
mod error;
mod sde;
mod time;

use self::character::*;
use self::sde::*;
use self::time::*;

use caph_eve_data_wrapper::EveDataWrapper;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing_subscriber::FmtSubscriber;
use tracing::instrument;
use tracing::Level;

const PG_ADDR: &str = "DATABASE_URL";

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let pg_addr = std::env::var(PG_ADDR).unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;

    tracing::info!("Preparing SDE.");
    let eve = EveDataWrapper::new().await?;
    tracing::info!("Prepared SDE.");

    let eve_copy = eve.clone();
    let pool_copy = pool.clone();
    let sde = tokio::task::spawn(async {
        let mut sde = Sde::new(eve_copy, pool_copy);

        loop {
            tracing::info!("SDE task started.");
            if let Err(e) = sde.run().await {
                tracing::error!("Error running SDE task {:?}", e);
            }
            tracing::info!("SDE task done.");

            let next_run = duration_next_sde_download()
                .unwrap_or_else(|_| Duration::from_secs(24 * 60 * 60));
            tokio::time::sleep(next_run).await;
        }
    });

    let eve_copy = eve.clone();
    let pool_copy = pool.clone();
    let character = tokio::task::spawn(async {
        let mut character = Character::new(eve_copy, pool_copy);

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
