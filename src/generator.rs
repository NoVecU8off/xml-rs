use crate::model::{Field, FixVersion};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct FixGenerator;

impl FixGenerator {
    pub fn generate_types<P: AsRef<Path>>(
        version: &FixVersion,
        output_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(output_path)?;

        // Write module header
        writeln!(
            file,
            "// Generated FIX types for {}",
            version.version_string()
        )?;
        writeln!(
            file,
            "// DO NOT EDIT - This file was automatically generated"
        )?;
        writeln!(file, "")?;
        writeln!(file, "use std::fmt;")?;
        writeln!(file, "use chrono::{{NaiveDate, NaiveDateTime, NaiveTime}};")?;
        writeln!(file, "")?;

        // Generate field enum types
        Self::generate_field_enums(version, &mut file)?;

        // Generate message structs
        Self::generate_message_structs(version, &mut file)?;

        Ok(())
    }

    fn generate_field_enums(
        version: &FixVersion,
        file: &mut File,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get all fields grouped by type
        let mut types_map: HashMap<String, Vec<&Field>> = HashMap::new();

        for field in version.fields.values() {
            types_map
                .entry(field.field_type.clone())
                .or_insert_with(Vec::new)
                .push(field);
        }

        // Generate common data types
        writeln!(file, "#[derive(Debug, Clone, PartialEq)]")?;
        writeln!(file, "pub enum FixDataType {{")?;
        writeln!(file, "    String(String),")?;
        writeln!(file, "    Int(i64),")?;
        writeln!(file, "    Float(f64),")?;
        writeln!(file, "    Char(char),")?;
        writeln!(file, "    Boolean(bool),")?;
        writeln!(file, "    Data(Vec<u8>),")?;
        writeln!(file, "    UtcTimestamp(NaiveDateTime),")?;
        writeln!(file, "    UtcDate(NaiveDate),")?;
        writeln!(file, "    UtcTimeOnly(NaiveTime),")?;
        writeln!(file, "    LocalMktDate(NaiveDate),")?;
        writeln!(file, "    TzTimeOnly(String),")?;
        writeln!(file, "    TzTimestamp(String),")?;
        writeln!(file, "    MonthYear(String),")?;
        writeln!(file, "    DayOfMonth(u8),")?;
        writeln!(file, "}}")?;
        writeln!(file, "")?;

        // Generate enums for fields with enum values
        for field in version.fields.values() {
            if !field.values.is_empty() {
                let enum_name = fix_field_to_enum_name(&field.name);

                writeln!(file, "#[derive(Debug, Clone, PartialEq)]")?;
                writeln!(file, "pub enum {} {{", enum_name)?;

                // Add enum variants
                for value in &field.values {
                    let variant_name = fix_value_to_enum_variant(&value.description);
                    writeln!(file, "    /// {}", value.description)?;
                    writeln!(file, "    {},", variant_name)?;
                }

                // Add Unknown variant for backwards compatibility
                writeln!(file, "    /// Unknown value")?;
                writeln!(file, "    Unknown(String),")?;

                writeln!(file, "}}")?;
                writeln!(file, "")?;

                // Implement From<String> for each enum
                writeln!(file, "impl From<String> for {} {{", enum_name)?;
                writeln!(file, "    fn from(s: String) -> Self {{")?;
                writeln!(file, "        match s.as_str() {{")?;

                for value in &field.values {
                    let variant_name = fix_value_to_enum_variant(&value.description);
                    writeln!(
                        file,
                        "            \"{}\" => {}::{},",
                        value.enum_value, enum_name, variant_name
                    )?;
                }

                writeln!(file, "            _ => {}::Unknown(s),", enum_name)?;
                writeln!(file, "        }}")?;
                writeln!(file, "    }}")?;
                writeln!(file, "}}")?;
                writeln!(file, "")?;

                // Implement Display for each enum
                writeln!(file, "impl fmt::Display for {} {{", enum_name)?;
                writeln!(
                    file,
                    "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
                )?;
                writeln!(file, "        match self {{")?;

                for value in &field.values {
                    let variant_name = fix_value_to_enum_variant(&value.description);
                    writeln!(
                        file,
                        "            {}::{} => write!(f, \"{}\")",
                        enum_name, variant_name, value.enum_value
                    )?;
                }

                writeln!(
                    file,
                    "            {}::Unknown(s) => write!(f, \"{{}}\")",
                    enum_name
                )?;
                writeln!(file, "        }}")?;
                writeln!(file, "    }}")?;
                writeln!(file, "}}")?;
                writeln!(file, "")?;
            }
        }

        // Generate TagValue enum with all possible field types
        writeln!(file, "#[derive(Debug, Clone, PartialEq)]")?;
        writeln!(file, "pub enum TagValue {{")?;

        // Add variants for each field
        for field in version.fields.values() {
            let field_name = capitalize_first_letter(&field.name);

            if !field.values.is_empty() {
                let enum_name = fix_field_to_enum_name(&field.name);
                writeln!(file, "    /// {} - Field {}", field.name, field.number)?;
                writeln!(file, "    {}({}),", field_name, enum_name)?;
            } else {
                let rust_type = fix_type_to_rust_type(&field.field_type);
                writeln!(file, "    /// {} - Field {}", field.name, field.number)?;
                writeln!(file, "    {}({}),", field_name, rust_type)?;
            }
        }

        writeln!(file, "}}")?;
        writeln!(file, "")?;

        // Generate tag number constants
        writeln!(file, "// Tag number constants")?;
        for field in version.fields.values() {
            let constant_name = field.name.to_uppercase();
            writeln!(
                file,
                "pub const TAG_{}: u32 = {};",
                constant_name, field.number
            )?;
        }
        writeln!(file, "")?;

        Ok(())
    }

