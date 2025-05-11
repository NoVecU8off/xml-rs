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
#[allow(non_camel_case_types)]
pub enum PossDupFlagEnum {
    /// NO
    NO,
    /// YES
    YES,
    /// Unknown value
    Unknown(String),
}

impl From<String> for PossDupFlagEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" => PossDupFlagEnum::NO,
            "Y" => PossDupFlagEnum::YES,
            _ => PossDupFlagEnum::Unknown(s),
        }
    }
}

impl fmt::Display for PossDupFlagEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PossDupFlagEnum::NO => write!(f, "N"),
            PossDupFlagEnum::YES => write!(f, "Y"),
            PossDupFlagEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TestMessageIndicatorEnum {
    /// NO
    NO,
    /// YES
    YES,
    /// Unknown value
    Unknown(String),
}

impl From<String> for TestMessageIndicatorEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" => TestMessageIndicatorEnum::NO,
            "Y" => TestMessageIndicatorEnum::YES,
            _ => TestMessageIndicatorEnum::Unknown(s),
        }
    }
}

impl fmt::Display for TestMessageIndicatorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestMessageIndicatorEnum::NO => write!(f, "N"),
            TestMessageIndicatorEnum::YES => write!(f, "Y"),
            TestMessageIndicatorEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SessionStatusEnum {
    /// SESSION_ACTIVE
    SESSION_ACTIVE,
    /// SESSION_PASSWORD_CHANGED
    SESSION_PASSWORD_CHANGED,
    /// SESSION_PASSWORD_DUE_TO_EXPIRE
    SESSION_PASSWORD_DUE_TO_EXPIRE,
    /// NEW_SESSION_PASSWORD_DOES_NOT_COMPLY_WITH_POLICY
    NEW_SESSION_PASSWORD_DOES_NOT_COMPLY_WITH_POLICY,
    /// SESSION_LOGOUT_COMPLETE
    SESSION_LOGOUT_COMPLETE,
    /// INVALID_USERNAME_OR_PASSWORD
    INVALID_USERNAME_OR_PASSWORD,
    /// ACCOUNT_LOCKED
    ACCOUNT_LOCKED,
    /// LOGONS_ARE_NOT_ALLOWED_AT_THIS_TIME
    LOGONS_ARE_NOT_ALLOWED_AT_THIS_TIME,
    /// PASSWORD_EXPIRED
    PASSWORD_EXPIRED,
    /// Unknown value
    Unknown(String),
}

impl From<String> for SessionStatusEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => SessionStatusEnum::SESSION_ACTIVE,
            "1" => SessionStatusEnum::SESSION_PASSWORD_CHANGED,
            "2" => SessionStatusEnum::SESSION_PASSWORD_DUE_TO_EXPIRE,
            "3" => SessionStatusEnum::NEW_SESSION_PASSWORD_DOES_NOT_COMPLY_WITH_POLICY,
            "4" => SessionStatusEnum::SESSION_LOGOUT_COMPLETE,
            "5" => SessionStatusEnum::INVALID_USERNAME_OR_PASSWORD,
            "6" => SessionStatusEnum::ACCOUNT_LOCKED,
            "7" => SessionStatusEnum::LOGONS_ARE_NOT_ALLOWED_AT_THIS_TIME,
            "8" => SessionStatusEnum::PASSWORD_EXPIRED,
            _ => SessionStatusEnum::Unknown(s),
        }
    }
}

impl fmt::Display for SessionStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionStatusEnum::SESSION_ACTIVE => write!(f, "0"),
            SessionStatusEnum::SESSION_PASSWORD_CHANGED => write!(f, "1"),
            SessionStatusEnum::SESSION_PASSWORD_DUE_TO_EXPIRE => write!(f, "2"),
            SessionStatusEnum::NEW_SESSION_PASSWORD_DOES_NOT_COMPLY_WITH_POLICY => write!(f, "3"),
            SessionStatusEnum::SESSION_LOGOUT_COMPLETE => write!(f, "4"),
            SessionStatusEnum::INVALID_USERNAME_OR_PASSWORD => write!(f, "5"),
            SessionStatusEnum::ACCOUNT_LOCKED => write!(f, "6"),
            SessionStatusEnum::LOGONS_ARE_NOT_ALLOWED_AT_THIS_TIME => write!(f, "7"),
            SessionStatusEnum::PASSWORD_EXPIRED => write!(f, "8"),
            SessionStatusEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SessionRejectReasonEnum {
    /// INVALID_TAG_NUMBER
    INVALID_TAG_NUMBER,
    /// REQUIRED_TAG_MISSING
    REQUIRED_TAG_MISSING,
    /// TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE
    TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE,
    /// UNDEFINED_TAG
    UNDEFINED_TAG,
    /// TAG_SPECIFIED_WITHOUT_A_VALUE
    TAG_SPECIFIED_WITHOUT_A_VALUE,
    /// VALUE_IS_INCORRECT
    VALUE_IS_INCORRECT,
    /// INCORRECT_DATA_FORMAT_FOR_VALUE
    INCORRECT_DATA_FORMAT_FOR_VALUE,
    /// DECRYPTION_PROBLEM
    DECRYPTION_PROBLEM,
    /// SIGNATURE_PROBLEM
    SIGNATURE_PROBLEM,
    /// COMPID_PROBLEM
    COMPID_PROBLEM,
    /// SENDINGTIME_ACCURACY_PROBLEM
    SENDINGTIME_ACCURACY_PROBLEM,
    /// INVALID_MSGTYPE
    INVALID_MSGTYPE,
    /// XML_VALIDATION_ERROR
    XML_VALIDATION_ERROR,
    /// TAG_APPEARS_MORE_THAN_ONCE
    TAG_APPEARS_MORE_THAN_ONCE,
    /// TAG_SPECIFIED_OUT_OF_REQUIRED_ORDER
    TAG_SPECIFIED_OUT_OF_REQUIRED_ORDER,
    /// REPEATING_GROUP_FIELDS_OUT_OF_ORDER
    REPEATING_GROUP_FIELDS_OUT_OF_ORDER,
    /// INCORRECT_NUMINGROUP_COUNT_FOR_REPEATING_GROUP
    INCORRECT_NUMINGROUP_COUNT_FOR_REPEATING_GROUP,
    /// NON_DATA_VALUE_INCLUDES_FIELD_DELIMITER
    NON_DATA_VALUE_INCLUDES_FIELD_DELIMITER,
    /// INVALID_UNSUPPORTED_APPLICATION_VERSION
    INVALID_UNSUPPORTED_APPLICATION_VERSION,
    /// OTHER
    OTHER,
    /// Unknown value
    Unknown(String),
}

impl From<String> for SessionRejectReasonEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => SessionRejectReasonEnum::INVALID_TAG_NUMBER,
            "1" => SessionRejectReasonEnum::REQUIRED_TAG_MISSING,
            "2" => SessionRejectReasonEnum::TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE,
            "3" => SessionRejectReasonEnum::UNDEFINED_TAG,
            "4" => SessionRejectReasonEnum::TAG_SPECIFIED_WITHOUT_A_VALUE,
            "5" => SessionRejectReasonEnum::VALUE_IS_INCORRECT,
            "6" => SessionRejectReasonEnum::INCORRECT_DATA_FORMAT_FOR_VALUE,
            "7" => SessionRejectReasonEnum::DECRYPTION_PROBLEM,
            "8" => SessionRejectReasonEnum::SIGNATURE_PROBLEM,
            "9" => SessionRejectReasonEnum::COMPID_PROBLEM,
            "10" => SessionRejectReasonEnum::SENDINGTIME_ACCURACY_PROBLEM,
            "11" => SessionRejectReasonEnum::INVALID_MSGTYPE,
            "12" => SessionRejectReasonEnum::XML_VALIDATION_ERROR,
            "13" => SessionRejectReasonEnum::TAG_APPEARS_MORE_THAN_ONCE,
            "14" => SessionRejectReasonEnum::TAG_SPECIFIED_OUT_OF_REQUIRED_ORDER,
            "15" => SessionRejectReasonEnum::REPEATING_GROUP_FIELDS_OUT_OF_ORDER,
            "16" => SessionRejectReasonEnum::INCORRECT_NUMINGROUP_COUNT_FOR_REPEATING_GROUP,
            "17" => SessionRejectReasonEnum::NON_DATA_VALUE_INCLUDES_FIELD_DELIMITER,
            "18" => SessionRejectReasonEnum::INVALID_UNSUPPORTED_APPLICATION_VERSION,
            "99" => SessionRejectReasonEnum::OTHER,
            _ => SessionRejectReasonEnum::Unknown(s),
        }
    }
}

