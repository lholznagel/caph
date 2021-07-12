use cachem::{ConnectionPool, Protocol};
use caph_db::{FetchMarketOrderInfoBulkReq, FetchMarketOrderInfoResBulk, FetchRawMarketOrderReq, FetchRawMarketOrderRes, MarketItemOrder, MarketOrderInfoEntry};
use chrono::Utc;
use metrix_exporter::Metrix;
use std::fmt;

type Result<T> = std::result::Result<T, AnalyzerError>;

#[derive(Debug)]
enum AnalyzerError {
    Cachem(cachem::CachemError),
    Metrix(Box<dyn std::error::Error>),
}

impl std::error::Error for AnalyzerError {  }

impl From<cachem::CachemError> for AnalyzerError {
    fn from(x: cachem::CachemError) -> Self {
        Self::Cachem(x)
    }
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let metrix = Metrix::new(env!("CARGO_PKG_NAME").into(), "0.0.0.0:8889").await.map_err(AnalyzerError::Metrix)?;
    let pool = ConnectionPool::new("0.0.0.0:9999", metrix.get_sender(), 10).await?;
    let mut conn = pool.acquire().await?;

    let type_id = 28606u32;

    // Fetches all market data for the given typeid
    let market_data = Protocol::request::<_, FetchRawMarketOrderRes>(
        &mut conn,
        FetchRawMarketOrderReq(type_id)
    )
    .await
    .map(|x| x.0)?;

    // Generates a list of order ids
    let mut order_ids = market_data
        .iter()
        .map(|x| x.order_id)
        .collect::<Vec<_>>();
    order_ids.sort();
    order_ids.dedup();

    // Fetches all order id informations
    let market_infos = Protocol::request::<_, FetchMarketOrderInfoResBulk>(
        &mut conn,
        FetchMarketOrderInfoBulkReq(order_ids)
    )
    .await
    .map(|x| x.0)?;

    let buy_info  = info_type(market_infos.iter(), true);
    let sell_info = info_type(market_infos.iter(), false);

    let buy_market = market_data
        .iter()
        .filter(|x| buy_info.iter().find(|y| x.order_id == y.order_id).is_some())
        .collect::<Vec<_>>();
    let sell_market = market_data
        .iter()
        .filter(|x| sell_info.iter().find(|y| x.order_id == y.order_id).is_some())
        .collect::<Vec<_>>();

    let (buy_max, buy_min) = min_max_price(buy_info.iter());
    let (sell_max, sell_min) = min_max_price(sell_info.iter());

    println!("===== Stats =====");
    println!("Count data:  \t\t{}", market_data.len());
    println!("Count orders:\t\t{}", market_infos.len());
    println!("Expired:     \t\t{}", expired(market_data.iter(), market_infos.iter()).len());
    println!("\tVolume:      \t{}", expired_volume(market_data.iter()).len());
    println!("\tTime:        \t{}", expired_time(market_infos.iter()).len());
    println!();

    println!("===== Buy =====");
    println!("Orders:    \t\t{:?}", buy_info.len());
    println!("Data:        \t\t{}", buy_market.len());
    println!("Max price:   \t\t{}", buy_max);
    println!("Min price:   \t\t{}", buy_min);
    println!("Total Volume:\t\t{}", volume_total(buy_info.iter()));
    println!();

    println!("===== Sell =====");
    println!("Orders:    \t\t{:?}", sell_info.len());
    println!("Data:        \t\t{}", sell_market.len());
    println!("Max price:   \t\t{}", sell_max);
    println!("Min price:   \t\t{}", sell_min);
    println!("Total Volume:\t\t{}", volume_total(sell_info.iter()));
    Ok(())
}

fn expired<'a, D, I>(x: D, y: I) -> Vec<u64>
where
    D: Iterator<Item = &'a MarketItemOrder>,
    I: Iterator<Item = &'a MarketOrderInfoEntry>
{
    let mut volume = expired_volume(x);
    let time = expired_time(y);

    volume.extend(time);
    volume
}

fn expired_volume<'a, I>(x: I) -> Vec<u64>
where
    I: Iterator<Item = &'a MarketItemOrder>,
{
    x
        .filter(|x| x.volume == 0)
        .map(|x| x.order_id)
        .collect::<Vec<_>>()
}

fn expired_time<'a, I>(x: I) -> Vec<u64>
where
    I: Iterator<Item = &'a MarketOrderInfoEntry>,
{
    let now = Utc::now().timestamp() as u64;
    x
        .filter(|x| x.expire < now)
        .map(|x| x.order_id)
        .collect::<Vec<_>>()
}

///
/// * `true` -> The info is a buy order
/// * `false` -> The info is a sell order
fn info_type<'a, I>(x: I, is_buy: bool) -> Vec<MarketOrderInfoEntry>
where
    I: Iterator<Item = &'a MarketOrderInfoEntry>
{
    x
        .filter(|x| x.is_buy_order == is_buy)
        .map(|x| *x)
        .collect::<Vec<_>>()
}

fn min_max_price<'a, I>(x: I) -> (f32, f32)
where
    I: Iterator<Item = &'a MarketOrderInfoEntry>
{
    let mut max = f32::MIN;
    let mut min = f32::MAX;
    for i in x {
        max = max.max(i.price);
        min = min.min(i.price);
    }

    (max, min)
}

fn volume_total<'a, I>(x: I) -> u64
where
    I: Iterator<Item = &'a MarketOrderInfoEntry>
{
    x
        .map(|x| x.volume_total as u64)
        .sum::<u64>()
}

