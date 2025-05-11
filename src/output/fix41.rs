// Generated FIX types for FIX.4.1
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
    /// AdvTransType - Field 5
    AdvTransType(String),
    /// SecurityType - Field 167
    SecurityType(String),
    /// IOIQltyInd - Field 25
    IOIQltyInd(String),
    /// AllocStatus - Field 87
    AllocStatus(i64),
    /// IOIShares - Field 27
    IOIShares(String),
    /// PossDupFlag - Field 43
    PossDupFlag(String),
    /// SettlmntTyp - Field 63
    SettlmntTyp(String),
    /// ProcessCode - Field 81
    ProcessCode(String),
    /// EmailType - Field 94
    EmailType(String),
    /// Rule80A - Field 47
    Rule80A(String),
    /// IOIQualifier - Field 104
    IOIQualifier(String),
    /// GapFillFlag - Field 123
    GapFillFlag(String),
    /// DKReason - Field 127
    DKReason(String),
    /// CommType - Field 13
    CommType(String),
    /// IOINaturalFlag - Field 130
    IOINaturalFlag(String),
    /// SettlInstTransType - Field 163
    SettlInstTransType(String),
    /// OrdRejReason - Field 103
    OrdRejReason(i64),
    /// NotifyBrokerOfCredit - Field 208
    NotifyBrokerOfCredit(String),
    /// ExecTransType - Field 20
    ExecTransType(String),
    /// IDSource - Field 22
    IDSource(String),
    /// MiscFeeType - Field 139
    MiscFeeType(String),
    /// AllocHandlInst - Field 209
    AllocHandlInst(i64),
    /// ReportToExch - Field 113
    ReportToExch(String),
    /// OrdStatus - Field 39
    OrdStatus(String),
    /// CustomerOrFirm - Field 204
    CustomerOrFirm(i64),
    /// HandlInst - Field 21
    HandlInst(String),
    /// Urgency - Field 61
    Urgency(String),
    /// IOITransType - Field 28
    IOITransType(String),
    /// EncryptMethod - Field 98
    EncryptMethod(i64),
    /// ExecType - Field 150
    ExecType(String),
    /// AllocRejCode - Field 88
    AllocRejCode(i64),
    /// OrdType - Field 40
    OrdType(String),
    /// SettlInstSource - Field 165
    SettlInstSource(String),
    /// CxlRejReason - Field 102
    CxlRejReason(i64),
    /// SettlInstMode - Field 160
    SettlInstMode(String),
    /// PossResend - Field 97
    PossResend(String),
    /// LocateReqd - Field 114
    LocateReqd(String),
    /// SettlLocation - Field 166
    SettlLocation(String),
    /// CoveredOrUncovered - Field 203
    CoveredOrUncovered(i64),
    /// TimeInForce - Field 59
    TimeInForce(String),
    /// ForexReq - Field 121
    ForexReq(String),
    /// OpenClose - Field 77
    OpenClose(String),
    /// LastCapacity - Field 29
    LastCapacity(String),
    /// Side - Field 54
    Side(String),
    /// ResetSeqNumFlag - Field 141
    ResetSeqNumFlag(String),
    /// PutOrCall - Field 201
    PutOrCall(i64),
    /// AllocLinkType - Field 197
    AllocLinkType(i64),
    /// IOIOthSvc - Field 24
    IOIOthSvc(String),
    /// MsgType - Field 35
    MsgType(String),
    /// AllocTransType - Field 71
    AllocTransType(String),
    /// StandInstDbType - Field 169
    StandInstDbType(i64),
    /// ExecInst - Field 18
    ExecInst(String),
    /// AdvSide - Field 4
    AdvSide(String),
}