impl fmt::Display for SessionRejectReasonEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionRejectReasonEnum::INVALID_TAG_NUMBER => write!(f, "0"),
            SessionRejectReasonEnum::REQUIRED_TAG_MISSING => write!(f, "1"),
            SessionRejectReasonEnum::TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE => write!(f, "2"),
            SessionRejectReasonEnum::UNDEFINED_TAG => write!(f, "3"),
            SessionRejectReasonEnum::TAG_SPECIFIED_WITHOUT_A_VALUE => write!(f, "4"),
            SessionRejectReasonEnum::VALUE_IS_INCORRECT => write!(f, "5"),
            SessionRejectReasonEnum::INCORRECT_DATA_FORMAT_FOR_VALUE => write!(f, "6"),
            SessionRejectReasonEnum::DECRYPTION_PROBLEM => write!(f, "7"),
            SessionRejectReasonEnum::SIGNATURE_PROBLEM => write!(f, "8"),
            SessionRejectReasonEnum::COMPID_PROBLEM => write!(f, "9"),
            SessionRejectReasonEnum::SENDINGTIME_ACCURACY_PROBLEM => write!(f, "10"),
            SessionRejectReasonEnum::INVALID_MSGTYPE => write!(f, "11"),
            SessionRejectReasonEnum::XML_VALIDATION_ERROR => write!(f, "12"),
            SessionRejectReasonEnum::TAG_APPEARS_MORE_THAN_ONCE => write!(f, "13"),
            SessionRejectReasonEnum::TAG_SPECIFIED_OUT_OF_REQUIRED_ORDER => write!(f, "14"),
            SessionRejectReasonEnum::REPEATING_GROUP_FIELDS_OUT_OF_ORDER => write!(f, "15"),
            SessionRejectReasonEnum::INCORRECT_NUMINGROUP_COUNT_FOR_REPEATING_GROUP => write!(f, "16"),
            SessionRejectReasonEnum::NON_DATA_VALUE_INCLUDES_FIELD_DELIMITER => write!(f, "17"),
            SessionRejectReasonEnum::INVALID_UNSUPPORTED_APPLICATION_VERSION => write!(f, "18"),
            SessionRejectReasonEnum::OTHER => write!(f, "99"),
            SessionRejectReasonEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ApplVerIDEnum {
    /// FIX27
    FIX27,
    /// FIX30
    FIX30,
    /// FIX40
    FIX40,
    /// FIX41
    FIX41,
    /// FIX42
    FIX42,
    /// FIX43
    FIX43,
    /// FIX44
    FIX44,
    /// FIX50
    FIX50,
    /// FIX50_SP1
    FIX50_SP1,
    /// FIX50_SP2
    FIX50_SP2,
    /// Unknown value
    Unknown(String),
}

impl From<String> for ApplVerIDEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => ApplVerIDEnum::FIX27,
            "1" => ApplVerIDEnum::FIX30,
            "2" => ApplVerIDEnum::FIX40,
            "3" => ApplVerIDEnum::FIX41,
            "4" => ApplVerIDEnum::FIX42,
            "5" => ApplVerIDEnum::FIX43,
            "6" => ApplVerIDEnum::FIX44,
            "7" => ApplVerIDEnum::FIX50,
            "8" => ApplVerIDEnum::FIX50_SP1,
            "9" => ApplVerIDEnum::FIX50_SP2,
            _ => ApplVerIDEnum::Unknown(s),
        }
    }
}

impl fmt::Display for ApplVerIDEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplVerIDEnum::FIX27 => write!(f, "0"),
            ApplVerIDEnum::FIX30 => write!(f, "1"),
            ApplVerIDEnum::FIX40 => write!(f, "2"),
            ApplVerIDEnum::FIX41 => write!(f, "3"),
            ApplVerIDEnum::FIX42 => write!(f, "4"),
            ApplVerIDEnum::FIX43 => write!(f, "5"),
            ApplVerIDEnum::FIX44 => write!(f, "6"),
            ApplVerIDEnum::FIX50 => write!(f, "7"),
            ApplVerIDEnum::FIX50_SP1 => write!(f, "8"),
            ApplVerIDEnum::FIX50_SP2 => write!(f, "9"),
            ApplVerIDEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum GapFillFlagEnum {
    /// NO
    NO,
    /// YES
    YES,
    /// Unknown value
    Unknown(String),
}

impl From<String> for GapFillFlagEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" => GapFillFlagEnum::NO,
            "Y" => GapFillFlagEnum::YES,
            _ => GapFillFlagEnum::Unknown(s),
        }
    }
}

impl fmt::Display for GapFillFlagEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GapFillFlagEnum::NO => write!(f, "N"),
            GapFillFlagEnum::YES => write!(f, "Y"),
            GapFillFlagEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PossResendEnum {
    /// NO
    NO,
    /// YES
    YES,
    /// Unknown value
    Unknown(String),
}

impl From<String> for PossResendEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" => PossResendEnum::NO,
            "Y" => PossResendEnum::YES,
            _ => PossResendEnum::Unknown(s),
        }
    }
}

impl fmt::Display for PossResendEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PossResendEnum::NO => write!(f, "N"),
            PossResendEnum::YES => write!(f, "Y"),
            PossResendEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum EncryptMethodEnum {
    /// NONE_OTHER
    NONE_OTHER,
    /// PKCS
    PKCS,
    /// DES
    DES,
    /// PKCSDES
    PKCSDES,
    /// PGPDES
    PGPDES,
    /// PGPDESMD5
    PGPDESMD5,
    /// PEMDESMD5
    PEMDESMD5,
    /// Unknown value
    Unknown(String),
}

impl From<String> for EncryptMethodEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => EncryptMethodEnum::NONE_OTHER,
            "1" => EncryptMethodEnum::PKCS,
            "2" => EncryptMethodEnum::DES,
            "3" => EncryptMethodEnum::PKCSDES,
            "4" => EncryptMethodEnum::PGPDES,
            "5" => EncryptMethodEnum::PGPDESMD5,
            "6" => EncryptMethodEnum::PEMDESMD5,
            _ => EncryptMethodEnum::Unknown(s),
        }
    }
}

impl fmt::Display for EncryptMethodEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptMethodEnum::NONE_OTHER => write!(f, "0"),
            EncryptMethodEnum::PKCS => write!(f, "1"),
            EncryptMethodEnum::DES => write!(f, "2"),
            EncryptMethodEnum::PKCSDES => write!(f, "3"),
            EncryptMethodEnum::PGPDES => write!(f, "4"),
            EncryptMethodEnum::PGPDESMD5 => write!(f, "5"),
            EncryptMethodEnum::PEMDESMD5 => write!(f, "6"),
            EncryptMethodEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ResetSeqNumFlagEnum {
    /// NO
    NO,
    /// YES
    YES,
    /// Unknown value
    Unknown(String),
}

impl From<String> for ResetSeqNumFlagEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" => ResetSeqNumFlagEnum::NO,
            "Y" => ResetSeqNumFlagEnum::YES,
            _ => ResetSeqNumFlagEnum::Unknown(s),
        }
    }
}

