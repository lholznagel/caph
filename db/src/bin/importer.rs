use caph_db::{MarketOrderEntry, MarketOrderInfoEntry};
use chrono::{Duration, prelude::*};
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufStream};

// psql -U caph -d caph_eve -c "\copy (SELECT * FROM market_orders) TO '/tmp/market_order_info.csv' WITH CSV DELIMITER ',' HEADER;"
const ORDERS: &'static str = "market_order_info.csv";
// psql -U caph -d caph_eve -c "\copy (SELECT * FROM market_history) TO '/tmp/market_history.csv' WITH CSV DELIMITER ',' HEADER;"
const HISTORY: &'static str = "market_history.csv";
const DELIMITER: &'static str = ",";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let order_infos = load_market_order_infos().await?;
    let mut orders = load_market_order(&order_infos).await?;
    flatten_order_time(&mut orders);
    fix_collision(&mut orders);

    let orders_by_item = collect_by_item(orders);
    let mut tritanium = orders_by_item.get(&34).unwrap().clone();
    tritanium.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());

    let mut data = VecDeque::new();
    let mut timestamp = tritanium.first().unwrap().timestamp;
    while timestamp <= Utc::now().timestamp() as u64 {
        let collected = tritanium
            .iter()
            .filter(|x| x.timestamp == timestamp)
            .collect::<Vec<_>>();

        data.push_front(collected.clone());
        timestamp = next_30_minutes(timestamp);
    }

    for (i, x) in data.iter().enumerate() {
        println!("{:10} {:?}", i, x);
    }

    Ok(())
}

fn collect_by_item(orders: HashMap<u64, Vec<MarketOrderEntry>>) -> HashMap<u32, Vec<MarketOrderEntry>> {
    let mut item_map = HashMap::new();

    for (_, entries) in orders {
        let item_id = entries.first().unwrap().item_id;
        item_map
            .entry(item_id)
            .and_modify(|x: &mut Vec<MarketOrderEntry>| x.extend(entries.clone()))
            .or_insert(entries);
    }

    item_map
}

/// Flattens the time the praevious 30 minute mark
fn flatten_order_time(map: &mut HashMap<u64, Vec<MarketOrderEntry>>) {
    for (_, orders) in map {
        for order in orders.iter_mut() {
            order.timestamp = previous_30_minute(order.timestamp);
        }
    }
}

/// Sets the given timestamp to the last 30 minute mark back
fn previous_30_minute(timestamp: u64) -> u64 {
    let date_time = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let time = if date_time.minute() >= 30 {
        NaiveTime::from_hms(date_time.hour(), 30, 0)
    } else {
        NaiveTime::from_hms(date_time.hour(), 0, 0)
    };
    NaiveDateTime::new(date_time.date(), time).timestamp() as u64
}

/// Sets the given timestamp to the next 30 minute mark
fn next_30_minutes(timestamp: u64) -> u64 {
    let date_time = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let date_time = date_time.checked_add_signed(Duration::minutes(30)).unwrap();
    date_time.timestamp() as u64
}

/// When a collision is detected, it moves the second colliding timestamp to 
/// the next 30 minute mark
fn fix_collision(map: &mut HashMap<u64, Vec<MarketOrderEntry>>) {
    for (_, orders) in map {
        if !has_collision(orders.clone()) {
            continue;
        }

        for i in 0..orders.len() - 1 {
            let a = orders.get(i).unwrap();
            let b = orders.get(i + 1).unwrap();

            if a.timestamp == b.timestamp {
                 orders.get_mut(i + 1).unwrap().timestamp = next_30_minutes(b.timestamp);
            }
        }
    }
}

fn has_collision(data: Vec<MarketOrderEntry>) -> bool {
    let current_length = data.len();
    let mut orders = data.clone();
    orders.dedup_by(|a, b| a.timestamp.cmp(&b.timestamp) == Ordering::Equal);
    orders.len() != current_length
}

async fn load_market_order_infos() -> Result<HashMap<u64, MarketOrderInfoEntry>, Box<dyn std::error::Error>> {
    let file = File::open(ORDERS).await?;
    let buf = BufStream::new(file);
    let mut lines = buf.lines();

    // ignore the header line
    let _ = lines.next_line().await?;

    let mut map = HashMap::new();
    while let Some(x) = lines.next_line().await? {
        let order_info = line_to_market_order_info_entry(x);
        map.insert(order_info.order_id, order_info);
    }

    Ok(map)
}

async fn load_market_order(infos: &HashMap<u64, MarketOrderInfoEntry>) -> Result<HashMap<u64, Vec<MarketOrderEntry>>, Box<dyn std::error::Error>> {
    let file = File::open(HISTORY).await?;
    let buf = BufStream::new(file);
    let mut lines = buf.lines();

    // ignore the header line
    let _ = lines.next_line().await?;

    let mut map = HashMap::new();
    while let Some(x) = lines.next_line().await? {
        if let Some(order) = line_to_market_order_entry(x, &infos) {
            map
                .entry(order.order_id)
                .and_modify(|x: &mut Vec<MarketOrderEntry>| x.push(order))
                .or_insert(vec![order]);
        } else {
            continue;
        };
    }

    Ok(map)
}

fn line_to_market_order_info_entry(x: String) -> MarketOrderInfoEntry {
    let mut splitted = x.split(DELIMITER);
    let issued = splitted
        .next()
        .map(|x|
            Utc
                .datetime_from_str(x, "%Y-%m-%dT%H:%M:%SZ")
                .unwrap()
                .naive_local()
                .timestamp() as u64
        )
        .unwrap();

    let volume_total = splitted
        .next()
        .map(|x| x.parse().unwrap())
        .unwrap();

    let system_id = splitted
        .next()
        .map(|x| x.parse().unwrap())
        .unwrap();

    let item_id = splitted
        .next()
        .map(|x| x.parse().unwrap())
        .unwrap();

    let order_id = splitted
        .next()
        .map(|x| x.parse().unwrap())
        .unwrap();

    let location_id = splitted
        .next()
        .map(|x| x.parse().unwrap())
        .unwrap();

    let price = splitted
        .next()
        .map(|x| x.parse().unwrap())
        .unwrap();

    let is_buy_order = splitted
        .next()
        .map(|x| x == "t")
        .unwrap();

    MarketOrderInfoEntry::new(
        order_id,
        issued,
        volume_total,
        system_id,
        item_id,
        location_id,
        price,
        is_buy_order,
    )
}

fn line_to_market_order_entry(x: String, infos: &HashMap<u64, MarketOrderInfoEntry>) -> Option<MarketOrderEntry> {
    let mut splitted = x.split(DELIMITER);
    let volume_remain = splitted
        .next()
        .map(|x| x.parse().unwrap())
        .unwrap();

    let timestamp = splitted
        .next()
        .map(|x| x[0..10].parse().unwrap())
        .unwrap();

    let order_id = splitted
        .next()
        .map(|x| x.parse().unwrap())
        .unwrap();

    let item_id = if let Some(x) = infos.get(&order_id) {
        x.item_id
    } else {
        // if the order id does not exist, we just skip it
        return None;
    };

    Some(MarketOrderEntry::new(
        order_id,
        timestamp,
        volume_remain,
        item_id,
    ))
}
