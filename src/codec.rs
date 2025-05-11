use crate::model::FixVersion;
use std::collections::HashMap;
use std::io::{Read, Write};

/// FIX protocol delimiters
pub const SOH: u8 = 0x01; // Start of heading (often replaced with | in human-readable formats)
pub const EQUALS: u8 = b'=';
pub const SOH_DISPLAY: char = '|'; // For human-readable display

/// Errors that can occur during FIX message parsing or generation
#[derive(Debug)]
pub enum FixError {
    IoError(std::io::Error),
    ParseError(String),
    ValidationError(String),
}

impl From<std::io::Error> for FixError {
    fn from(error: std::io::Error) -> Self {
        FixError::IoError(error)
    }
}

/// Represents a raw FIX message as a collection of tag-value pairs
#[derive(Debug, Clone)]
pub struct RawFixMessage {
    pub fields: HashMap<u32, String>,
}

impl RawFixMessage {
    /// Create a new empty FIX message
    pub fn new() -> Self {
        RawFixMessage {
            fields: HashMap::new(),
        }
    }

    /// Set a field value
    pub fn set_field(&mut self, tag: u32, value: &str) {
        self.fields.insert(tag, value.to_string());
    }

    /// Get a field value
    pub fn get_field(&self, tag: u32) -> Option<&String> {
        self.fields.get(&tag)
    }

    /// Parse a FIX message from a string
    pub fn parse(message: &str) -> Result<Self, FixError> {
        let mut fields = HashMap::new();

        // Split the message into fields
        for field in message.split(SOH_DISPLAY) {
            if field.is_empty() {
                continue;
            }

            // Split the field into tag and value
            let parts: Vec<&str> = field.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(FixError::ParseError(format!(
                    "Invalid field format: {}",
                    field
                )));
            }

            // Parse the tag
            let tag = parts[0]
                .parse::<u32>()
                .map_err(|_| FixError::ParseError(format!("Invalid tag: {}", parts[0])))?;

            // Store the field
            fields.insert(tag, parts[1].to_string());
        }

        Ok(RawFixMessage { fields })
    }

    /// Read a FIX message from a stream
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, FixError> {
        let mut buffer = Vec::new();
        let mut byte = [0u8; 1];

        // Read until we find the checksum field (tag 10)
        loop {
            match reader.read_exact(&mut byte) {
                Ok(_) => {
                    buffer.push(byte[0]);

                    // Check if we've reached the end of the checksum field
                    if buffer.len() >= 7 && // Minimum "10=xxx<SOH>" is 7 bytes
                        buffer[buffer.len() - 1] == SOH &&
                        buffer[buffer.len() - 7..buffer.len() - 1].starts_with(b"10=")
                    {
                        break;
                    }
                }
                Err(e) => return Err(FixError::IoError(e)),
            }
        }

        // Convert raw bytes to a string, replacing SOH with pipe for parsing
        let message_str =
            String::from_utf8_lossy(&buffer).replace(SOH as char, &SOH_DISPLAY.to_string());

        Self::parse(&message_str)
    }

    /// Write a FIX message to a stream
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), FixError> {
        // Build the message, excluding the checksum field
        let mut message = Vec::new();

        // Sort the fields by tag for deterministic ordering
        let mut tags: Vec<&u32> = self.fields.keys().collect();
        tags.sort();

        for &tag in &tags {
            // Skip the checksum field as we'll calculate it at the end
            if *tag == 10 {
                continue;
            }

            let value = &self.fields[tag];

            // Write tag=value<SOH>
            message.extend_from_slice(tag.to_string().as_bytes());
            message.push(EQUALS);
            message.extend_from_slice(value.as_bytes());
            message.push(SOH);
        }

        // Calculate the checksum
        let checksum = message.iter().map(|&b| b as u32).sum::<u32>() % 256;

        // Write the checksum field
        message.extend_from_slice(b"10=");
        message.extend_from_slice(format!("{:03}", checksum).as_bytes());
        message.push(SOH);

        // Write the complete message
        writer.write_all(&message)?;

        Ok(())
    }

    /// Validate the message against a FIX version
    pub fn validate(&self, version: &FixVersion) -> Result<(), FixError> {
        // Check required fields
        let msg_type = self
            .get_field(35)
            .ok_or_else(|| FixError::ValidationError("Missing MsgType(35) field".to_string()))?;

        let message = version.find_message_by_type(msg_type).ok_or_else(|| {
            FixError::ValidationError(format!("Unknown message type: {}", msg_type))
        })?;

        // Check header required fields
        for field_ref in &version.header.fields {
            if field_ref.required {
                let field = version.find_field_by_name(&field_ref.name).ok_or_else(|| {
                    FixError::ValidationError(format!("Unknown field: {}", field_ref.name))
                })?;

                if !self.fields.contains_key(field.0) {
                    return Err(FixError::ValidationError(format!(
                        "Missing required header field: {} ({})",
                        field_ref.name, field.0
                    )));
                }
            }
        }

        // Check message required fields
        for field_ref in &message.fields {
            if field_ref.required {
                let field = version.find_field_by_name(&field_ref.name).ok_or_else(|| {
                    FixError::ValidationError(format!("Unknown field: {}", field_ref.name))
                })?;

                if !self.fields.contains_key(field.0) {
                    return Err(FixError::ValidationError(format!(
                        "Missing required message field: {} ({})",
                        field_ref.name, field.0
                    )));
                }
            }
        }

        // Check trailer required fields
        for field_ref in &version.trailer.fields {
            if field_ref.required {
                let field = version.find_field_by_name(&field_ref.name).ok_or_else(|| {
                    FixError::ValidationError(format!("Unknown field: {}", field_ref.name))
                })?;

                if !self.fields.contains_key(field.0) {
                    return Err(FixError::ValidationError(format!(
                        "Missing required trailer field: {} ({})",
                        field_ref.name, field.0
                    )));
                }
            }
        }

        Ok(())
    }

    /// Format the message as a human-readable string
    pub fn to_string(&self, version: Option<&FixVersion>) -> String {
        let mut result = String::new();

        // Sort the fields by tag
        let mut tags: Vec<&u32> = self.fields.keys().collect();
        tags.sort();

        for &tag in &tags {
            let value = &self.fields[tag];

            // Include field name in output if version is provided
            if let Some(ver) = version {
                let tag_num = *tag;
                if let Some(field) = ver.fields.get(&tag_num) {
                    result.push_str(&format!("{}({})", field.name, tag));
                } else {
                    result.push_str(&tag.to_string());
                }
            } else {
                result.push_str(&tag.to_string());
            }

            result.push_str(&format!("={}{}", value, SOH_DISPLAY));
        }

        result
    }
}