// Tag number constants
pub const TAG_ADVTRANSTYPE: u32 = 5;
pub const TAG_SECURITYTYPE: u32 = 167;
pub const TAG_IOIQLTYIND: u32 = 25;
pub const TAG_ALLOCSTATUS: u32 = 87;
pub const TAG_IOISHARES: u32 = 27;
pub const TAG_POSSDUPFLAG: u32 = 43;
pub const TAG_SETTLMNTTYP: u32 = 63;
pub const TAG_PROCESSCODE: u32 = 81;
pub const TAG_EMAILTYPE: u32 = 94;
pub const TAG_RULE80A: u32 = 47;
pub const TAG_IOIQUALIFIER: u32 = 104;
pub const TAG_GAPFILLFLAG: u32 = 123;
pub const TAG_DKREASON: u32 = 127;
pub const TAG_COMMTYPE: u32 = 13;
pub const TAG_IOINATURALFLAG: u32 = 130;
pub const TAG_SETTLINSTTRANSTYPE: u32 = 163;
pub const TAG_ORDREJREASON: u32 = 103;
pub const TAG_NOTIFYBROKEROFCREDIT: u32 = 208;
pub const TAG_EXECTRANSTYPE: u32 = 20;
pub const TAG_IDSOURCE: u32 = 22;
pub const TAG_MISCFEETYPE: u32 = 139;
pub const TAG_ALLOCHANDLINST: u32 = 209;
pub const TAG_REPORTTOEXCH: u32 = 113;
pub const TAG_ORDSTATUS: u32 = 39;
pub const TAG_CUSTOMERORFIRM: u32 = 204;
pub const TAG_HANDLINST: u32 = 21;
pub const TAG_URGENCY: u32 = 61;
pub const TAG_IOITRANSTYPE: u32 = 28;
pub const TAG_ENCRYPTMETHOD: u32 = 98;
pub const TAG_EXECTYPE: u32 = 150;
pub const TAG_ALLOCREJCODE: u32 = 88;
pub const TAG_ORDTYPE: u32 = 40;
pub const TAG_SETTLINSTSOURCE: u32 = 165;
pub const TAG_CXLREJREASON: u32 = 102;
pub const TAG_SETTLINSTMODE: u32 = 160;
pub const TAG_POSSRESEND: u32 = 97;
pub const TAG_LOCATEREQD: u32 = 114;
pub const TAG_SETTLLOCATION: u32 = 166;
pub const TAG_COVEREDORUNCOVERED: u32 = 203;
pub const TAG_TIMEINFORCE: u32 = 59;
pub const TAG_FOREXREQ: u32 = 121;
pub const TAG_OPENCLOSE: u32 = 77;
pub const TAG_LASTCAPACITY: u32 = 29;
pub const TAG_SIDE: u32 = 54;
pub const TAG_RESETSEQNUMFLAG: u32 = 141;
pub const TAG_PUTORCALL: u32 = 201;
pub const TAG_ALLOCLINKTYPE: u32 = 197;
pub const TAG_IOIOTHSVC: u32 = 24;
pub const TAG_MSGTYPE: u32 = 35;
pub const TAG_ALLOCTRANSTYPE: u32 = 71;
pub const TAG_STANDINSTDBTYPE: u32 = 169;
pub const TAG_EXECINST: u32 = 18;
pub const TAG_ADVSIDE: u32 = 4;

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    /// ListExecute - 'L'
    ListExecute,
    /// OrderCancelReject - '9'
    OrderCancelReject,
    /// OrderCancelRequest - 'F'
    OrderCancelRequest,
    /// OrderStatusRequest - 'H'
    OrderStatusRequest,
    /// Logon - 'A'
    Logon,
    /// TestRequest - '1'
    TestRequest,
    /// ResendRequest - '2'
    ResendRequest,
    /// Reject - '3'
    Reject,
    /// Allocation - 'J'
    Allocation,
    /// NewOrderSingle - 'D'
    NewOrderSingle,
    /// ListStatusRequest - 'M'
    ListStatusRequest,
    /// Quote - 'S'
    Quote,
    /// Logout - '5'
    Logout,
    /// ListStatus - 'N'
    ListStatus,
    /// Email - 'C'
    Email,
    /// DontKnowTrade - 'Q'
    DontKnowTrade,
    /// QuoteRequest - 'R'
    QuoteRequest,
    /// IOI - '6'
    IOI,
    /// ExecutionReport - '8'
    ExecutionReport,
    /// AllocationInstructionAck - 'P'
    AllocationInstructionAck,
    /// NewOrderList - 'E'
    NewOrderList,
    /// ListCancelRequest - 'K'
    ListCancelRequest,
    /// Advertisement - '7'
    Advertisement,
    /// SettlementInstructions - 'T'
    SettlementInstructions,
    /// News - 'B'
    News,
    /// Heartbeat - '0'
    Heartbeat,
    /// SequenceReset - '4'
    SequenceReset,
    /// OrderCancelReplaceRequest - 'G'
    OrderCancelReplaceRequest,
    /// Unknown message type
    Unknown(String),
}

