use crate::generator::FixGenerator;
use crate::model::FixVersion;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// Generate files for the different FIX versions
pub fn generate_fix_files<P: AsRef<Path>>(
    output_dir: P,
    version: &FixVersion,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure output directory exists
    fs::create_dir_all(&output_dir)?;

    // Determine the version name
    let version_str = if version.protocol_type == "FIXT" {
        format!("fixt{}{}", version.major, version.minor)
    } else {
        format!("fix{}{}", version.major, version.minor)
    };

    // Generate a Rust module file for this version
    let module_path = output_dir.as_ref().join(format!("{}.rs", version_str));
    FixGenerator::generate_types(version, &module_path)?;

    // Update the mod.rs file to export the new module
    let mod_file_path = output_dir.as_ref().join("mod.rs");
    update_mod_file(&mod_file_path, &version_str)?;

    Ok(())
}

// Update the mod.rs file to include the new module
fn update_mod_file<P: AsRef<Path>>(
    mod_file_path: P,
    module_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();

    // If file exists, read it first
    if mod_file_path.as_ref().exists() {
        contents = fs::read_to_string(&mod_file_path)?;
    }

    // Check if the module is already included
    let module_line = format!("pub mod {};", module_name);
    if !contents.contains(&module_line) {
        // Add the module declaration
        if !contents.is_empty() && !contents.ends_with("\n") {
            contents.push('\n');
        }
        contents.push_str(&module_line);
        contents.push('\n');

        // Write back the updated content
        fs::write(&mod_file_path, contents)?;
    }

    Ok(())
}

