mod error;
mod market;
mod sde;
mod time;

use self::market::*;
use self::sde::*;
use self::time::*;

use cachem::ConnectionPool;
use metrix_exporter::Metrix;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let metrix = Metrix::new(env!("CARGO_PKG_NAME").into(), "0.0.0.0:8889").await?;
    let pool = ConnectionPool::new("0.0.0.0:9999".into(), metrix.get_sender(), 10).await?;

    let metrix_sender = metrix.get_sender();

    tokio::task::spawn(async { metrix.listen().await; });

    log::info!("Start SDE");
    let mut sde = Sde::new(metrix_sender.clone(), pool.clone()).await;
    sde.run().await.unwrap();
    log::info!("Done SDE");

    let metrix_copy = metrix_sender.clone();
    let pool_copy = pool.clone();
    tokio::task::spawn(async {
        let mut market = Market::new(metrix_copy, pool_copy);

        loop {
            log::info!("Start market");
            if let Err(e) = market.task().await {
                log::error!("Error running market task {:?}", e);
            }
            log::info!("Done market");

            let next_run = duration_to_next_30_minute();
            tokio::time::sleep(next_run).await; // Run on the next 30 minute interval
        }
    })
    .await
    .unwrap();

    Ok(())
}