/// FIX date-time format functions
pub mod datetime {
    use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

    /// Parse a FIX UTCTIMESTAMP format (YYYYMMDD-HH:MM:SS or YYYYMMDD-HH:MM:SS.sss)
    pub fn parse_timestamp(s: &str) -> Option<NaiveDateTime> {
        if s.len() < 17 {
            return None;
        }

        let date_part = &s[0..8];
        let time_part = &s[9..];

        let date = parse_date(date_part)?;
        let time = parse_time(time_part)?;

        Some(NaiveDateTime::new(date, time))
    }

    /// Parse a FIX UTCDATE format (YYYYMMDD)
    pub fn parse_date(s: &str) -> Option<NaiveDate> {
        if s.len() != 8 {
            return None;
        }

        let year: i32 = s[0..4].parse().ok()?;
        let month: u32 = s[4..6].parse().ok()?;
        let day: u32 = s[6..8].parse().ok()?;

        NaiveDate::from_ymd_opt(year, month, day)
    }

    /// Parse a FIX UTCTIMEONLY format (HH:MM:SS or HH:MM:SS.sss)
    pub fn parse_time(s: &str) -> Option<NaiveTime> {
        if s.len() < 8 {
            return None;
        }

        let hour: u32 = s[0..2].parse().ok()?;
        let minute: u32 = s[3..5].parse().ok()?;
        let second: u32 = s[6..8].parse().ok()?;

        let mut nanosecond: u32 = 0;
        if s.len() > 9 {
            let fraction = &s[9..];
            let mut ns: u32 = fraction.parse().ok()?;
            // Adjust to nanoseconds based on the number of digits
            for _ in 0..(9 - fraction.len()) {
                ns *= 10;
            }
            nanosecond = ns;
        }

        NaiveTime::from_hms_nano_opt(hour, minute, second, nanosecond)
    }

    /// Format a NaiveDateTime as a FIX UTCTIMESTAMP
    pub fn format_timestamp(dt: &NaiveDateTime) -> String {
        format!("{}-{}", format_date(&dt.date()), format_time(&dt.time()))
    }

    /// Format a NaiveDate as a FIX UTCDATE
    pub fn format_date(date: &NaiveDate) -> String {
        format!("{}{:02}{:02}", date.year(), date.month(), date.day())
    }

    /// Format a NaiveTime as a FIX UTCTIMEONLY
    pub fn format_time(time: &NaiveTime) -> String {
        if time.nanosecond() == 0 {
            format!(
                "{:02}:{:02}:{:02}",
                time.hour(),
                time.minute(),
                time.second()
            )
        } else {
            let millis = time.nanosecond() / 1_000_000;
            format!(
                "{:02}:{:02}:{:02}.{:03}",
                time.hour(),
                time.minute(),
                time.second(),
                millis
            )
        }
    }
}