    fn generate_message_structs(
        version: &FixVersion,
        file: &mut File,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Generate message types enum
        writeln!(file, "#[derive(Debug, Clone, PartialEq)]")?;
        writeln!(file, "pub enum MessageType {{")?;

        for message in version.messages.values() {
            let msg_variant = message.name.replace(" ", "");
            writeln!(file, "    /// {} - '{}'", message.name, message.msg_type)?;
            writeln!(file, "    {},", msg_variant)?;
        }

        writeln!(file, "    /// Unknown message type")?;
        writeln!(file, "    Unknown(String),")?;
        writeln!(file, "}}")?;
        writeln!(file, "")?;

        // Implement From<String> for MessageType
        writeln!(file, "impl From<String> for MessageType {{")?;
        writeln!(file, "    fn from(s: String) -> Self {{")?;
        writeln!(file, "        match s.as_str() {{")?;

        for message in version.messages.values() {
            let msg_variant = message.name.replace(" ", "");
            writeln!(
                file,
                "            \"{}\" => MessageType::{},",
                message.msg_type, msg_variant
            )?;
        }

        writeln!(file, "            _ => MessageType::Unknown(s),")?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file, "")?;

        // Implement Display for MessageType
        writeln!(file, "impl fmt::Display for MessageType {{")?;
        writeln!(
            file,
            "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
        )?;
        writeln!(file, "        match self {{")?;

        for message in version.messages.values() {
            let msg_variant = message.name.replace(" ", "");
            writeln!(
                file,
                "            MessageType::{} => write!(f, \"{}\")",
                msg_variant, message.msg_type
            )?;
        }

