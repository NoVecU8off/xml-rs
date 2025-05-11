// Generated FIX types for FIXT.1.1
// DO NOT EDIT - This file was automatically generated

use std::fmt;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug, Clone, PartialEq)]
pub enum FixDataType {
    String(String),
    Int(i64),
    Float(f64),
    Char(char),
    Boolean(bool),
    Data(Vec<u8>),
    UtcTimestamp(NaiveDateTime),
    UtcDate(NaiveDate),
    UtcTimeOnly(NaiveTime),
    LocalMktDate(NaiveDate),
    TzTimeOnly(String),
    TzTimestamp(String),
    MonthYear(String),
    DayOfMonth(u8),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagValue {
    /// EncryptMethod - Field 98
    EncryptMethod(i64),
    /// ResetSeqNumFlag - Field 141
    ResetSeqNumFlag(bool),
    /// SessionRejectReason - Field 373
    SessionRejectReason(i64),
    /// GapFillFlag - Field 123
    GapFillFlag(bool),
    /// TestMessageIndicator - Field 464
    TestMessageIndicator(bool),
    /// PossResend - Field 97
    PossResend(bool),
    /// ApplVerID - Field 1128
    ApplVerID(String),
    /// SessionStatus - Field 1409
    SessionStatus(i64),
    /// PossDupFlag - Field 43
    PossDupFlag(bool),
}

// Tag number constants
pub const TAG_ENCRYPTMETHOD: u32 = 98;
pub const TAG_RESETSEQNUMFLAG: u32 = 141;
pub const TAG_SESSIONREJECTREASON: u32 = 373;
pub const TAG_GAPFILLFLAG: u32 = 123;
pub const TAG_TESTMESSAGEINDICATOR: u32 = 464;
pub const TAG_POSSRESEND: u32 = 97;
pub const TAG_APPLVERID: u32 = 1128;
pub const TAG_SESSIONSTATUS: u32 = 1409;
pub const TAG_POSSDUPFLAG: u32 = 43;

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    /// Reject - '3'
    Reject,
    /// SequenceReset - '4'
    SequenceReset,
    /// ResendRequest - '2'
    ResendRequest,
    /// TestRequest - '1'
    TestRequest,
    /// Logout - '5'
    Logout,
    /// Logon - 'A'
    Logon,
    /// Heartbeat - '0'
    Heartbeat,
    /// Unknown message type
    Unknown(String),
}

impl From<String> for MessageType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "3" => MessageType::Reject,
            "4" => MessageType::SequenceReset,
            "2" => MessageType::ResendRequest,
            "1" => MessageType::TestRequest,
            "5" => MessageType::Logout,
            "A" => MessageType::Logon,
            "0" => MessageType::Heartbeat,
            _ => MessageType::Unknown(s),
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Reject => write!(f, "3"),
            MessageType::SequenceReset => write!(f, "4"),
            MessageType::ResendRequest => write!(f, "2"),
            MessageType::TestRequest => write!(f, "1"),
            MessageType::Logout => write!(f, "5"),
            MessageType::Logon => write!(f, "A"),
            MessageType::Heartbeat => write!(f, "0"),
            MessageType::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FixMessage {
    pub msg_type: MessageType,
    pub fields: std::collections::HashMap<u32, TagValue>,
}

#[derive(Debug, Clone)]
pub struct Reject {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<Reject> for FixMessage {
    fn from(msg: Reject) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::Reject,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SequenceReset {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<SequenceReset> for FixMessage {
    fn from(msg: SequenceReset) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::SequenceReset,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResendRequest {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<ResendRequest> for FixMessage {
    fn from(msg: ResendRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::ResendRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestRequest {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<TestRequest> for FixMessage {
    fn from(msg: TestRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::TestRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Logout {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<Logout> for FixMessage {
    fn from(msg: Logout) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::Logout,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Logon {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<Logon> for FixMessage {
    fn from(msg: Logon) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::Logon,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Heartbeat {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<Heartbeat> for FixMessage {
    fn from(msg: Heartbeat) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::Heartbeat,
            fields,
        }
    }
}

