use chrono::{NaiveDateTime, NaiveTime, Timelike, Utc};
use std::time::Duration;

use crate::error::CollectorError;

/// Sets the given timestamp to the previous 20 or 50 minute mark
pub fn previous_30_minute(timestamp: u64) -> Result<u64, CollectorError> {
    let date_time = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let time = if date_time.minute() >= 50 {
        NaiveTime::from_hms(date_time.hour(), 50, 0)
    } else if date_time.minute() < 20 {
        let duration = chrono::Duration::hours(1);
        let date_time = date_time.checked_sub_signed(duration).ok_or(CollectorError::ChronoError)?;
        NaiveTime::from_hms(date_time.hour(), 50, 0)
    } else {
        NaiveTime::from_hms(date_time.hour(), 20, 0)
    };
    Ok(NaiveDateTime::new(date_time.date(), time).timestamp() as u64)
}

/// Create a duration to the next 30 minute mark
pub fn duration_to_next_30_minute() -> Result<Duration, CollectorError> {
    let current = Utc::now().timestamp() as u64;
    let next = next_30_minutes(current)?;
    let diff = next - current;
    Ok(Duration::from_secs(diff))
}

/// Creates a new duration to the next 14:20:00 time.
///
/// Eve´s downtime is at 14:00, so giving them 20 minutes should be ok.
pub fn duration_next_sde_download() -> Result<Duration, CollectorError> {
    // Current timestamp
    let timestamp = Utc::now().timestamp();
    // Create a naive date time and add one day to it
    let date_time = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let date_time = date_time.checked_add_signed(chrono::Duration::days(1)).ok_or(CollectorError::ChronoError)?;

    // Creates a new naive date time based on the date time that is one day
    // ahead. We take the date and set the hms to 14:30:00.
    let next = NaiveDateTime::new(
        date_time.date(),
        NaiveTime::from_hms(14, 30, 0)
    )
    .timestamp();

    // Execute at exactly 14:30
    let diff = next - timestamp;
    Ok(Duration::from_secs(diff as u64))
}

/// Adds 30 minutes to the given timestamp
fn next_30_minutes(timestamp: u64) -> Result<u64, CollectorError> {
    let date_time = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    // add 30 minutes
    let date_time = date_time.checked_add_signed(chrono::Duration::minutes(30)).ok_or(CollectorError::ChronoError)?;
    // calculate it down to be even 30 minutes
    previous_30_minute(date_time.timestamp() as u64)
}

#[cfg(test)]
mod time_tests {
    use super::*;

    #[test]
    fn previous_30_min_01() {
        // 1970.01.01 00:21:00
        let start = 21 * 60;
        let is = previous_30_minute(start).unwrap();

        let expected = 20 * 60;
        assert_eq!(is, expected);
    }

    #[test]
    fn previous_30_min_02() {
        // 1970.01.01 00:21:00
        let start = 51 * 60;
        let is = previous_30_minute(start).unwrap();

        let expected = 50 * 60;
        assert_eq!(is, expected);
    }

    #[test]
    fn previous_30_min_03() {
        // 1970.01.01 01:10:00
        let start = 60 * 60 + 10 * 60;
        let is = previous_30_minute(start).unwrap();

        let expected = 50 * 60;
        assert_eq!(is, expected);
    }
}
