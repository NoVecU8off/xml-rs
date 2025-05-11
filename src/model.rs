use std::collections::{HashMap, HashSet};

/// Represents a FIX protocol version
#[derive(Debug, Clone)]
pub struct FixVersion {
    pub protocol_type: String,
    pub major: u8,
    pub minor: u8,
    pub servicepack: u8,
    pub header: MessageSection,
    pub trailer: MessageSection,
    pub messages: HashMap<String, Message>,
    pub components: HashMap<String, Component>,
    pub fields: HashMap<u32, Field>,
}

/// Represents a section in a FIX message (like header or trailer)
#[derive(Debug, Clone)]
pub struct MessageSection {
    pub fields: Vec<FieldRef>,
    pub groups: Vec<Group>,
    pub components: Vec<ComponentRef>,
}

/// Represents a FIX message
#[derive(Debug, Clone)]
pub struct Message {
    pub name: String,
    pub msg_type: String,
    pub msg_cat: String,
    pub fields: Vec<FieldRef>,
    pub groups: Vec<Group>,
    pub components: Vec<ComponentRef>,
}

/// Represents a FIX component
#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub fields: Vec<FieldRef>,
    pub groups: Vec<Group>,
    pub components: Vec<ComponentRef>,
}

/// Reference to a component
#[derive(Debug, Clone)]
pub struct ComponentRef {
    pub name: String,
    pub required: bool,
}

/// Reference to a field
#[derive(Debug, Clone)]
pub struct FieldRef {
    pub name: String,
    pub required: bool,
}

/// Represents a repeating group
#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub required: bool,
    pub fields: Vec<FieldRef>,
    pub groups: Vec<Group>,
    pub components: Vec<ComponentRef>,
}

/// Represents a FIX field
#[derive(Debug, Clone)]
pub struct Field {
    pub number: u32,
    pub name: String,
    pub field_type: String,
    pub values: Vec<FieldValue>,
}

/// Represents a valid value for a FIX field
#[derive(Debug, Clone)]
pub struct FieldValue {
    pub enum_value: String,
    pub description: String,
}

impl FixVersion {
    pub fn new(protocol_type: &str, major: u8, minor: u8, servicepack: u8) -> Self {
        FixVersion {
            protocol_type: protocol_type.to_string(),
            major,
            minor,
            servicepack,
            header: MessageSection {
                fields: Vec::new(),
                groups: Vec::new(),
                components: Vec::new(),
            },
            trailer: MessageSection {
                fields: Vec::new(),
                groups: Vec::new(),
                components: Vec::new(),
            },
            messages: HashMap::new(),
            components: HashMap::new(),
            fields: HashMap::new(),
        }
    }

    /// Get the FIX version as a string (e.g., "FIX.4.0")
    pub fn version_string(&self) -> String {
        if self.protocol_type == "FIXT" {
            format!("FIXT.{}.{}", self.major, self.minor)
        } else {
            format!("FIX.{}.{}", self.major, self.minor)
        }
    }

    /// Check if a field is present in the header
    pub fn is_header_field(&self, name: &str) -> bool {
        self.header.fields.iter().any(|f| f.name == name)
    }

    /// Check if a field is present in the trailer
    pub fn is_trailer_field(&self, name: &str) -> bool {
        self.trailer.fields.iter().any(|f| f.name == name)
    }

    /// Get all field numbers used in this FIX version
    pub fn get_all_field_numbers(&self) -> HashSet<u32> {
        self.fields.keys().cloned().collect()
    }

    /// Find a field by its name
    pub fn find_field_by_name(&self, name: &str) -> Option<(&u32, &Field)> {
        self.fields.iter().find(|(_, field)| field.name == name)
    }

    /// Find a message by its type
    pub fn find_message_by_type(&self, msg_type: &str) -> Option<&Message> {
        self.messages.values().find(|msg| msg.msg_type == msg_type)
    }
}
