use chrono::{NaiveDateTime, NaiveTime, Timelike, Utc};
use std::time::Duration;

/// Helper for handling time so that the tasks run at a specific time
pub struct Time {
    /// Time this struct was created
    current_ts: i64,
}

impl Time {
    /// Creates a new duration for the next character task run
    ///
    pub fn duration_next_market(&self) -> Duration {
        let next = self.datetime_next_market().timestamp();
        let diff = next - self.current_ts;
        Duration::from_secs(diff as u64)
    }

    /// Creates a new date time aligned to the next 15 minute time
    ///
    pub fn datetime_next_market(&self) -> NaiveDateTime {
        let date_time = NaiveDateTime::from_timestamp(
            self.current_ts,
            0
        );

        let minutes = match date_time.minute() + 1 {
            0..= 15 => 15,
            16..=30 => 30,
            31..=45 => 45,
            _ => 0
        };

        let hours = if minutes == 0 {
            date_time.hour() + 1
        } else {
            date_time.hour()
        };

        let hours = if hours == 24 {
            0
        } else {
            hours
        };

        let time = NaiveTime::from_hms(hours, minutes, 0);
        NaiveDateTime::new(date_time.date(), time)
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            current_ts: Utc::now().timestamp()
        }
    }
}
