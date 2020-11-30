mod error;
mod market;
mod metrics;
mod postgres;
mod sde;

use async_std::future;
use async_std::prelude::*;
use futures::stream::FuturesUnordered;
use metrics::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init()?;

    let metrics = Metrics::default();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://caph:caph@cygnus.local:5432/caph_eve")
        .await?;

    // Make sure the database has the newest scripts applied
    let mut conn = pool.acquire().await?;
    conn.execute(include_str!("./tables.sql")).await?;

    let pool_copy = pool.clone();
    let sde_metrics = metrics.sde.clone();
    let sde = async_std::task::spawn(async {
        let mut sde = sde::Sde::new(pool_copy, sde_metrics);

        loop {
            if let Err(e) = sde.background().await {
                log::error!("Error running SDE task: {:?}", e);
            }

            future::ready(1u32)
                .delay(std::time::Duration::from_secs(24 * 60 * 60)) // 24 hours
                .await;
        }
    });

    let pool_copy = pool.clone();
    let market_metrics = metrics.market.clone();
    let market = async_std::task::spawn(async {
        let mut market = market::Market::new(pool_copy, market_metrics);

        loop {
            if let Err(e) = market.background().await {
                log::error!("Error running market task {:?}", e);
            }

            future::ready(1u32)
                .delay(std::time::Duration::from_secs(30 * 60)) // 30 minutes
                .await;
        }
    });

    let pool_copy = pool.clone();
    let postgres_metrics = metrics.postgres.clone();
    let postgres = async_std::task::spawn(async {
        let mut postgres = postgres::PostgresService::new(pool_copy, postgres_metrics);

        loop {
            if let Err(e) = postgres.background().await {
                log::error!("Error running postgres task {:?}", e);
            }

            future::ready(1u32)
                .delay(std::time::Duration::from_secs(15 * 60)) // 15 minutes
                .await;
        }
    });

    let metric_server = async_std::task::spawn(async move { metrics.task().await });

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
