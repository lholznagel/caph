use chrono::{NaiveDateTime, NaiveTime, Timelike, Utc};
use std::time::Duration;

use crate::error::CollectorError;

/// Create a duration to the next 30 minute mark
pub fn duration_to_next_10_minute() -> Result<Duration, CollectorError> {
    let current = Utc::now().timestamp() as u64;
    let next = next_10_minutes()?;
    let diff = next - current;
    Ok(Duration::from_secs(diff))
}

/// Creates a new duration to the next 14:20:00 time.
///
/// Eve´s downtime is at 14:00, so giving them 30 minutes should be ok.
pub fn duration_next_sde_download() -> Result<Duration, CollectorError> {
    // Current timestamp
    let timestamp = Utc::now().timestamp();
    // Create a naive date time and add one day to it
    let date_time = NaiveDateTime::from_timestamp(timestamp as i64, 0);

    if date_time.hour() > 14 ||
       date_time.hour() == 14 && date_time.minute() > 30 {

        date_time
            .checked_add_signed(chrono::Duration::days(1))
            .ok_or(CollectorError::ChronoError)?;
    }

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

/// Gets the next full ten minutes time frame
fn next_10_minutes() -> Result<u64, CollectorError> {
    let date_time = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);

    let minutes = match date_time.minute() {
         0..= 9 => 10,
        10..=19 => 20,
        20..=29 => 30,
        30..=39 => 40,
        40..=49 => 50,
        50..=59 =>  0,
        _ => 0
    };

    let hours = if minutes == 0 {
        date_time.hour() + 1
    } else {
        date_time.hour()
    };

    let time = NaiveTime::from_hms(hours, minutes, 0);
    let date = NaiveDateTime::new(date_time.date(), time).timestamp() as u64;
    Ok(date)
}