// Generate common utility functions
pub fn generate_common<P: AsRef<Path>>(output_dir: P) -> Result<(), Box<dyn std::error::Error>> {
    let common_path = output_dir.as_ref().join("common.rs");
    let mut file = File::create(common_path)?;

    // Write common utility functions and types
    writeln!(file, "// Common types and utilities for FIX protocol")?;
    writeln!(
        file,
        "// DO NOT EDIT - This file was automatically generated"
    )?;
    writeln!(file, "")?;
    writeln!(file, "use chrono::{{NaiveDate, NaiveDateTime, NaiveTime}};")?;
    writeln!(file, "")?;
    writeln!(
        file,
        "/// Trait for converting between FIX messages and Rust types"
    )?;
    writeln!(file, "#[allow(clippy::wrong_self_convention)]")?;
    writeln!(file, "pub trait FixMessage {{")?;
    writeln!(
        file,
        "    /// Convert the message to a collection of FIX tag-value pairs"
    )?;
    writeln!(
        file,
        "    fn to_fix(&self) -> std::collections::HashMap<u32, String>;"
    )?;
    writeln!(file, "")?;
    writeln!(
        file,
        "    /// Create a message from a collection of FIX tag-value pairs"
    )?;
    writeln!(file, "    fn from_fix(fields: &std::collections::HashMap<u32, String>) -> Result<Self, Box<dyn std::error::Error>>")?;
    writeln!(file, "    where Self: Sized;")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(
        file,
        "/// Helper functions for converting between FIX and Rust types"
    )?;
    writeln!(file, "pub mod convert {{")?;
    writeln!(
        file,
        "    use chrono::{{NaiveDate, NaiveDateTime, NaiveTime}};"
    )?;
    writeln!(file, "")?;
    writeln!(file, "    /// Parse a FIX boolean value (Y/N)")?;
    writeln!(file, "    pub fn parse_bool(value: &str) -> bool {{")?;
    writeln!(file, "        value == \"Y\"")?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;
    writeln!(file, "    /// Format a boolean as a FIX value")?;
    writeln!(file, "    pub fn format_bool(value: bool) -> String {{")?;
    writeln!(
        file,
        "        if value {{ \"Y\" }} else {{ \"N\" }}.to_string()"
    )?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;
    writeln!(
        file,
        "    /// Parse a FIX UTC timestamp (YYYYMMDD-HH:MM:SS or YYYYMMDD-HH:MM:SS.sss)"
    )?;
    writeln!(file, "    pub fn parse_utc_timestamp(value: &str) -> Result<NaiveDateTime, Box<dyn std::error::Error>> {{")?;
    writeln!(file, "        if value.len() < 17 {{")?;
    writeln!(
        file,
        "            return Err(\"Invalid timestamp format\".into());"
    )?;
    writeln!(file, "        }}")?;
    writeln!(file, "")?;
    writeln!(file, "        let date_part = &value[0..8];")?;
    writeln!(file, "        let time_part = &value[9..];")?;
    writeln!(file, "")?;
    writeln!(file, "        let date = parse_utc_date(date_part)?;")?;
    writeln!(file, "        let time = parse_utc_time(time_part)?;")?;
    writeln!(file, "")?;
    writeln!(file, "        Ok(NaiveDateTime::new(date, time))")?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;
    writeln!(file, "    /// Format a UTC timestamp in FIX format")?;
    writeln!(
        file,
        "    pub fn format_utc_timestamp(dt: &NaiveDateTime) -> String {{"
    )?;
    writeln!(file, "        format!(\"{{}}-{{}}\", ")?;
    writeln!(file, "            format_utc_date(&dt.date()),")?;
    writeln!(file, "            format_utc_time(&dt.time())")?;
    writeln!(file, "        )")?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;
    writeln!(file, "    /// Parse a FIX UTC date (YYYYMMDD)")?;
    writeln!(file, "    pub fn parse_utc_date(value: &str) -> Result<NaiveDate, Box<dyn std::error::Error>> {{")?;
    writeln!(file, "        if value.len() != 8 {{")?;
    writeln!(
        file,
        "            return Err(\"Invalid date format\".into());"
    )?;
    writeln!(file, "        }}")?;
    writeln!(file, "")?;
    writeln!(file, "        let year: i32 = value[0..4].parse()?;")?;
    writeln!(file, "        let month: u32 = value[4..6].parse()?;")?;
    writeln!(file, "        let day: u32 = value[6..8].parse()?;")?;
    writeln!(file, "")?;
    writeln!(file, "        NaiveDate::from_ymd_opt(year, month, day)")?;
    writeln!(
        file,
        "            .ok_or_else(|| \"Invalid date components\".into())"
    )?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;
    writeln!(file, "    /// Format a UTC date in FIX format")?;
    writeln!(
        file,
        "    pub fn format_utc_date(date: &NaiveDate) -> String {{"
    )?;
    writeln!(file, "        format!(\"{{}}{{:02}}{{:02}}\", ")?;
    writeln!(file, "            date.year(),")?;
    writeln!(file, "            date.month(),")?;
    writeln!(file, "            date.day()")?;
    writeln!(file, "        )")?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;
    writeln!(
        file,
        "    /// Parse a FIX UTC time (HH:MM:SS or HH:MM:SS.sss)"
    )?;
    writeln!(file, "    pub fn parse_utc_time(value: &str) -> Result<NaiveTime, Box<dyn std::error::Error>> {{")?;
    writeln!(file, "        if value.len() < 8 {{")?;
    writeln!(
        file,
        "            return Err(\"Invalid time format\".into());"
    )?;
    writeln!(file, "        }}")?;
    writeln!(file, "")?;
    writeln!(file, "        let hour: u32 = value[0..2].parse()?;")?;
    writeln!(file, "        let minute: u32 = value[3..5].parse()?;")?;
    writeln!(file, "        let second: u32 = value[6..8].parse()?;")?;
    writeln!(file, "")?;
    writeln!(file, "        let mut nanosecond: u32 = 0;")?;
    writeln!(file, "        if value.len() > 9 {{")?;
    writeln!(file, "            let fraction = &value[9..];")?;
    writeln!(file, "            let mut ns: u32 = fraction.parse()?;")?;
    writeln!(
        file,
        "            // Adjust to nanoseconds based on the number of digits"
    )?;
    writeln!(file, "            for _ in 0..(9 - fraction.len()) {{")?;
    writeln!(file, "                ns *= 10;")?;
    writeln!(file, "            }}")?;
    writeln!(file, "            nanosecond = ns;")?;
    writeln!(file, "        }}")?;
    writeln!(file, "")?;
    writeln!(
        file,
        "        NaiveTime::from_hms_nano_opt(hour, minute, second, nanosecond)"
    )?;
    writeln!(
        file,
        "            .ok_or_else(|| \"Invalid time components\".into())"
    )?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;
    writeln!(file, "    /// Format a UTC time in FIX format")?;
    writeln!(
        file,
        "    pub fn format_utc_time(time: &NaiveTime) -> String {{"
    )?;
    writeln!(file, "        if time.nanosecond() == 0 {{")?;
    writeln!(file, "            format!(\"{{:02}}:{{:02}}:{{:02}}\", ")?;
    writeln!(file, "                time.hour(),")?;
    writeln!(file, "                time.minute(),")?;
    writeln!(file, "                time.second()")?;
    writeln!(file, "            )")?;
    writeln!(file, "        }} else {{")?;
    writeln!(
        file,
        "            let millis = time.nanosecond() / 1_000_000;"
    )?;
    writeln!(
        file,
        "            format!(\"{{:02}}:{{:02}}:{{:02}}.{{:03}}\", "
    )?;
    writeln!(file, "                time.hour(),")?;
    writeln!(file, "                time.minute(),")?;
    writeln!(file, "                time.second(),")?;
    writeln!(file, "                millis")?;
    writeln!(file, "            )")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    // Update the mod.rs file to include common
    let mod_file_path = output_dir.as_ref().join("mod.rs");
    update_mod_file(&mod_file_path, "common")?;

    Ok(())
}