impl From<String> for MessageType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "L" => MessageType::ListExecute,
            "9" => MessageType::OrderCancelReject,
            "F" => MessageType::OrderCancelRequest,
            "H" => MessageType::OrderStatusRequest,
            "A" => MessageType::Logon,
            "1" => MessageType::TestRequest,
            "2" => MessageType::ResendRequest,
            "3" => MessageType::Reject,
            "J" => MessageType::Allocation,
            "D" => MessageType::NewOrderSingle,
            "M" => MessageType::ListStatusRequest,
            "S" => MessageType::Quote,
            "5" => MessageType::Logout,
            "N" => MessageType::ListStatus,
            "C" => MessageType::Email,
            "Q" => MessageType::DontKnowTrade,
            "R" => MessageType::QuoteRequest,
            "6" => MessageType::IOI,
            "8" => MessageType::ExecutionReport,
            "P" => MessageType::AllocationInstructionAck,
            "E" => MessageType::NewOrderList,
            "K" => MessageType::ListCancelRequest,
            "7" => MessageType::Advertisement,
            "T" => MessageType::SettlementInstructions,
            "B" => MessageType::News,
            "0" => MessageType::Heartbeat,
            "4" => MessageType::SequenceReset,
            "G" => MessageType::OrderCancelReplaceRequest,
            _ => MessageType::Unknown(s),
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::ListExecute => write!(f, "L"),
            MessageType::OrderCancelReject => write!(f, "9"),
            MessageType::OrderCancelRequest => write!(f, "F"),
            MessageType::OrderStatusRequest => write!(f, "H"),
            MessageType::Logon => write!(f, "A"),
            MessageType::TestRequest => write!(f, "1"),
            MessageType::ResendRequest => write!(f, "2"),
            MessageType::Reject => write!(f, "3"),
            MessageType::Allocation => write!(f, "J"),
            MessageType::NewOrderSingle => write!(f, "D"),
            MessageType::ListStatusRequest => write!(f, "M"),
            MessageType::Quote => write!(f, "S"),
            MessageType::Logout => write!(f, "5"),
            MessageType::ListStatus => write!(f, "N"),
            MessageType::Email => write!(f, "C"),
            MessageType::DontKnowTrade => write!(f, "Q"),
            MessageType::QuoteRequest => write!(f, "R"),
            MessageType::IOI => write!(f, "6"),
            MessageType::ExecutionReport => write!(f, "8"),
            MessageType::AllocationInstructionAck => write!(f, "P"),
            MessageType::NewOrderList => write!(f, "E"),
            MessageType::ListCancelRequest => write!(f, "K"),
            MessageType::Advertisement => write!(f, "7"),
            MessageType::SettlementInstructions => write!(f, "T"),
            MessageType::News => write!(f, "B"),
            MessageType::Heartbeat => write!(f, "0"),
            MessageType::SequenceReset => write!(f, "4"),
            MessageType::OrderCancelReplaceRequest => write!(f, "G"),
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
pub struct ListExecute {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<ListExecute> for FixMessage {
    fn from(msg: ListExecute) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::ListExecute,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderCancelReject {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<OrderCancelReject> for FixMessage {
    fn from(msg: OrderCancelReject) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::OrderCancelReject,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderCancelRequest {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<OrderCancelRequest> for FixMessage {
    fn from(msg: OrderCancelRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::OrderCancelRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderStatusRequest {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<OrderStatusRequest> for FixMessage {
    fn from(msg: OrderStatusRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::OrderStatusRequest,
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
pub struct Allocation {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<Allocation> for FixMessage {
    fn from(msg: Allocation) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::Allocation,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewOrderSingle {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<NewOrderSingle> for FixMessage {
    fn from(msg: NewOrderSingle) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::NewOrderSingle,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListStatusRequest {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<ListStatusRequest> for FixMessage {
    fn from(msg: ListStatusRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::ListStatusRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Quote {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<Quote> for FixMessage {
    fn from(msg: Quote) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::Quote,
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
pub struct ListStatus {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<ListStatus> for FixMessage {
    fn from(msg: ListStatus) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::ListStatus,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Email {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<Email> for FixMessage {
    fn from(msg: Email) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::Email,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DontKnowTrade {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<DontKnowTrade> for FixMessage {
    fn from(msg: DontKnowTrade) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::DontKnowTrade,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QuoteRequest {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<QuoteRequest> for FixMessage {
    fn from(msg: QuoteRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::QuoteRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IOI {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<IOI> for FixMessage {
    fn from(msg: IOI) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::IOI,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionReport {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<ExecutionReport> for FixMessage {
    fn from(msg: ExecutionReport) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::ExecutionReport,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AllocationInstructionAck {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<AllocationInstructionAck> for FixMessage {
    fn from(msg: AllocationInstructionAck) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::AllocationInstructionAck,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewOrderList {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<NewOrderList> for FixMessage {
    fn from(msg: NewOrderList) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::NewOrderList,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListCancelRequest {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<ListCancelRequest> for FixMessage {
    fn from(msg: ListCancelRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::ListCancelRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Advertisement {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<Advertisement> for FixMessage {
    fn from(msg: Advertisement) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::Advertisement,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettlementInstructions {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<SettlementInstructions> for FixMessage {
    fn from(msg: SettlementInstructions) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::SettlementInstructions,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct News {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<News> for FixMessage {
    fn from(msg: News) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::News,
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
pub struct OrderCancelReplaceRequest {
    // Header fields
    // Message fields
    // Trailer fields
}

impl From<OrderCancelReplaceRequest> for FixMessage {
    fn from(msg: OrderCancelReplaceRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        FixMessage {
            msg_type: MessageType::OrderCancelReplaceRequest,
            fields,
        }
    }
}

