mod error;
mod market;
mod sde;
mod time;

use self::market::*;
use self::sde::*;
use self::time::*;

use cachem::ConnectionPool;
use caph_eve_data_wrapper::EveDataWrapper;
use metrix_exporter::Metrix;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let metrix = Metrix::new(env!("CARGO_PKG_NAME").into(), "0.0.0.0:8889").await?;
    let pool = ConnectionPool::new("0.0.0.0:9999".into(), metrix.get_sender(), 10).await?;

    let metrix_sender = metrix.get_sender();

    let metrix = tokio::task::spawn(async { metrix.listen().await; });

    log::info!("Preparing SDE");
    let eve = EveDataWrapper::new().await?;
    log::info!("Prepared SDE");

    let eve_copy = eve.clone();
    let metrix_copy = metrix_sender.clone();
    let pool_copy = pool.clone();
    let sde = tokio::task::spawn(async {
        let mut sde = Sde::new(eve_copy, metrix_copy, pool_copy);

        loop {
            log::info!("SDE start");
            if let Err(e) = sde.run().await {
                log::error!("Error running sde task {:?}", e);
            }
            log::info!("SDE done");

            let next_run = duration_next_sde_download();
            tokio::time::sleep(next_run).await;
        }
    });

    let eve_copy = eve.clone();
    let metrix_copy = metrix_sender.clone();
    let pool_copy = pool.clone();
    let market = tokio::task::spawn(async {
        let mut market = Market::new(eve_copy, metrix_copy, pool_copy);

        loop {
            log::info!("Market start");
            if let Err(e) = market.task().await {
                log::error!("Error running market task {:?}", e);
            }
            log::info!("Market done");

            let next_run = duration_to_next_30_minute();
            tokio::time::sleep(next_run).await; // Run on the next 30 minute interval
        }
    });

    let _ = tokio::join!(
        market,
        metrix,
        sde,
    );

    Ok(())
}
