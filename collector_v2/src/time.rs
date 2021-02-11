use chrono::{NaiveDateTime, NaiveTime, Timelike, Utc};
use std::time::Duration;

/// Sets the given timestamp to the last 30 minute mark back
pub fn previous_30_minute(timestamp: u64) -> u64 {
    let date_time = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let time = if date_time.minute() >= 30 {
        NaiveTime::from_hms(date_time.hour(), 30, 0)
    } else {
        NaiveTime::from_hms(date_time.hour(), 0, 0)
    };
    NaiveDateTime::new(date_time.date(), time).timestamp() as u64
}

/// Create a duration to the next 30 minute mark
pub fn duration_to_next_30_minute() -> Duration {
    let current = Utc::now().timestamp() as u64;
    let next = next_30_minutes(current);
    let diff = next - current;
    Duration::from_secs(diff)
}

/// Adds 30 minutes to the given timestamp
fn next_30_minutes(timestamp: u64) -> u64 {
    let date_time = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let date_time = date_time.checked_add_signed(chrono::Duration::minutes(30)).unwrap();
    date_time.timestamp() as u64
}
