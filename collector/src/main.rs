mod character;
mod error;
mod market;
mod sde;
mod time;

use self::character::*;
use self::market::*;
use self::sde::*;
use self::time::*;

use cachem::v2::ConnectionPool;
use caph_eve_data_wrapper::EveDataWrapper;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let pool = ConnectionPool::new("0.0.0.0:55555", 10).await?;

    log::info!("Preparing SDE");
    let eve = EveDataWrapper::new().await?;
    log::info!("Prepared SDE");

    let eve_copy = eve.clone();
    let pool_copy = pool.clone();
    let sde = tokio::task::spawn(async {
        let mut sde = Sde::new(eve_copy, pool_copy);

        loop {
            log::info!("SDE start");
            if let Err(e) = sde.run().await {
                log::error!("Error running sde task {:?}", e);
            }
            log::info!("SDE done");

            let next_run = duration_next_sde_download()
                .unwrap_or_else(|_| Duration::from_secs(24 * 60 * 60));
            tokio::time::sleep(next_run).await;
        }
    });

    let eve_copy = eve.clone();
    let pool_copy = pool.clone();
    let character = tokio::task::spawn(async {
        let mut market = Character::new(eve_copy, pool_copy);

        loop {
            log::info!("Character start");
            if let Err(e) = market.task().await {
                log::error!("Error running market task {:?}", e);
            }
            log::info!("Character done");

            let next_run = duration_to_next_30_minute()
                .unwrap_or_else(|_| Duration::from_secs(30 * 60));
            tokio::time::sleep(next_run).await; // Run on the next 30 minute interval
        }
    });

    /*let eve_copy = eve.clone();
    let pool_copy = pool.clone();
    let market = tokio::task::spawn(async {
        let mut market = Market::new(eve_copy, pool_copy);

        loop {
            log::info!("Market start");
            if let Err(e) = market.task().await {
                log::error!("Error running market task {:?}", e);
            }
            log::info!("Market done");

            let next_run = duration_to_next_30_minute()
                .unwrap_or_else(|_| Duration::from_secs(30 * 60));
            tokio::time::sleep(next_run).await; // Run on the next 30 minute interval
        }
    });*/

    let _ = tokio::join!(
        character,
        //market,
        sde,
    );

    Ok(())
}