        writeln!(
            file,
            "            MessageType::Unknown(s) => write!(f, \"{{}}\")"
        )?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file, "")?;

        // Generate base Message struct
        writeln!(file, "#[derive(Debug, Clone)]")?;
        writeln!(file, "pub struct FixMessage {{")?;
        writeln!(file, "    pub msg_type: MessageType,")?;
        writeln!(
            file,
            "    pub fields: std::collections::HashMap<u32, TagValue>,"
        )?;
        writeln!(file, "}}")?;
        writeln!(file, "")?;

        // Generate specific message structs
        for message in version.messages.values() {
            let struct_name = message.name.replace(" ", "");

            writeln!(file, "#[derive(Debug, Clone)]")?;
            writeln!(file, "pub struct {} {{", struct_name)?;

            // Add common header fields
            writeln!(file, "    // Header fields")?;
            for field_ref in &version.header.fields {
                let field = version.find_field_by_name(&field_ref.name);
                if let Some((_field_num, field_info)) = field {
                    let field_type = if !field_info.values.is_empty() {
                        fix_field_to_enum_name(&field_info.name)
                    } else {
                        fix_type_to_rust_type(&field_info.field_type)
                    };

                    if field_ref.required {
                        writeln!(
                            file,
                            "    pub {}: {},",
                            snake_case(&field_info.name),
                            field_type
                        )?;
                    } else {
                        writeln!(
                            file,
                            "    pub {}: Option<{}>,",
                            snake_case(&field_info.name),
                            field_type
                        )?;
                    }
                }
            }

            // Add message-specific fields
            writeln!(file, "    // Message fields")?;
            for field_ref in &message.fields {
                let field = version.find_field_by_name(&field_ref.name);
                if let Some((_field_num, field_info)) = field {
                    let field_type = if !field_info.values.is_empty() {
                        fix_field_to_enum_name(&field_info.name)
                    } else {
                        fix_type_to_rust_type(&field_info.field_type)
                    };

                    if field_ref.required {
                        writeln!(
                            file,
                            "    pub {}: {},",
                            snake_case(&field_info.name),
                            field_type
                        )?;
                    } else {
                        writeln!(
                            file,
                            "    pub {}: Option<{}>,",
                            snake_case(&field_info.name),
                            field_type
                        )?;
                    }
                }
            }

            // Add trailer fields
            writeln!(file, "    // Trailer fields")?;
            for field_ref in &version.trailer.fields {
                let field = version.find_field_by_name(&field_ref.name);
                if let Some((_field_num, field_info)) = field {
                    let field_type = if !field_info.values.is_empty() {
                        fix_field_to_enum_name(&field_info.name)
                    } else {
                        fix_type_to_rust_type(&field_info.field_type)
                    };

                    if field_ref.required {
                        writeln!(
                            file,
                            "    pub {}: {},",
                            snake_case(&field_info.name),
                            field_type
                        )?;
                    } else {
                        writeln!(
                            file,
                            "    pub {}: Option<{}>,",
                            snake_case(&field_info.name),
                            field_type
                        )?;
                    }
                }
            }

            writeln!(file, "}}")?;
            writeln!(file, "")?;

            // Implement conversion to FixMessage
            writeln!(file, "impl From<{}> for FixMessage {{", struct_name)?;
            writeln!(file, "    fn from(msg: {}) -> Self {{", struct_name)?;
            writeln!(
                file,
                "        let mut fields = std::collections::HashMap::new();"
            )?;

            // Add header fields
            for field_ref in &version.header.fields {
                let field = version.find_field_by_name(&field_ref.name);
                if let Some((_field_num, field_info)) = field {
                    let field_snake = snake_case(&field_info.name);
                    let field_pascal = capitalize_first_letter(&field_info.name);

                    if field_ref.required {
                        writeln!(
                            file,
                            "        fields.insert(TAG_{}, TagValue::{}(msg.{}));",
                            field_info.name.to_uppercase(),
                            field_pascal,
                            field_snake
                        )?;
                    } else {
                        writeln!(file, "        if let Some(value) = msg.{} {{", field_snake)?;
                        writeln!(
                            file,
                            "            fields.insert(TAG_{}, TagValue::{}(value));",
                            field_info.name.to_uppercase(),
                            field_pascal
                        )?;
                        writeln!(file, "        }}")?;
                    }
                }
            }

            // Add message fields
            for field_ref in &message.fields {
                let field = version.find_field_by_name(&field_ref.name);
                if let Some((_field_num, field_info)) = field {
                    let field_snake = snake_case(&field_info.name);
                    let field_pascal = capitalize_first_letter(&field_info.name);

                    if field_ref.required {
                        writeln!(
                            file,
                            "        fields.insert(TAG_{}, TagValue::{}(msg.{}));",
                            field_info.name.to_uppercase(),
                            field_pascal,
                            field_snake
                        )?;
                    } else {
                        writeln!(file, "        if let Some(value) = msg.{} {{", field_snake)?;
                        writeln!(
                            file,
                            "            fields.insert(TAG_{}, TagValue::{}(value));",
                            field_info.name.to_uppercase(),
                            field_pascal
                        )?;
                        writeln!(file, "        }}")?;
                    }
                }
            }

            // Add trailer fields
            for field_ref in &version.trailer.fields {
                let field = version.find_field_by_name(&field_ref.name);
                if let Some((_field_num, field_info)) = field {
                    let field_snake = snake_case(&field_info.name);
                    let field_pascal = capitalize_first_letter(&field_info.name);

                    if field_ref.required {
                        writeln!(
                            file,
                            "        fields.insert(TAG_{}, TagValue::{}(msg.{}));",
                            field_info.name.to_uppercase(),
                            field_pascal,
                            field_snake
                        )?;
                    } else {
                        writeln!(file, "        if let Some(value) = msg.{} {{", field_snake)?;
                        writeln!(
                            file,
                            "            fields.insert(TAG_{}, TagValue::{}(value));",
                            field_info.name.to_uppercase(),
                            field_pascal
                        )?;
                        writeln!(file, "        }}")?;
                    }
                }
            }

            writeln!(file, "        FixMessage {{")?;
            writeln!(file, "            msg_type: MessageType::{},", struct_name)?;
            writeln!(file, "            fields,")?;
            writeln!(file, "        }}")?;
            writeln!(file, "    }}")?;
            writeln!(file, "}}")?;
            writeln!(file, "")?;
        }

        Ok(())
    }
}