impl fmt::Display for ResetSeqNumFlagEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResetSeqNumFlagEnum::NO => write!(f, "N"),
            ResetSeqNumFlagEnum::YES => write!(f, "Y"),
            ResetSeqNumFlagEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagValue {
    /// EncodedText - Field 355
    EncodedText(Vec<u8>),
    /// MsgType - Field 35
    MsgType(String),
    /// HeartBtInt - Field 108
    HeartBtInt(i64),
    /// PossDupFlag - Field 43
    PossDupFlag(PossDupFlagEnum),
    /// Signature - Field 89
    Signature(Vec<u8>),
    /// MessageEncoding - Field 347
    MessageEncoding(String),
    /// RefMsgType - Field 372
    RefMsgType(String),
    /// TestMessageIndicator - Field 464
    TestMessageIndicator(TestMessageIndicatorEnum),
    /// BeginSeqNo - Field 7
    BeginSeqNo(i64),
    /// EncryptedPasswordLen - Field 1401
    EncryptedPasswordLen(i64),
    /// TargetLocationID - Field 143
    TargetLocationID(String),
    /// SessionStatus - Field 1409
    SessionStatus(SessionStatusEnum),
    /// SessionRejectReason - Field 373
    SessionRejectReason(SessionRejectReasonEnum),
    /// RefSeqNum - Field 45
    RefSeqNum(i64),
    /// HopRefID - Field 630
    HopRefID(i64),
    /// EncryptedNewPasswordLen - Field 1403
    EncryptedNewPasswordLen(i64),
    /// DeliverToLocationID - Field 145
    DeliverToLocationID(String),
    /// DefaultCstmApplVerID - Field 1408
    DefaultCstmApplVerID(String),
    /// RefTagID - Field 371
    RefTagID(i64),
    /// EncryptedPasswordMethod - Field 1400
    EncryptedPasswordMethod(i64),
    /// ApplVerID - Field 1128
    ApplVerID(ApplVerIDEnum),
    /// MsgSeqNum - Field 34
    MsgSeqNum(i64),
    /// GapFillFlag - Field 123
    GapFillFlag(GapFillFlagEnum),
    /// XmlDataLen - Field 212
    XmlDataLen(i64),
    /// EncryptedPassword - Field 1402
    EncryptedPassword(Vec<u8>),
    /// CheckSum - Field 10
    CheckSum(String),
    /// OnBehalfOfSubID - Field 116
    OnBehalfOfSubID(String),
    /// OnBehalfOfCompID - Field 115
    OnBehalfOfCompID(String),
    /// EncodedTextLen - Field 354
    EncodedTextLen(i64),
    /// Password - Field 554
    Password(String),
    /// XmlData - Field 213
    XmlData(Vec<u8>),
    /// ApplExtID - Field 1156
    ApplExtID(i64),
    /// EndSeqNo - Field 16
    EndSeqNo(i64),
    /// SecureData - Field 91
    SecureData(Vec<u8>),
    /// PossResend - Field 97
    PossResend(PossResendEnum),
    /// RawDataLength - Field 95
    RawDataLength(i64),
    /// HopSendingTime - Field 629
    HopSendingTime(NaiveDateTime),
    /// HopCompID - Field 628
    HopCompID(String),
    /// Text - Field 58
    Text(String),
    /// SendingTime - Field 52
    SendingTime(NaiveDateTime),
    /// NewSeqNo - Field 36
    NewSeqNo(i64),
    /// NextExpectedMsgSeqNum - Field 789
    NextExpectedMsgSeqNum(i64),
    /// OnBehalfOfLocationID - Field 144
    OnBehalfOfLocationID(String),
    /// NewPassword - Field 925
    NewPassword(String),
    /// BodyLength - Field 9
    BodyLength(i64),
    /// EncryptMethod - Field 98
    EncryptMethod(EncryptMethodEnum),
    /// TargetCompID - Field 56
    TargetCompID(String),
    /// OrigSendingTime - Field 122
    OrigSendingTime(NaiveDateTime),
    /// RefCstmApplVerID - Field 1131
    RefCstmApplVerID(String),
    /// TargetSubID - Field 57
    TargetSubID(String),
    /// CstmApplVerID - Field 1129
    CstmApplVerID(String),
    /// RefApplVerID - Field 1130
    RefApplVerID(String),
    /// SenderSubID - Field 50
    SenderSubID(String),
    /// TestReqID - Field 112
    TestReqID(String),
    /// ResetSeqNumFlag - Field 141
    ResetSeqNumFlag(ResetSeqNumFlagEnum),
    /// BeginString - Field 8
    BeginString(String),
    /// DefaultApplVerID - Field 1137
    DefaultApplVerID(String),
    /// DeliverToCompID - Field 128
    DeliverToCompID(String),
    /// MaxMessageSize - Field 383
    MaxMessageSize(i64),
    /// SenderLocationID - Field 142
    SenderLocationID(String),
    /// DefaultApplExtID - Field 1407
    DefaultApplExtID(i64),
    /// SenderCompID - Field 49
    SenderCompID(String),
    /// SignatureLength - Field 93
    SignatureLength(i64),
    /// LastMsgSeqNumProcessed - Field 369
    LastMsgSeqNumProcessed(i64),
    /// NoHops - Field 627
    NoHops(i64),
    /// EncryptedNewPassword - Field 1404
    EncryptedNewPassword(Vec<u8>),
    /// SecureDataLen - Field 90
    SecureDataLen(i64),
    /// Username - Field 553
    Username(String),
    /// RawData - Field 96
    RawData(Vec<u8>),
    /// RefApplExtID - Field 1406
    RefApplExtID(i64),
    /// DeliverToSubID - Field 129
    DeliverToSubID(String),
}

// Tag number constants
pub const TAG_ENCODEDTEXT: u32 = 355;
pub const TAG_MSGTYPE: u32 = 35;
pub const TAG_HEARTBTINT: u32 = 108;
pub const TAG_POSSDUPFLAG: u32 = 43;
pub const TAG_SIGNATURE: u32 = 89;
pub const TAG_MESSAGEENCODING: u32 = 347;
pub const TAG_REFMSGTYPE: u32 = 372;
pub const TAG_TESTMESSAGEINDICATOR: u32 = 464;
pub const TAG_BEGINSEQNO: u32 = 7;
pub const TAG_ENCRYPTEDPASSWORDLEN: u32 = 1401;
pub const TAG_TARGETLOCATIONID: u32 = 143;
pub const TAG_SESSIONSTATUS: u32 = 1409;
pub const TAG_SESSIONREJECTREASON: u32 = 373;
pub const TAG_REFSEQNUM: u32 = 45;
pub const TAG_HOPREFID: u32 = 630;
pub const TAG_ENCRYPTEDNEWPASSWORDLEN: u32 = 1403;
pub const TAG_DELIVERTOLOCATIONID: u32 = 145;
pub const TAG_DEFAULTCSTMAPPLVERID: u32 = 1408;
pub const TAG_REFTAGID: u32 = 371;
pub const TAG_ENCRYPTEDPASSWORDMETHOD: u32 = 1400;
pub const TAG_APPLVERID: u32 = 1128;
pub const TAG_MSGSEQNUM: u32 = 34;
pub const TAG_GAPFILLFLAG: u32 = 123;
pub const TAG_XMLDATALEN: u32 = 212;
pub const TAG_ENCRYPTEDPASSWORD: u32 = 1402;
pub const TAG_CHECKSUM: u32 = 10;
pub const TAG_ONBEHALFOFSUBID: u32 = 116;
pub const TAG_ONBEHALFOFCOMPID: u32 = 115;
pub const TAG_ENCODEDTEXTLEN: u32 = 354;
pub const TAG_PASSWORD: u32 = 554;
pub const TAG_XMLDATA: u32 = 213;
pub const TAG_APPLEXTID: u32 = 1156;
pub const TAG_ENDSEQNO: u32 = 16;
pub const TAG_SECUREDATA: u32 = 91;
pub const TAG_POSSRESEND: u32 = 97;
pub const TAG_RAWDATALENGTH: u32 = 95;
pub const TAG_HOPSENDINGTIME: u32 = 629;
pub const TAG_HOPCOMPID: u32 = 628;
pub const TAG_TEXT: u32 = 58;
pub const TAG_SENDINGTIME: u32 = 52;
pub const TAG_NEWSEQNO: u32 = 36;
pub const TAG_NEXTEXPECTEDMSGSEQNUM: u32 = 789;
pub const TAG_ONBEHALFOFLOCATIONID: u32 = 144;
pub const TAG_NEWPASSWORD: u32 = 925;
pub const TAG_BODYLENGTH: u32 = 9;
pub const TAG_ENCRYPTMETHOD: u32 = 98;
pub const TAG_TARGETCOMPID: u32 = 56;
pub const TAG_ORIGSENDINGTIME: u32 = 122;
pub const TAG_REFCSTMAPPLVERID: u32 = 1131;
pub const TAG_TARGETSUBID: u32 = 57;
pub const TAG_CSTMAPPLVERID: u32 = 1129;
pub const TAG_REFAPPLVERID: u32 = 1130;
pub const TAG_SENDERSUBID: u32 = 50;
pub const TAG_TESTREQID: u32 = 112;
pub const TAG_RESETSEQNUMFLAG: u32 = 141;
pub const TAG_BEGINSTRING: u32 = 8;
pub const TAG_DEFAULTAPPLVERID: u32 = 1137;
pub const TAG_DELIVERTOCOMPID: u32 = 128;
pub const TAG_MAXMESSAGESIZE: u32 = 383;
pub const TAG_SENDERLOCATIONID: u32 = 142;
pub const TAG_DEFAULTAPPLEXTID: u32 = 1407;
pub const TAG_SENDERCOMPID: u32 = 49;
pub const TAG_SIGNATURELENGTH: u32 = 93;
pub const TAG_LASTMSGSEQNUMPROCESSED: u32 = 369;
pub const TAG_NOHOPS: u32 = 627;
pub const TAG_ENCRYPTEDNEWPASSWORD: u32 = 1404;
pub const TAG_SECUREDATALEN: u32 = 90;
pub const TAG_USERNAME: u32 = 553;
pub const TAG_RAWDATA: u32 = 96;
pub const TAG_REFAPPLEXTID: u32 = 1406;
pub const TAG_DELIVERTOSUBID: u32 = 129;

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    /// Heartbeat - '0'
    Heartbeat,
    /// Logout - '5'
    Logout,
    /// Logon - 'A'
    Logon,
    /// TestRequest - '1'
    TestRequest,
    /// Reject - '3'
    Reject,
    /// ResendRequest - '2'
    ResendRequest,
    /// SequenceReset - '4'
    SequenceReset,
    /// Unknown message type
    Unknown(String),
}

