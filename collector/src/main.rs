mod error;
mod market;
mod metrics;
mod postgres;
mod sde;

use tokio::stream::StreamExt;
use futures::stream::FuturesUnordered;
use metrics::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init()?;

    let metrics = Metrics::default();

    let db_uri = std::env::var("DB").unwrap_or("postgres://caph:caph@cygnus.local:5432/caph_test".into());
    let pool = PgPoolOptions::new()
        .max_connections(25)
        .connect(&db_uri)
        .await?;

    // Make sure the database has the newest scripts applied
    let mut conn = pool.acquire().await?;
    conn.execute(include_str!("./tables.sql")).await?;

    let pool_copy = pool.clone();
    let sde_metrics = metrics.sde.clone();
    let sde = tokio::task::spawn(async {
        let mut sde = sde::Sde::new(pool_copy, sde_metrics);

        loop {
            if let Err(e) = sde.background().await {
                log::error!("Error running SDE task: {:?}", e);
            }

            tokio::time::delay_for(Duration::from_secs(24 * 60 * 60)).await; // 24 hours
        }
    });

    let pool_copy = pool.clone();
    let market_metrics = metrics.market.clone();
    let market = tokio::task::spawn(async {
        let mut market = market::Market::new(pool_copy, market_metrics);

        loop {
            if let Err(e) = market.background().await {
                log::error!("Error running market task {:?}", e);
            }

            tokio::time::delay_for(Duration::from_secs(30 * 60)).await; // 30 minutes
        }
    });

    let pool_copy = pool.clone();
    let postgres_metrics = metrics.postgres.clone();
    let postgres = tokio::task::spawn(async {
        let mut postgres = postgres::PostgresService::new(pool_copy, postgres_metrics);

        loop {
            if let Err(e) = postgres.background().await {
                log::error!("Error running postgres task {:?}", e);
            }

            tokio::time::delay_for(Duration::from_secs(15 * 60)).await; // 15 minutes
        }
    });

    let metric_server = tokio::task::spawn(async move { metrics.task().await });

    let mut background_tasks = FuturesUnordered::new();
    background_tasks.push(sde);
    background_tasks.push(market);
    background_tasks.push(postgres);
    background_tasks.push(metric_server);

    while let Some(_) = background_tasks.next().await {
        // just make sure that the background tasks are not killed
    }

    Ok(())
}