/// Convert a FIX field name to a Rust enum name
fn fix_field_to_enum_name(field_name: &str) -> String {
    format!("{}Enum", field_name)
}

/// Convert a FIX value description to a Rust enum variant
fn fix_value_to_enum_variant(desc: &str) -> String {
    let cleaned = desc
        .replace(" ", "")
        .replace("-", "_")
        .replace(".", "_")
        .replace("/", "_")
        .replace("&", "And")
        .replace("(", "")
        .replace(")", "");

    capitalize_first_letter(&cleaned)
}

/// Convert a FIX type to a Rust type
fn fix_type_to_rust_type(fix_type: &str) -> String {
    match fix_type {
        "STRING" | "CHAR" => "String".to_string(),
        "INT" | "SEQNUM" | "LENGTH" | "NUMINGROUP" => "i64".to_string(),
        "FLOAT" | "PRICE" | "QTY" | "AMT" => "f64".to_string(),
        "BOOLEAN" => "bool".to_string(),
        "DATA" => "Vec<u8>".to_string(),
        "UTCTIMESTAMP" => "NaiveDateTime".to_string(),
        "UTCDATE" | "LOCALMKTDATE" => "NaiveDate".to_string(),
        "UTCTIMEONLY" => "NaiveTime".to_string(),
        "MONTHYEAR" => "String".to_string(),
        "DAYOFMONTH" => "u8".to_string(),
        _ => "String".to_string(), // Default to String for unknown types
    }
}

/// Convert string to snake_case
fn snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_char_is_lowercase = false;

    for c in s.chars() {
        if c.is_uppercase() {
            if prev_char_is_lowercase {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_char_is_lowercase = false;
        } else {
            result.push(c);
            prev_char_is_lowercase = c.is_lowercase();
        }
    }

    result
}

/// Capitalize first letter of a string
fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
