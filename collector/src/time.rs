use chrono::{NaiveDateTime, NaiveTime, Timelike, Utc};
use std::time::Duration;

/// Helper for handling time so that the tasks run at a specific time
pub struct Time {
    /// Time this struct was created
    current_ts: i64,
}

impl Time {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            current_ts: Utc::now().timestamp()
        }
    }

    /// Creates a new duration for the next character task run
    ///
    pub fn duration_next_character(&self) -> Duration {
        let next = self.datetime_next_character().timestamp();
        let diff = next - self.current_ts;
        Duration::from_secs(diff as u64)
    }

    /// Creates a new date time aligned to the next 10 minute time
    ///
    pub fn datetime_next_character(&self) -> NaiveDateTime {
        let date_time = NaiveDateTime::from_timestamp(
            self.current_ts,
            0
        );

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
        NaiveDateTime::new(date_time.date(), time)
    }

    /// Creates a new duration to the next 14:20:00 time.
    ///
    /// Eve´s downtime is at 14:00, so giving them 30 minutes should be ok.
    ///
    pub fn duration_next_sde(&self) -> Duration {
        let next = self.datetime_next_sde().timestamp();

        // Execute at exactly 14:30
        let diff = next - self.current_ts;
        Duration::from_secs(diff as u64)
    }

    /// Creates a new datetime either to 14:30 of the current day or 14:30 of
    /// the next day
    ///
    pub fn datetime_next_sde(&self) -> NaiveDateTime {
        // Create a naive date time and add one day to it
        let date_time = NaiveDateTime::from_timestamp(
            self.current_ts,
            0
        );

        if date_time.hour() > 14 ||
           date_time.hour() == 14 && date_time.minute() > 30 {

            let _ = date_time
                .checked_add_signed(chrono::Duration::days(1));
        }

        // Creates a new naive date time based on the date time that is one day
        // ahead. We take the date and set the hms to 14:30:00.
        NaiveDateTime::new(
            date_time.date(),
            NaiveTime::from_hms(14, 30, 0)
        )
    }
}