impl From<String> for MessageType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => MessageType::Heartbeat,
            "5" => MessageType::Logout,
            "A" => MessageType::Logon,
            "1" => MessageType::TestRequest,
            "3" => MessageType::Reject,
            "2" => MessageType::ResendRequest,
            "4" => MessageType::SequenceReset,
            _ => MessageType::Unknown(s),
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Heartbeat => write!(f, "0"),
            MessageType::Logout => write!(f, "5"),
            MessageType::Logon => write!(f, "A"),
            MessageType::TestRequest => write!(f, "1"),
            MessageType::Reject => write!(f, "3"),
            MessageType::ResendRequest => write!(f, "2"),
            MessageType::SequenceReset => write!(f, "4"),
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
pub struct Heartbeat {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: String,
    pub appl_ver_id: Option<ApplVerIDEnum>,
    pub appl_ext_id: Option<i64>,
    pub cstm_appl_ver_id: Option<String>,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub on_behalf_of_comp_id: Option<String>,
    pub deliver_to_comp_id: Option<String>,
    pub secure_data_len: Option<i64>,
    pub secure_data: Option<Vec<u8>>,
    pub msg_seq_num: i64,
    pub sender_sub_id: Option<String>,
    pub sender_location_id: Option<String>,
    pub target_sub_id: Option<String>,
    pub target_location_id: Option<String>,
    pub on_behalf_of_sub_id: Option<String>,
    pub on_behalf_of_location_id: Option<String>,
    pub deliver_to_sub_id: Option<String>,
    pub deliver_to_location_id: Option<String>,
    pub poss_dup_flag: Option<PossDupFlagEnum>,
    pub poss_resend: Option<PossResendEnum>,
    pub sending_time: NaiveDateTime,
    pub orig_sending_time: Option<NaiveDateTime>,
    pub xml_data_len: Option<i64>,
    pub xml_data: Option<Vec<u8>>,
    pub message_encoding: Option<String>,
    pub last_msg_seq_num_processed: Option<i64>,
    // Message fields
    pub test_req_id: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<Heartbeat> for FixMessage {
    fn from(msg: Heartbeat) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
        if let Some(value) = msg.appl_ver_id {
            fields.insert(TAG_APPLVERID, TagValue::ApplVerID(value));
        }
        if let Some(value) = msg.appl_ext_id {
            fields.insert(TAG_APPLEXTID, TagValue::ApplExtID(value));
        }
        if let Some(value) = msg.cstm_appl_ver_id {
            fields.insert(TAG_CSTMAPPLVERID, TagValue::CstmApplVerID(value));
        }
        fields.insert(TAG_SENDERCOMPID, TagValue::SenderCompID(msg.sender_comp_id));
        fields.insert(TAG_TARGETCOMPID, TagValue::TargetCompID(msg.target_comp_id));
        if let Some(value) = msg.on_behalf_of_comp_id {
            fields.insert(TAG_ONBEHALFOFCOMPID, TagValue::OnBehalfOfCompID(value));
        }
        if let Some(value) = msg.deliver_to_comp_id {
            fields.insert(TAG_DELIVERTOCOMPID, TagValue::DeliverToCompID(value));
        }
        if let Some(value) = msg.secure_data_len {
            fields.insert(TAG_SECUREDATALEN, TagValue::SecureDataLen(value));
        }
        if let Some(value) = msg.secure_data {
            fields.insert(TAG_SECUREDATA, TagValue::SecureData(value));
        }
        fields.insert(TAG_MSGSEQNUM, TagValue::MsgSeqNum(msg.msg_seq_num));
        if let Some(value) = msg.sender_sub_id {
            fields.insert(TAG_SENDERSUBID, TagValue::SenderSubID(value));
        }
        if let Some(value) = msg.sender_location_id {
            fields.insert(TAG_SENDERLOCATIONID, TagValue::SenderLocationID(value));
        }
        if let Some(value) = msg.target_sub_id {
            fields.insert(TAG_TARGETSUBID, TagValue::TargetSubID(value));
        }
        if let Some(value) = msg.target_location_id {
            fields.insert(TAG_TARGETLOCATIONID, TagValue::TargetLocationID(value));
        }
        if let Some(value) = msg.on_behalf_of_sub_id {
            fields.insert(TAG_ONBEHALFOFSUBID, TagValue::OnBehalfOfSubID(value));
        }
        if let Some(value) = msg.on_behalf_of_location_id {
            fields.insert(TAG_ONBEHALFOFLOCATIONID, TagValue::OnBehalfOfLocationID(value));
        }
        if let Some(value) = msg.deliver_to_sub_id {
            fields.insert(TAG_DELIVERTOSUBID, TagValue::DeliverToSubID(value));
        }
        if let Some(value) = msg.deliver_to_location_id {
            fields.insert(TAG_DELIVERTOLOCATIONID, TagValue::DeliverToLocationID(value));
        }
        if let Some(value) = msg.poss_dup_flag {
            fields.insert(TAG_POSSDUPFLAG, TagValue::PossDupFlag(value));
        }
        if let Some(value) = msg.poss_resend {
            fields.insert(TAG_POSSRESEND, TagValue::PossResend(value));
        }
        fields.insert(TAG_SENDINGTIME, TagValue::SendingTime(msg.sending_time));
        if let Some(value) = msg.orig_sending_time {
            fields.insert(TAG_ORIGSENDINGTIME, TagValue::OrigSendingTime(value));
        }
        if let Some(value) = msg.xml_data_len {
            fields.insert(TAG_XMLDATALEN, TagValue::XmlDataLen(value));
        }
        if let Some(value) = msg.xml_data {
            fields.insert(TAG_XMLDATA, TagValue::XmlData(value));
        }
        if let Some(value) = msg.message_encoding {
            fields.insert(TAG_MESSAGEENCODING, TagValue::MessageEncoding(value));
        }
        if let Some(value) = msg.last_msg_seq_num_processed {
            fields.insert(TAG_LASTMSGSEQNUMPROCESSED, TagValue::LastMsgSeqNumProcessed(value));
        }
        if let Some(value) = msg.test_req_id {
            fields.insert(TAG_TESTREQID, TagValue::TestReqID(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::Heartbeat,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Logout {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: String,
    pub appl_ver_id: Option<ApplVerIDEnum>,
    pub appl_ext_id: Option<i64>,
    pub cstm_appl_ver_id: Option<String>,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub on_behalf_of_comp_id: Option<String>,
    pub deliver_to_comp_id: Option<String>,
    pub secure_data_len: Option<i64>,
    pub secure_data: Option<Vec<u8>>,
    pub msg_seq_num: i64,
    pub sender_sub_id: Option<String>,
    pub sender_location_id: Option<String>,
    pub target_sub_id: Option<String>,
    pub target_location_id: Option<String>,
    pub on_behalf_of_sub_id: Option<String>,
    pub on_behalf_of_location_id: Option<String>,
    pub deliver_to_sub_id: Option<String>,
    pub deliver_to_location_id: Option<String>,
    pub poss_dup_flag: Option<PossDupFlagEnum>,
    pub poss_resend: Option<PossResendEnum>,
    pub sending_time: NaiveDateTime,
    pub orig_sending_time: Option<NaiveDateTime>,
    pub xml_data_len: Option<i64>,
    pub xml_data: Option<Vec<u8>>,
    pub message_encoding: Option<String>,
    pub last_msg_seq_num_processed: Option<i64>,
    // Message fields
    pub session_status: Option<SessionStatusEnum>,
    pub text: Option<String>,
    pub encoded_text_len: Option<i64>,
    pub encoded_text: Option<Vec<u8>>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<Logout> for FixMessage {
    fn from(msg: Logout) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
        if let Some(value) = msg.appl_ver_id {
            fields.insert(TAG_APPLVERID, TagValue::ApplVerID(value));
        }
        if let Some(value) = msg.appl_ext_id {
            fields.insert(TAG_APPLEXTID, TagValue::ApplExtID(value));
        }
        if let Some(value) = msg.cstm_appl_ver_id {
            fields.insert(TAG_CSTMAPPLVERID, TagValue::CstmApplVerID(value));
        }
        fields.insert(TAG_SENDERCOMPID, TagValue::SenderCompID(msg.sender_comp_id));
        fields.insert(TAG_TARGETCOMPID, TagValue::TargetCompID(msg.target_comp_id));
        if let Some(value) = msg.on_behalf_of_comp_id {
            fields.insert(TAG_ONBEHALFOFCOMPID, TagValue::OnBehalfOfCompID(value));
        }
        if let Some(value) = msg.deliver_to_comp_id {
            fields.insert(TAG_DELIVERTOCOMPID, TagValue::DeliverToCompID(value));
        }
        if let Some(value) = msg.secure_data_len {
            fields.insert(TAG_SECUREDATALEN, TagValue::SecureDataLen(value));
        }
        if let Some(value) = msg.secure_data {
            fields.insert(TAG_SECUREDATA, TagValue::SecureData(value));
        }
        fields.insert(TAG_MSGSEQNUM, TagValue::MsgSeqNum(msg.msg_seq_num));
        if let Some(value) = msg.sender_sub_id {
            fields.insert(TAG_SENDERSUBID, TagValue::SenderSubID(value));
        }
        if let Some(value) = msg.sender_location_id {
            fields.insert(TAG_SENDERLOCATIONID, TagValue::SenderLocationID(value));
        }
        if let Some(value) = msg.target_sub_id {
            fields.insert(TAG_TARGETSUBID, TagValue::TargetSubID(value));
        }
        if let Some(value) = msg.target_location_id {
            fields.insert(TAG_TARGETLOCATIONID, TagValue::TargetLocationID(value));
        }
        if let Some(value) = msg.on_behalf_of_sub_id {
            fields.insert(TAG_ONBEHALFOFSUBID, TagValue::OnBehalfOfSubID(value));
        }
        if let Some(value) = msg.on_behalf_of_location_id {
            fields.insert(TAG_ONBEHALFOFLOCATIONID, TagValue::OnBehalfOfLocationID(value));
        }
        if let Some(value) = msg.deliver_to_sub_id {
            fields.insert(TAG_DELIVERTOSUBID, TagValue::DeliverToSubID(value));
        }
        if let Some(value) = msg.deliver_to_location_id {
            fields.insert(TAG_DELIVERTOLOCATIONID, TagValue::DeliverToLocationID(value));
        }
        if let Some(value) = msg.poss_dup_flag {
            fields.insert(TAG_POSSDUPFLAG, TagValue::PossDupFlag(value));
        }
        if let Some(value) = msg.poss_resend {
            fields.insert(TAG_POSSRESEND, TagValue::PossResend(value));
        }
        fields.insert(TAG_SENDINGTIME, TagValue::SendingTime(msg.sending_time));
        if let Some(value) = msg.orig_sending_time {
            fields.insert(TAG_ORIGSENDINGTIME, TagValue::OrigSendingTime(value));
        }
        if let Some(value) = msg.xml_data_len {
            fields.insert(TAG_XMLDATALEN, TagValue::XmlDataLen(value));
        }
        if let Some(value) = msg.xml_data {
            fields.insert(TAG_XMLDATA, TagValue::XmlData(value));
        }
        if let Some(value) = msg.message_encoding {
            fields.insert(TAG_MESSAGEENCODING, TagValue::MessageEncoding(value));
        }
        if let Some(value) = msg.last_msg_seq_num_processed {
            fields.insert(TAG_LASTMSGSEQNUMPROCESSED, TagValue::LastMsgSeqNumProcessed(value));
        }
        if let Some(value) = msg.session_status {
            fields.insert(TAG_SESSIONSTATUS, TagValue::SessionStatus(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.encoded_text_len {
            fields.insert(TAG_ENCODEDTEXTLEN, TagValue::EncodedTextLen(value));
        }
        if let Some(value) = msg.encoded_text {
            fields.insert(TAG_ENCODEDTEXT, TagValue::EncodedText(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::Logout,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Logon {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: String,
    pub appl_ver_id: Option<ApplVerIDEnum>,
    pub appl_ext_id: Option<i64>,
    pub cstm_appl_ver_id: Option<String>,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub on_behalf_of_comp_id: Option<String>,
    pub deliver_to_comp_id: Option<String>,
    pub secure_data_len: Option<i64>,
    pub secure_data: Option<Vec<u8>>,
    pub msg_seq_num: i64,
    pub sender_sub_id: Option<String>,
    pub sender_location_id: Option<String>,
    pub target_sub_id: Option<String>,
    pub target_location_id: Option<String>,
    pub on_behalf_of_sub_id: Option<String>,
    pub on_behalf_of_location_id: Option<String>,
    pub deliver_to_sub_id: Option<String>,
    pub deliver_to_location_id: Option<String>,
    pub poss_dup_flag: Option<PossDupFlagEnum>,
    pub poss_resend: Option<PossResendEnum>,
    pub sending_time: NaiveDateTime,
    pub orig_sending_time: Option<NaiveDateTime>,
    pub xml_data_len: Option<i64>,
    pub xml_data: Option<Vec<u8>>,
    pub message_encoding: Option<String>,
    pub last_msg_seq_num_processed: Option<i64>,
    // Message fields
    pub encrypt_method: EncryptMethodEnum,
    pub heart_bt_int: i64,
    pub raw_data_length: Option<i64>,
    pub raw_data: Option<Vec<u8>>,
    pub reset_seq_num_flag: Option<ResetSeqNumFlagEnum>,
    pub next_expected_msg_seq_num: Option<i64>,
    pub max_message_size: Option<i64>,
    pub test_message_indicator: Option<TestMessageIndicatorEnum>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub new_password: Option<String>,
    pub encrypted_password_method: Option<i64>,
    pub encrypted_password_len: Option<i64>,
    pub encrypted_password: Option<Vec<u8>>,
    pub encrypted_new_password_len: Option<i64>,
    pub encrypted_new_password: Option<Vec<u8>>,
    pub session_status: Option<SessionStatusEnum>,
    pub default_appl_ver_id: String,
    pub default_appl_ext_id: Option<i64>,
    pub default_cstm_appl_ver_id: Option<String>,
    pub text: Option<String>,
    pub encoded_text_len: Option<i64>,
    pub encoded_text: Option<Vec<u8>>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<Logon> for FixMessage {
    fn from(msg: Logon) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
        if let Some(value) = msg.appl_ver_id {
            fields.insert(TAG_APPLVERID, TagValue::ApplVerID(value));
        }
        if let Some(value) = msg.appl_ext_id {
            fields.insert(TAG_APPLEXTID, TagValue::ApplExtID(value));
        }
        if let Some(value) = msg.cstm_appl_ver_id {
            fields.insert(TAG_CSTMAPPLVERID, TagValue::CstmApplVerID(value));
        }
        fields.insert(TAG_SENDERCOMPID, TagValue::SenderCompID(msg.sender_comp_id));
        fields.insert(TAG_TARGETCOMPID, TagValue::TargetCompID(msg.target_comp_id));
        if let Some(value) = msg.on_behalf_of_comp_id {
            fields.insert(TAG_ONBEHALFOFCOMPID, TagValue::OnBehalfOfCompID(value));
        }
        if let Some(value) = msg.deliver_to_comp_id {
            fields.insert(TAG_DELIVERTOCOMPID, TagValue::DeliverToCompID(value));
        }
        if let Some(value) = msg.secure_data_len {
            fields.insert(TAG_SECUREDATALEN, TagValue::SecureDataLen(value));
        }
        if let Some(value) = msg.secure_data {
            fields.insert(TAG_SECUREDATA, TagValue::SecureData(value));
        }
        fields.insert(TAG_MSGSEQNUM, TagValue::MsgSeqNum(msg.msg_seq_num));
        if let Some(value) = msg.sender_sub_id {
            fields.insert(TAG_SENDERSUBID, TagValue::SenderSubID(value));
        }
        if let Some(value) = msg.sender_location_id {
            fields.insert(TAG_SENDERLOCATIONID, TagValue::SenderLocationID(value));
        }
        if let Some(value) = msg.target_sub_id {
            fields.insert(TAG_TARGETSUBID, TagValue::TargetSubID(value));
        }
        if let Some(value) = msg.target_location_id {
            fields.insert(TAG_TARGETLOCATIONID, TagValue::TargetLocationID(value));
        }
        if let Some(value) = msg.on_behalf_of_sub_id {
            fields.insert(TAG_ONBEHALFOFSUBID, TagValue::OnBehalfOfSubID(value));
        }
        if let Some(value) = msg.on_behalf_of_location_id {
            fields.insert(TAG_ONBEHALFOFLOCATIONID, TagValue::OnBehalfOfLocationID(value));
        }
        if let Some(value) = msg.deliver_to_sub_id {
            fields.insert(TAG_DELIVERTOSUBID, TagValue::DeliverToSubID(value));
        }
        if let Some(value) = msg.deliver_to_location_id {
            fields.insert(TAG_DELIVERTOLOCATIONID, TagValue::DeliverToLocationID(value));
        }
        if let Some(value) = msg.poss_dup_flag {
            fields.insert(TAG_POSSDUPFLAG, TagValue::PossDupFlag(value));
        }
        if let Some(value) = msg.poss_resend {
            fields.insert(TAG_POSSRESEND, TagValue::PossResend(value));
        }
        fields.insert(TAG_SENDINGTIME, TagValue::SendingTime(msg.sending_time));
        if let Some(value) = msg.orig_sending_time {
            fields.insert(TAG_ORIGSENDINGTIME, TagValue::OrigSendingTime(value));
        }
        if let Some(value) = msg.xml_data_len {
            fields.insert(TAG_XMLDATALEN, TagValue::XmlDataLen(value));
        }
        if let Some(value) = msg.xml_data {
            fields.insert(TAG_XMLDATA, TagValue::XmlData(value));
        }
        if let Some(value) = msg.message_encoding {
            fields.insert(TAG_MESSAGEENCODING, TagValue::MessageEncoding(value));
        }
        if let Some(value) = msg.last_msg_seq_num_processed {
            fields.insert(TAG_LASTMSGSEQNUMPROCESSED, TagValue::LastMsgSeqNumProcessed(value));
        }
        fields.insert(TAG_ENCRYPTMETHOD, TagValue::EncryptMethod(msg.encrypt_method));
        fields.insert(TAG_HEARTBTINT, TagValue::HeartBtInt(msg.heart_bt_int));
        if let Some(value) = msg.raw_data_length {
            fields.insert(TAG_RAWDATALENGTH, TagValue::RawDataLength(value));
        }
        if let Some(value) = msg.raw_data {
            fields.insert(TAG_RAWDATA, TagValue::RawData(value));
        }
        if let Some(value) = msg.reset_seq_num_flag {
            fields.insert(TAG_RESETSEQNUMFLAG, TagValue::ResetSeqNumFlag(value));
        }
        if let Some(value) = msg.next_expected_msg_seq_num {
            fields.insert(TAG_NEXTEXPECTEDMSGSEQNUM, TagValue::NextExpectedMsgSeqNum(value));
        }
        if let Some(value) = msg.max_message_size {
            fields.insert(TAG_MAXMESSAGESIZE, TagValue::MaxMessageSize(value));
        }
        if let Some(value) = msg.test_message_indicator {
            fields.insert(TAG_TESTMESSAGEINDICATOR, TagValue::TestMessageIndicator(value));
        }
        if let Some(value) = msg.username {
            fields.insert(TAG_USERNAME, TagValue::Username(value));
        }
        if let Some(value) = msg.password {
            fields.insert(TAG_PASSWORD, TagValue::Password(value));
        }
        if let Some(value) = msg.new_password {
            fields.insert(TAG_NEWPASSWORD, TagValue::NewPassword(value));
        }
        if let Some(value) = msg.encrypted_password_method {
            fields.insert(TAG_ENCRYPTEDPASSWORDMETHOD, TagValue::EncryptedPasswordMethod(value));
        }
        if let Some(value) = msg.encrypted_password_len {
            fields.insert(TAG_ENCRYPTEDPASSWORDLEN, TagValue::EncryptedPasswordLen(value));
        }
        if let Some(value) = msg.encrypted_password {
            fields.insert(TAG_ENCRYPTEDPASSWORD, TagValue::EncryptedPassword(value));
        }
        if let Some(value) = msg.encrypted_new_password_len {
            fields.insert(TAG_ENCRYPTEDNEWPASSWORDLEN, TagValue::EncryptedNewPasswordLen(value));
        }
        if let Some(value) = msg.encrypted_new_password {
            fields.insert(TAG_ENCRYPTEDNEWPASSWORD, TagValue::EncryptedNewPassword(value));
        }
        if let Some(value) = msg.session_status {
            fields.insert(TAG_SESSIONSTATUS, TagValue::SessionStatus(value));
        }
        fields.insert(TAG_DEFAULTAPPLVERID, TagValue::DefaultApplVerID(msg.default_appl_ver_id));
        if let Some(value) = msg.default_appl_ext_id {
            fields.insert(TAG_DEFAULTAPPLEXTID, TagValue::DefaultApplExtID(value));
        }
        if let Some(value) = msg.default_cstm_appl_ver_id {
            fields.insert(TAG_DEFAULTCSTMAPPLVERID, TagValue::DefaultCstmApplVerID(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.encoded_text_len {
            fields.insert(TAG_ENCODEDTEXTLEN, TagValue::EncodedTextLen(value));
        }
        if let Some(value) = msg.encoded_text {
            fields.insert(TAG_ENCODEDTEXT, TagValue::EncodedText(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::Logon,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestRequest {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: String,
    pub appl_ver_id: Option<ApplVerIDEnum>,
    pub appl_ext_id: Option<i64>,
    pub cstm_appl_ver_id: Option<String>,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub on_behalf_of_comp_id: Option<String>,
    pub deliver_to_comp_id: Option<String>,
    pub secure_data_len: Option<i64>,
    pub secure_data: Option<Vec<u8>>,
    pub msg_seq_num: i64,
    pub sender_sub_id: Option<String>,
    pub sender_location_id: Option<String>,
    pub target_sub_id: Option<String>,
    pub target_location_id: Option<String>,
    pub on_behalf_of_sub_id: Option<String>,
    pub on_behalf_of_location_id: Option<String>,
    pub deliver_to_sub_id: Option<String>,
    pub deliver_to_location_id: Option<String>,
    pub poss_dup_flag: Option<PossDupFlagEnum>,
    pub poss_resend: Option<PossResendEnum>,
    pub sending_time: NaiveDateTime,
    pub orig_sending_time: Option<NaiveDateTime>,
    pub xml_data_len: Option<i64>,
    pub xml_data: Option<Vec<u8>>,
    pub message_encoding: Option<String>,
    pub last_msg_seq_num_processed: Option<i64>,
    // Message fields
    pub test_req_id: String,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<TestRequest> for FixMessage {
    fn from(msg: TestRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
        if let Some(value) = msg.appl_ver_id {
            fields.insert(TAG_APPLVERID, TagValue::ApplVerID(value));
        }
        if let Some(value) = msg.appl_ext_id {
            fields.insert(TAG_APPLEXTID, TagValue::ApplExtID(value));
        }
        if let Some(value) = msg.cstm_appl_ver_id {
            fields.insert(TAG_CSTMAPPLVERID, TagValue::CstmApplVerID(value));
        }
        fields.insert(TAG_SENDERCOMPID, TagValue::SenderCompID(msg.sender_comp_id));
        fields.insert(TAG_TARGETCOMPID, TagValue::TargetCompID(msg.target_comp_id));
        if let Some(value) = msg.on_behalf_of_comp_id {
            fields.insert(TAG_ONBEHALFOFCOMPID, TagValue::OnBehalfOfCompID(value));
        }
        if let Some(value) = msg.deliver_to_comp_id {
            fields.insert(TAG_DELIVERTOCOMPID, TagValue::DeliverToCompID(value));
        }
        if let Some(value) = msg.secure_data_len {
            fields.insert(TAG_SECUREDATALEN, TagValue::SecureDataLen(value));
        }
        if let Some(value) = msg.secure_data {
            fields.insert(TAG_SECUREDATA, TagValue::SecureData(value));
        }
        fields.insert(TAG_MSGSEQNUM, TagValue::MsgSeqNum(msg.msg_seq_num));
        if let Some(value) = msg.sender_sub_id {
            fields.insert(TAG_SENDERSUBID, TagValue::SenderSubID(value));
        }
        if let Some(value) = msg.sender_location_id {
            fields.insert(TAG_SENDERLOCATIONID, TagValue::SenderLocationID(value));
        }
        if let Some(value) = msg.target_sub_id {
            fields.insert(TAG_TARGETSUBID, TagValue::TargetSubID(value));
        }
        if let Some(value) = msg.target_location_id {
            fields.insert(TAG_TARGETLOCATIONID, TagValue::TargetLocationID(value));
        }
        if let Some(value) = msg.on_behalf_of_sub_id {
            fields.insert(TAG_ONBEHALFOFSUBID, TagValue::OnBehalfOfSubID(value));
        }
        if let Some(value) = msg.on_behalf_of_location_id {
            fields.insert(TAG_ONBEHALFOFLOCATIONID, TagValue::OnBehalfOfLocationID(value));
        }
        if let Some(value) = msg.deliver_to_sub_id {
            fields.insert(TAG_DELIVERTOSUBID, TagValue::DeliverToSubID(value));
        }
        if let Some(value) = msg.deliver_to_location_id {
            fields.insert(TAG_DELIVERTOLOCATIONID, TagValue::DeliverToLocationID(value));
        }
        if let Some(value) = msg.poss_dup_flag {
            fields.insert(TAG_POSSDUPFLAG, TagValue::PossDupFlag(value));
        }
        if let Some(value) = msg.poss_resend {
            fields.insert(TAG_POSSRESEND, TagValue::PossResend(value));
        }
        fields.insert(TAG_SENDINGTIME, TagValue::SendingTime(msg.sending_time));
        if let Some(value) = msg.orig_sending_time {
            fields.insert(TAG_ORIGSENDINGTIME, TagValue::OrigSendingTime(value));
        }
        if let Some(value) = msg.xml_data_len {
            fields.insert(TAG_XMLDATALEN, TagValue::XmlDataLen(value));
        }
        if let Some(value) = msg.xml_data {
            fields.insert(TAG_XMLDATA, TagValue::XmlData(value));
        }
        if let Some(value) = msg.message_encoding {
            fields.insert(TAG_MESSAGEENCODING, TagValue::MessageEncoding(value));
        }
        if let Some(value) = msg.last_msg_seq_num_processed {
            fields.insert(TAG_LASTMSGSEQNUMPROCESSED, TagValue::LastMsgSeqNumProcessed(value));
        }
        fields.insert(TAG_TESTREQID, TagValue::TestReqID(msg.test_req_id));
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::TestRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Reject {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: String,
    pub appl_ver_id: Option<ApplVerIDEnum>,
    pub appl_ext_id: Option<i64>,
    pub cstm_appl_ver_id: Option<String>,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub on_behalf_of_comp_id: Option<String>,
    pub deliver_to_comp_id: Option<String>,
    pub secure_data_len: Option<i64>,
    pub secure_data: Option<Vec<u8>>,
    pub msg_seq_num: i64,
    pub sender_sub_id: Option<String>,
    pub sender_location_id: Option<String>,
    pub target_sub_id: Option<String>,
    pub target_location_id: Option<String>,
    pub on_behalf_of_sub_id: Option<String>,
    pub on_behalf_of_location_id: Option<String>,
    pub deliver_to_sub_id: Option<String>,
    pub deliver_to_location_id: Option<String>,
    pub poss_dup_flag: Option<PossDupFlagEnum>,
    pub poss_resend: Option<PossResendEnum>,
    pub sending_time: NaiveDateTime,
    pub orig_sending_time: Option<NaiveDateTime>,
    pub xml_data_len: Option<i64>,
    pub xml_data: Option<Vec<u8>>,
    pub message_encoding: Option<String>,
    pub last_msg_seq_num_processed: Option<i64>,
    // Message fields
    pub ref_seq_num: i64,
    pub ref_tag_id: Option<i64>,
    pub ref_msg_type: Option<String>,
    pub ref_appl_ver_id: Option<String>,
    pub ref_appl_ext_id: Option<i64>,
    pub ref_cstm_appl_ver_id: Option<String>,
    pub session_reject_reason: Option<SessionRejectReasonEnum>,
    pub text: Option<String>,
    pub encoded_text_len: Option<i64>,
    pub encoded_text: Option<Vec<u8>>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<Reject> for FixMessage {
    fn from(msg: Reject) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
        if let Some(value) = msg.appl_ver_id {
            fields.insert(TAG_APPLVERID, TagValue::ApplVerID(value));
        }
        if let Some(value) = msg.appl_ext_id {
            fields.insert(TAG_APPLEXTID, TagValue::ApplExtID(value));
        }
        if let Some(value) = msg.cstm_appl_ver_id {
            fields.insert(TAG_CSTMAPPLVERID, TagValue::CstmApplVerID(value));
        }
        fields.insert(TAG_SENDERCOMPID, TagValue::SenderCompID(msg.sender_comp_id));
        fields.insert(TAG_TARGETCOMPID, TagValue::TargetCompID(msg.target_comp_id));
        if let Some(value) = msg.on_behalf_of_comp_id {
            fields.insert(TAG_ONBEHALFOFCOMPID, TagValue::OnBehalfOfCompID(value));
        }
        if let Some(value) = msg.deliver_to_comp_id {
            fields.insert(TAG_DELIVERTOCOMPID, TagValue::DeliverToCompID(value));
        }
        if let Some(value) = msg.secure_data_len {
            fields.insert(TAG_SECUREDATALEN, TagValue::SecureDataLen(value));
        }
        if let Some(value) = msg.secure_data {
            fields.insert(TAG_SECUREDATA, TagValue::SecureData(value));
        }
        fields.insert(TAG_MSGSEQNUM, TagValue::MsgSeqNum(msg.msg_seq_num));
        if let Some(value) = msg.sender_sub_id {
            fields.insert(TAG_SENDERSUBID, TagValue::SenderSubID(value));
        }
        if let Some(value) = msg.sender_location_id {
            fields.insert(TAG_SENDERLOCATIONID, TagValue::SenderLocationID(value));
        }
        if let Some(value) = msg.target_sub_id {
            fields.insert(TAG_TARGETSUBID, TagValue::TargetSubID(value));
        }
        if let Some(value) = msg.target_location_id {
            fields.insert(TAG_TARGETLOCATIONID, TagValue::TargetLocationID(value));
        }
        if let Some(value) = msg.on_behalf_of_sub_id {
            fields.insert(TAG_ONBEHALFOFSUBID, TagValue::OnBehalfOfSubID(value));
        }
        if let Some(value) = msg.on_behalf_of_location_id {
            fields.insert(TAG_ONBEHALFOFLOCATIONID, TagValue::OnBehalfOfLocationID(value));
        }
        if let Some(value) = msg.deliver_to_sub_id {
            fields.insert(TAG_DELIVERTOSUBID, TagValue::DeliverToSubID(value));
        }
        if let Some(value) = msg.deliver_to_location_id {
            fields.insert(TAG_DELIVERTOLOCATIONID, TagValue::DeliverToLocationID(value));
        }
        if let Some(value) = msg.poss_dup_flag {
            fields.insert(TAG_POSSDUPFLAG, TagValue::PossDupFlag(value));
        }
        if let Some(value) = msg.poss_resend {
            fields.insert(TAG_POSSRESEND, TagValue::PossResend(value));
        }
        fields.insert(TAG_SENDINGTIME, TagValue::SendingTime(msg.sending_time));
        if let Some(value) = msg.orig_sending_time {
            fields.insert(TAG_ORIGSENDINGTIME, TagValue::OrigSendingTime(value));
        }
        if let Some(value) = msg.xml_data_len {
            fields.insert(TAG_XMLDATALEN, TagValue::XmlDataLen(value));
        }
        if let Some(value) = msg.xml_data {
            fields.insert(TAG_XMLDATA, TagValue::XmlData(value));
        }
        if let Some(value) = msg.message_encoding {
            fields.insert(TAG_MESSAGEENCODING, TagValue::MessageEncoding(value));
        }
        if let Some(value) = msg.last_msg_seq_num_processed {
            fields.insert(TAG_LASTMSGSEQNUMPROCESSED, TagValue::LastMsgSeqNumProcessed(value));
        }
        fields.insert(TAG_REFSEQNUM, TagValue::RefSeqNum(msg.ref_seq_num));
        if let Some(value) = msg.ref_tag_id {
            fields.insert(TAG_REFTAGID, TagValue::RefTagID(value));
        }
        if let Some(value) = msg.ref_msg_type {
            fields.insert(TAG_REFMSGTYPE, TagValue::RefMsgType(value));
        }
        if let Some(value) = msg.ref_appl_ver_id {
            fields.insert(TAG_REFAPPLVERID, TagValue::RefApplVerID(value));
        }
        if let Some(value) = msg.ref_appl_ext_id {
            fields.insert(TAG_REFAPPLEXTID, TagValue::RefApplExtID(value));
        }
        if let Some(value) = msg.ref_cstm_appl_ver_id {
            fields.insert(TAG_REFCSTMAPPLVERID, TagValue::RefCstmApplVerID(value));
        }
        if let Some(value) = msg.session_reject_reason {
            fields.insert(TAG_SESSIONREJECTREASON, TagValue::SessionRejectReason(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.encoded_text_len {
            fields.insert(TAG_ENCODEDTEXTLEN, TagValue::EncodedTextLen(value));
        }
        if let Some(value) = msg.encoded_text {
            fields.insert(TAG_ENCODEDTEXT, TagValue::EncodedText(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::Reject,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResendRequest {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: String,
    pub appl_ver_id: Option<ApplVerIDEnum>,
    pub appl_ext_id: Option<i64>,
    pub cstm_appl_ver_id: Option<String>,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub on_behalf_of_comp_id: Option<String>,
    pub deliver_to_comp_id: Option<String>,
    pub secure_data_len: Option<i64>,
    pub secure_data: Option<Vec<u8>>,
    pub msg_seq_num: i64,
    pub sender_sub_id: Option<String>,
    pub sender_location_id: Option<String>,
    pub target_sub_id: Option<String>,
    pub target_location_id: Option<String>,
    pub on_behalf_of_sub_id: Option<String>,
    pub on_behalf_of_location_id: Option<String>,
    pub deliver_to_sub_id: Option<String>,
    pub deliver_to_location_id: Option<String>,
    pub poss_dup_flag: Option<PossDupFlagEnum>,
    pub poss_resend: Option<PossResendEnum>,
    pub sending_time: NaiveDateTime,
    pub orig_sending_time: Option<NaiveDateTime>,
    pub xml_data_len: Option<i64>,
    pub xml_data: Option<Vec<u8>>,
    pub message_encoding: Option<String>,
    pub last_msg_seq_num_processed: Option<i64>,
    // Message fields
    pub begin_seq_no: i64,
    pub end_seq_no: i64,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<ResendRequest> for FixMessage {
    fn from(msg: ResendRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
        if let Some(value) = msg.appl_ver_id {
            fields.insert(TAG_APPLVERID, TagValue::ApplVerID(value));
        }
        if let Some(value) = msg.appl_ext_id {
            fields.insert(TAG_APPLEXTID, TagValue::ApplExtID(value));
        }
        if let Some(value) = msg.cstm_appl_ver_id {
            fields.insert(TAG_CSTMAPPLVERID, TagValue::CstmApplVerID(value));
        }
        fields.insert(TAG_SENDERCOMPID, TagValue::SenderCompID(msg.sender_comp_id));
        fields.insert(TAG_TARGETCOMPID, TagValue::TargetCompID(msg.target_comp_id));
        if let Some(value) = msg.on_behalf_of_comp_id {
            fields.insert(TAG_ONBEHALFOFCOMPID, TagValue::OnBehalfOfCompID(value));
        }
        if let Some(value) = msg.deliver_to_comp_id {
            fields.insert(TAG_DELIVERTOCOMPID, TagValue::DeliverToCompID(value));
        }
        if let Some(value) = msg.secure_data_len {
            fields.insert(TAG_SECUREDATALEN, TagValue::SecureDataLen(value));
        }
        if let Some(value) = msg.secure_data {
            fields.insert(TAG_SECUREDATA, TagValue::SecureData(value));
        }
        fields.insert(TAG_MSGSEQNUM, TagValue::MsgSeqNum(msg.msg_seq_num));
        if let Some(value) = msg.sender_sub_id {
            fields.insert(TAG_SENDERSUBID, TagValue::SenderSubID(value));
        }
        if let Some(value) = msg.sender_location_id {
            fields.insert(TAG_SENDERLOCATIONID, TagValue::SenderLocationID(value));
        }
        if let Some(value) = msg.target_sub_id {
            fields.insert(TAG_TARGETSUBID, TagValue::TargetSubID(value));
        }
        if let Some(value) = msg.target_location_id {
            fields.insert(TAG_TARGETLOCATIONID, TagValue::TargetLocationID(value));
        }
        if let Some(value) = msg.on_behalf_of_sub_id {
            fields.insert(TAG_ONBEHALFOFSUBID, TagValue::OnBehalfOfSubID(value));
        }
        if let Some(value) = msg.on_behalf_of_location_id {
            fields.insert(TAG_ONBEHALFOFLOCATIONID, TagValue::OnBehalfOfLocationID(value));
        }
        if let Some(value) = msg.deliver_to_sub_id {
            fields.insert(TAG_DELIVERTOSUBID, TagValue::DeliverToSubID(value));
        }
        if let Some(value) = msg.deliver_to_location_id {
            fields.insert(TAG_DELIVERTOLOCATIONID, TagValue::DeliverToLocationID(value));
        }
        if let Some(value) = msg.poss_dup_flag {
            fields.insert(TAG_POSSDUPFLAG, TagValue::PossDupFlag(value));
        }
        if let Some(value) = msg.poss_resend {
            fields.insert(TAG_POSSRESEND, TagValue::PossResend(value));
        }
        fields.insert(TAG_SENDINGTIME, TagValue::SendingTime(msg.sending_time));
        if let Some(value) = msg.orig_sending_time {
            fields.insert(TAG_ORIGSENDINGTIME, TagValue::OrigSendingTime(value));
        }
        if let Some(value) = msg.xml_data_len {
            fields.insert(TAG_XMLDATALEN, TagValue::XmlDataLen(value));
        }
        if let Some(value) = msg.xml_data {
            fields.insert(TAG_XMLDATA, TagValue::XmlData(value));
        }
        if let Some(value) = msg.message_encoding {
            fields.insert(TAG_MESSAGEENCODING, TagValue::MessageEncoding(value));
        }
        if let Some(value) = msg.last_msg_seq_num_processed {
            fields.insert(TAG_LASTMSGSEQNUMPROCESSED, TagValue::LastMsgSeqNumProcessed(value));
        }
        fields.insert(TAG_BEGINSEQNO, TagValue::BeginSeqNo(msg.begin_seq_no));
        fields.insert(TAG_ENDSEQNO, TagValue::EndSeqNo(msg.end_seq_no));
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::ResendRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SequenceReset {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: String,
    pub appl_ver_id: Option<ApplVerIDEnum>,
    pub appl_ext_id: Option<i64>,
    pub cstm_appl_ver_id: Option<String>,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub on_behalf_of_comp_id: Option<String>,
    pub deliver_to_comp_id: Option<String>,
    pub secure_data_len: Option<i64>,
    pub secure_data: Option<Vec<u8>>,
    pub msg_seq_num: i64,
    pub sender_sub_id: Option<String>,
    pub sender_location_id: Option<String>,
    pub target_sub_id: Option<String>,
    pub target_location_id: Option<String>,
    pub on_behalf_of_sub_id: Option<String>,
    pub on_behalf_of_location_id: Option<String>,
    pub deliver_to_sub_id: Option<String>,
    pub deliver_to_location_id: Option<String>,
    pub poss_dup_flag: Option<PossDupFlagEnum>,
    pub poss_resend: Option<PossResendEnum>,
    pub sending_time: NaiveDateTime,
    pub orig_sending_time: Option<NaiveDateTime>,
    pub xml_data_len: Option<i64>,
    pub xml_data: Option<Vec<u8>>,
    pub message_encoding: Option<String>,
    pub last_msg_seq_num_processed: Option<i64>,
    // Message fields
    pub gap_fill_flag: Option<GapFillFlagEnum>,
    pub new_seq_no: i64,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<SequenceReset> for FixMessage {
    fn from(msg: SequenceReset) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
        if let Some(value) = msg.appl_ver_id {
            fields.insert(TAG_APPLVERID, TagValue::ApplVerID(value));
        }
        if let Some(value) = msg.appl_ext_id {
            fields.insert(TAG_APPLEXTID, TagValue::ApplExtID(value));
        }
        if let Some(value) = msg.cstm_appl_ver_id {
            fields.insert(TAG_CSTMAPPLVERID, TagValue::CstmApplVerID(value));
        }
        fields.insert(TAG_SENDERCOMPID, TagValue::SenderCompID(msg.sender_comp_id));
        fields.insert(TAG_TARGETCOMPID, TagValue::TargetCompID(msg.target_comp_id));
        if let Some(value) = msg.on_behalf_of_comp_id {
            fields.insert(TAG_ONBEHALFOFCOMPID, TagValue::OnBehalfOfCompID(value));
        }
        if let Some(value) = msg.deliver_to_comp_id {
            fields.insert(TAG_DELIVERTOCOMPID, TagValue::DeliverToCompID(value));
        }
        if let Some(value) = msg.secure_data_len {
            fields.insert(TAG_SECUREDATALEN, TagValue::SecureDataLen(value));
        }
        if let Some(value) = msg.secure_data {
            fields.insert(TAG_SECUREDATA, TagValue::SecureData(value));
        }
        fields.insert(TAG_MSGSEQNUM, TagValue::MsgSeqNum(msg.msg_seq_num));
        if let Some(value) = msg.sender_sub_id {
            fields.insert(TAG_SENDERSUBID, TagValue::SenderSubID(value));
        }
        if let Some(value) = msg.sender_location_id {
            fields.insert(TAG_SENDERLOCATIONID, TagValue::SenderLocationID(value));
        }
        if let Some(value) = msg.target_sub_id {
            fields.insert(TAG_TARGETSUBID, TagValue::TargetSubID(value));
        }
        if let Some(value) = msg.target_location_id {
            fields.insert(TAG_TARGETLOCATIONID, TagValue::TargetLocationID(value));
        }
        if let Some(value) = msg.on_behalf_of_sub_id {
            fields.insert(TAG_ONBEHALFOFSUBID, TagValue::OnBehalfOfSubID(value));
        }
        if let Some(value) = msg.on_behalf_of_location_id {
            fields.insert(TAG_ONBEHALFOFLOCATIONID, TagValue::OnBehalfOfLocationID(value));
        }
        if let Some(value) = msg.deliver_to_sub_id {
            fields.insert(TAG_DELIVERTOSUBID, TagValue::DeliverToSubID(value));
        }
        if let Some(value) = msg.deliver_to_location_id {
            fields.insert(TAG_DELIVERTOLOCATIONID, TagValue::DeliverToLocationID(value));
        }
        if let Some(value) = msg.poss_dup_flag {
            fields.insert(TAG_POSSDUPFLAG, TagValue::PossDupFlag(value));
        }
        if let Some(value) = msg.poss_resend {
            fields.insert(TAG_POSSRESEND, TagValue::PossResend(value));
        }
        fields.insert(TAG_SENDINGTIME, TagValue::SendingTime(msg.sending_time));
        if let Some(value) = msg.orig_sending_time {
            fields.insert(TAG_ORIGSENDINGTIME, TagValue::OrigSendingTime(value));
        }
        if let Some(value) = msg.xml_data_len {
            fields.insert(TAG_XMLDATALEN, TagValue::XmlDataLen(value));
        }
        if let Some(value) = msg.xml_data {
            fields.insert(TAG_XMLDATA, TagValue::XmlData(value));
        }
        if let Some(value) = msg.message_encoding {
            fields.insert(TAG_MESSAGEENCODING, TagValue::MessageEncoding(value));
        }
        if let Some(value) = msg.last_msg_seq_num_processed {
            fields.insert(TAG_LASTMSGSEQNUMPROCESSED, TagValue::LastMsgSeqNumProcessed(value));
        }
        if let Some(value) = msg.gap_fill_flag {
            fields.insert(TAG_GAPFILLFLAG, TagValue::GapFillFlag(value));
        }
        fields.insert(TAG_NEWSEQNO, TagValue::NewSeqNo(msg.new_seq_no));
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::SequenceReset,
            fields,
        }
    }
}

