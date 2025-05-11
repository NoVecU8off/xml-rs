use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::Reader;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::model::{
    Component, ComponentRef, Field, FieldRef, FieldValue, FixVersion, Group, Message,
};

pub struct FixParser;

impl FixParser {
    /// Parse a FIX specification XML file
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<FixVersion, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Self::parse(&contents)
    }

    /// Parse a FIX specification XML string
    pub fn parse(xml: &str) -> Result<FixVersion, Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut fix_version = None;
        let mut in_header = false;
        let mut in_trailer = false;
        let mut in_messages = false;
        let mut in_components = false;
        let mut in_fields = false;

        // First pass: get FIX version information
        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => match e.name() {
                    QName(b"fix") => {
                        fix_version = Some(Self::parse_fix_attributes(e)?);
                    }
                    QName(b"header") => in_header = true,
                    QName(b"trailer") => in_trailer = true,
                    QName(b"messages") => in_messages = true,
                    QName(b"components") => in_components = true,
                    QName(b"fields") => in_fields = true,
                    QName(b"field") if in_header => {
                        if let Some(ref mut fix) = fix_version {
                            fix.header.fields.push(Self::parse_field_ref(e)?);
                        }
                    }
                    QName(b"field") if in_trailer => {
                        if let Some(ref mut fix) = fix_version {
                            fix.trailer.fields.push(Self::parse_field_ref(e)?);
                        }
                    }
                    QName(b"group") if in_header => {
                        if let Some(ref mut fix) = fix_version {
                            fix.header.groups.push(Self::parse_group(e, &mut reader)?);
                        }
                    }
                    QName(b"group") if in_trailer => {
                        if let Some(ref mut fix) = fix_version {
                            fix.trailer.groups.push(Self::parse_group(e, &mut reader)?);
                        }
                    }
                    QName(b"message") if in_messages => {
                        if let Some(ref mut fix) = fix_version {
                            let message = Self::parse_message(e, &mut reader)?;
                            fix.messages.insert(message.name.clone(), message);
                        }
                    }
                    QName(b"component") if in_components => {
                        if let Some(ref mut fix) = fix_version {
                            let component = Self::parse_component(e, &mut reader)?;
                            fix.components.insert(component.name.clone(), component);
                        }
                    }
                    QName(b"field") if in_fields => {
                        if let Some(ref mut fix) = fix_version {
                            let field = Self::parse_field(e, &mut reader)?;
                            fix.fields.insert(field.number, field);
                        }
                    }
                    _ => {}
                },
                Ok(Event::End(ref e)) => match e.name() {
                    QName(b"header") => in_header = false,
                    QName(b"trailer") => in_trailer = false,
                    QName(b"messages") => in_messages = false,
                    QName(b"components") => in_components = false,
                    QName(b"fields") => in_fields = false,
                    _ => {}
                },
                Ok(Event::Eof) => break,
                Err(e) => return Err(Box::new(e)),
                _ => {}
            }
        }

        match fix_version {
            Some(fix) => {
                println!("\n{fix:?}\n");
                Ok(fix)
            }
            None => Err("No FIX version found in XML".into()),
        }
    }

    /// Parse the FIX version attributes from the main <fix> element
    fn parse_fix_attributes(e: &BytesStart) -> Result<FixVersion, Box<dyn std::error::Error>> {
        let mut protocol_type = "FIX".to_string();
        let mut major = 0;
        let mut minor = 0;
        let mut servicepack = 0;

        for attr in e.attributes() {
            let attr = attr?;
            let value = attr.value;
            let value_str = std::str::from_utf8(&value)?;

            match attr.key {
                QName(b"type") => protocol_type = value_str.to_string(),
                QName(b"major") => major = value_str.parse()?,
                QName(b"minor") => minor = value_str.parse()?,
                QName(b"servicepack") => servicepack = value_str.parse()?,
                _ => {}
            }
        }

        Ok(FixVersion::new(&protocol_type, major, minor, servicepack))
    }

    /// Parse a field reference (used in headers, trailers, messages)
    fn parse_field_ref(e: &BytesStart) -> Result<FieldRef, Box<dyn std::error::Error>> {
        let mut name = String::new();
        let mut required = false;

        for attr in e.attributes() {
            let attr = attr?;
            let value = attr.value;
            let value_str = std::str::from_utf8(&value)?;

            match attr.key {
                QName(b"name") => name = value_str.to_string(),
                QName(b"required") => required = value_str == "Y",
                _ => {}
            }
        }

        Ok(FieldRef { name, required })
    }

    /// Parse a component reference
    fn parse_component_ref(e: &BytesStart) -> Result<ComponentRef, Box<dyn std::error::Error>> {
        let mut name = String::new();
        let mut required = false;

        for attr in e.attributes() {
            let attr = attr?;
            let value = attr.value;
            let value_str = std::str::from_utf8(&value)?;

            match attr.key {
                QName(b"name") => name = value_str.to_string(),
                QName(b"required") => required = value_str == "Y",
                _ => {}
            }
        }

        Ok(ComponentRef { name, required })
    }

    /// Parse a repeating group
    fn parse_group(
        e: &BytesStart,
        reader: &mut Reader<&[u8]>,
    ) -> Result<Group, Box<dyn std::error::Error>> {
        let mut name = String::new();
        let mut required = false;
        let mut fields = Vec::new();
        let mut groups = Vec::new();
        let mut components = Vec::new();

        for attr in e.attributes() {
            let attr = attr?;
            let value = attr.value;
            let value_str = std::str::from_utf8(&value)?;

            match attr.key {
                QName(b"name") => name = value_str.to_string(),
                QName(b"required") => required = value_str == "Y",
                _ => {}
            }
        }

        // Parse nested elements
        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => match e.name() {
                    QName(b"field") => {
                        fields.push(Self::parse_field_ref(e)?);
                    }
                    QName(b"group") => {
                        groups.push(Self::parse_group(e, reader)?);
                    }
                    QName(b"component") => {
                        components.push(Self::parse_component_ref(e)?);
                    }
                    _ => {}
                },
                Ok(Event::End(ref e)) if e.name() == QName(b"group") => break,
                Ok(Event::Eof) => {
                    return Err("Unexpected EOF while parsing group".into());
                }
                Err(e) => return Err(Box::new(e)),
                _ => {}
            }
        }

        Ok(Group {
            name,
            required,
            fields,
            groups,
            components,
        })
    }

    /// Parse a message
    fn parse_message(
        e: &BytesStart,
        reader: &mut Reader<&[u8]>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let mut name = String::new();
        let mut msg_type = String::new();
        let mut msg_cat = String::new();
        let mut fields = Vec::new();
        let mut groups = Vec::new();
        let mut components = Vec::new();

        for attr in e.attributes() {
            let attr = attr?;
            let value = attr.value;
            let value_str = std::str::from_utf8(&value)?;

            match attr.key {
                QName(b"name") => name = value_str.to_string(),
                QName(b"msgtype") => msg_type = value_str.to_string(),
                QName(b"msgcat") => msg_cat = value_str.to_string(),
                _ => {}
            }
        }

        // Parse nested elements
        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => match e.name() {
                    QName(b"field") => {
                        fields.push(Self::parse_field_ref(e)?);
                    }
                    QName(b"group") => {
                        groups.push(Self::parse_group(e, reader)?);
                    }
                    QName(b"component") => {
                        components.push(Self::parse_component_ref(e)?);
                    }
                    _ => {}
                },
                Ok(Event::End(ref e)) if e.name() == QName(b"message") => break,
                Ok(Event::Eof) => {
                    return Err("Unexpected EOF while parsing message".into());
                }
                Err(e) => return Err(Box::new(e)),
                _ => {}
            }
        }

        Ok(Message {
            name,
            msg_type,
            msg_cat,
            fields,
            groups,
            components,
        })
    }

    /// Parse a component
    fn parse_component(
        e: &BytesStart,
        reader: &mut Reader<&[u8]>,
    ) -> Result<Component, Box<dyn std::error::Error>> {
        let mut name = String::new();
        let mut fields = Vec::new();
        let mut groups = Vec::new();
        let mut components = Vec::new();

        for attr in e.attributes() {
            let attr = attr?;
            let value = attr.value;
            let value_str = std::str::from_utf8(&value)?;

            match attr.key {
                QName(b"name") => name = value_str.to_string(),
                _ => {}
            }
        }

        // Parse nested elements
        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => match e.name() {
                    QName(b"field") => {
                        fields.push(Self::parse_field_ref(e)?);
                    }
                    QName(b"group") => {
                        groups.push(Self::parse_group(e, reader)?);
                    }
                    QName(b"component") => {
                        components.push(Self::parse_component_ref(e)?);
                    }
                    _ => {}
                },
                Ok(Event::End(ref e)) if e.name() == QName(b"component") => break,
                Ok(Event::Eof) => {
                    return Err("Unexpected EOF while parsing component".into());
                }
                Err(e) => return Err(Box::new(e)),
                _ => {}
            }
        }

        Ok(Component {
            name,
            fields,
            groups,
            components,
        })
    }

    /// Parse a field
    fn parse_field(
        e: &BytesStart,
        reader: &mut Reader<&[u8]>,
    ) -> Result<Field, Box<dyn std::error::Error>> {
        let mut number = 0;
        let mut name = String::new();
        let mut field_type = String::new();
        let mut values = Vec::new();

        for attr in e.attributes() {
            let attr = attr?;
            let value = attr.value;
            let value_str = std::str::from_utf8(&value)?;

            match attr.key {
                QName(b"number") => number = value_str.parse()?,
                QName(b"name") => name = value_str.to_string(),
                QName(b"type") => field_type = value_str.to_string(),
                _ => {}
            }
        }

        // Parse field values
        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    if e.name() == QName(b"value") {
                        let mut enum_value = String::new();
                        let mut description = String::new();

                        for attr in e.attributes() {
                            let attr = attr?;
                            let value = attr.value;
                            let value_str = std::str::from_utf8(&value)?;

                            match attr.key {
                                QName(b"enum") => enum_value = value_str.to_string(),
                                QName(b"description") => description = value_str.to_string(),
                                _ => {}
                            }
                        }

                        values.push(FieldValue {
                            enum_value,
                            description,
                        });
                    }
                }
                Ok(Event::End(ref e)) if e.name() == QName(b"field") => break,
                Ok(Event::Eof) => {
                    return Err("Unexpected EOF while parsing field".into());
                }
                Err(e) => return Err(Box::new(e)),
                _ => {}
            }
        }

        Ok(Field {
            number,
            name,
            field_type,
            values,
        })
    }
}
