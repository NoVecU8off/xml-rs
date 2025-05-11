// Common types and utilities for FIX protocol
// DO NOT EDIT - This file was automatically generated

/// Trait for converting between FIX messages and Rust types
#[allow(clippy::wrong_self_convention)]
pub trait FixMessage {
    /// Convert the message to a collection of FIX tag-value pairs
    fn to_fix(&self) -> std::collections::HashMap<u32, String>;

    /// Create a message from a collection of FIX tag-value pairs
    fn from_fix(fields: &std::collections::HashMap<u32, String>) -> Result<Self, Box<dyn std::error::Error>>
    where Self: Sized;
}

/// Helper functions for converting between FIX and Rust types
pub mod convert {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Datelike, Timelike};

    /// Parse a FIX boolean value (Y/N)
    pub fn parse_bool(value: &str) -> bool {
        value == "Y"
    }

    /// Format a boolean as a FIX value
    pub fn format_bool(value: bool) -> String {
        if value { "Y" } else { "N" }.to_string()
    }

    /// Parse a FIX UTC timestamp (YYYYMMDD-HH:MM:SS or YYYYMMDD-HH:MM:SS.sss)
    pub fn parse_utc_timestamp(value: &str) -> Result<NaiveDateTime, Box<dyn std::error::Error>> {
        if value.len() < 17 {
            return Err("Invalid timestamp format".into());
        }

        let date_part = &value[0..8];
        let time_part = &value[9..];

        let date = parse_utc_date(date_part)?;
        let time = parse_utc_time(time_part)?;

        Ok(NaiveDateTime::new(date, time))
    }

    /// Format a UTC timestamp in FIX format
    pub fn format_utc_timestamp(dt: &NaiveDateTime) -> String {
        format!("{}-{}", 
            format_utc_date(&dt.date()),
            format_utc_time(&dt.time())
        )
    }

    /// Parse a FIX UTC date (YYYYMMDD)
    pub fn parse_utc_date(value: &str) -> Result<NaiveDate, Box<dyn std::error::Error>> {
        if value.len() != 8 {
            return Err("Invalid date format".into());
        }

        let year: i32 = value[0..4].parse()?;
        let month: u32 = value[4..6].parse()?;
        let day: u32 = value[6..8].parse()?;

        NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| "Invalid date components".into())
    }

    /// Format a UTC date in FIX format
    pub fn format_utc_date(date: &NaiveDate) -> String {
        format!("{}{:02}{:02}", 
            date.year(),
            date.month(),
            date.day()
        )
    }

    /// Parse a FIX UTC time (HH:MM:SS or HH:MM:SS.sss)
    pub fn parse_utc_time(value: &str) -> Result<NaiveTime, Box<dyn std::error::Error>> {
        if value.len() < 8 {
            return Err("Invalid time format".into());
        }

        let hour: u32 = value[0..2].parse()?;
        let minute: u32 = value[3..5].parse()?;
        let second: u32 = value[6..8].parse()?;

        let mut nanosecond: u32 = 0;
        if value.len() > 9 {
            let fraction = &value[9..];
            let mut ns: u32 = fraction.parse()?;
            // Adjust to nanoseconds based on the number of digits
            for _ in 0..(9 - fraction.len()) {
                ns *= 10;
            }
            nanosecond = ns;
        }

        NaiveTime::from_hms_nano_opt(hour, minute, second, nanosecond)
            .ok_or_else(|| "Invalid time components".into())
    }

    /// Format a UTC time in FIX format
    pub fn format_utc_time(time: &NaiveTime) -> String {
        if time.nanosecond() == 0 {
            format!("{:02}:{:02}:{:02}", 
                time.hour(),
                time.minute(),
                time.second()
            )
        } else {
            let millis = time.nanosecond() / 1_000_000;
            format!("{:02}:{:02}:{:02}.{:03}", 
                time.hour(),
                time.minute(),
                time.second(),
                millis
            )
        }
    }
}
