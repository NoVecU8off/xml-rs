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
pub enum AdvTransTypeEnum {
    /// CANCEL
    CANCEL,
    /// NEW
    NEW,
    /// REPLACE
    REPLACE,
    /// Unknown value
    Unknown(String),
}

impl From<String> for AdvTransTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "C" => AdvTransTypeEnum::CANCEL,
            "N" => AdvTransTypeEnum::NEW,
            "R" => AdvTransTypeEnum::REPLACE,
            _ => AdvTransTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for AdvTransTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdvTransTypeEnum::CANCEL => write!(f, "C"),
            AdvTransTypeEnum::NEW => write!(f, "N"),
            AdvTransTypeEnum::REPLACE => write!(f, "R"),
            AdvTransTypeEnum::Unknown(s) => write!(f, "{s}"),
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
pub enum OpenCloseEnum {
    /// CLOSE
    CLOSE,
    /// OPEN
    OPEN,
    /// Unknown value
    Unknown(String),
}

impl From<String> for OpenCloseEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "C" => OpenCloseEnum::CLOSE,
            "O" => OpenCloseEnum::OPEN,
            _ => OpenCloseEnum::Unknown(s),
        }
    }
}

impl fmt::Display for OpenCloseEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenCloseEnum::CLOSE => write!(f, "C"),
            OpenCloseEnum::OPEN => write!(f, "O"),
            OpenCloseEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SettlmntTypEnum {
    /// REGULAR
    REGULAR,
    /// CASH
    CASH,
    /// NEXT_DAY
    NEXT_DAY,
    /// T_PLUS2
    T_PLUS2,
    /// T_PLUS3
    T_PLUS3,
    /// T_PLUS4
    T_PLUS4,
    /// FUTURE
    FUTURE,
    /// WHEN_AND_IF_ISSUED
    WHEN_AND_IF_ISSUED,
    /// SELLERS_OPTION
    SELLERS_OPTION,
    /// T_PLUS5
    T_PLUS5,
    /// Unknown value
    Unknown(String),
}

impl From<String> for SettlmntTypEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => SettlmntTypEnum::REGULAR,
            "1" => SettlmntTypEnum::CASH,
            "2" => SettlmntTypEnum::NEXT_DAY,
            "3" => SettlmntTypEnum::T_PLUS2,
            "4" => SettlmntTypEnum::T_PLUS3,
            "5" => SettlmntTypEnum::T_PLUS4,
            "6" => SettlmntTypEnum::FUTURE,
            "7" => SettlmntTypEnum::WHEN_AND_IF_ISSUED,
            "8" => SettlmntTypEnum::SELLERS_OPTION,
            "9" => SettlmntTypEnum::T_PLUS5,
            _ => SettlmntTypEnum::Unknown(s),
        }
    }
}

impl fmt::Display for SettlmntTypEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettlmntTypEnum::REGULAR => write!(f, "0"),
            SettlmntTypEnum::CASH => write!(f, "1"),
            SettlmntTypEnum::NEXT_DAY => write!(f, "2"),
            SettlmntTypEnum::T_PLUS2 => write!(f, "3"),
            SettlmntTypEnum::T_PLUS3 => write!(f, "4"),
            SettlmntTypEnum::T_PLUS4 => write!(f, "5"),
            SettlmntTypEnum::FUTURE => write!(f, "6"),
            SettlmntTypEnum::WHEN_AND_IF_ISSUED => write!(f, "7"),
            SettlmntTypEnum::SELLERS_OPTION => write!(f, "8"),
            SettlmntTypEnum::T_PLUS5 => write!(f, "9"),
            SettlmntTypEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SecurityTypeEnum {
    /// BANKERS_ACCEPTANCE
    BANKERS_ACCEPTANCE,
    /// CERTIFICATE_OF_DEPOSIT
    CERTIFICATE_OF_DEPOSIT,
    /// COLLATERALIZED_MORTGAGE_OBLIGATION
    COLLATERALIZED_MORTGAGE_OBLIGATION,
    /// CORPORATE_BOND
    CORPORATE_BOND,
    /// COMMERCIAL_PAPER
    COMMERCIAL_PAPER,
    /// CORPORATE_PRIVATE_PLACEMENT
    CORPORATE_PRIVATE_PLACEMENT,
    /// COMMON_STOCK
    COMMON_STOCK,
    /// FEDERAL_HOUSING_AUTHORITY
    FEDERAL_HOUSING_AUTHORITY,
    /// FEDERAL_HOME_LOAN
    FEDERAL_HOME_LOAN,
    /// FEDERAL_NATIONAL_MORTGAGE_ASSOCIATION
    FEDERAL_NATIONAL_MORTGAGE_ASSOCIATION,
    /// FOREIGN_EXCHANGE_CONTRACT
    FOREIGN_EXCHANGE_CONTRACT,
    /// FUTURE
    FUTURE,
    /// GOVERNMENT_NATIONAL_MORTGAGE_ASSOCIATION
    GOVERNMENT_NATIONAL_MORTGAGE_ASSOCIATION,
    /// TREASURIES_AGENCY_DEBENTURE
    TREASURIES_AGENCY_DEBENTURE,
    /// MUTUAL_FUND
    MUTUAL_FUND,
    /// MORTGAGE_INTEREST_ONLY
    MORTGAGE_INTEREST_ONLY,
    /// MORTGAGE_PRINCIPAL_ONLY
    MORTGAGE_PRINCIPAL_ONLY,
    /// MORTGAGE_PRIVATE_PLACEMENT
    MORTGAGE_PRIVATE_PLACEMENT,
    /// MISCELLANEOUS_PASS_THROUGH
    MISCELLANEOUS_PASS_THROUGH,
    /// MUNICIPAL_BOND
    MUNICIPAL_BOND,
    /// NO_SECURITY_TYPE
    NO_SECURITY_TYPE,
    /// OPTION
    OPTION,
    /// PREFERRED_STOCK
    PREFERRED_STOCK,
    /// REPURCHASE_AGREEMENT
    REPURCHASE_AGREEMENT,
    /// REVERSE_REPURCHASE_AGREEMENT
    REVERSE_REPURCHASE_AGREEMENT,
    /// STUDENT_LOAN_MARKETING_ASSOCIATION
    STUDENT_LOAN_MARKETING_ASSOCIATION,
    /// TIME_DEPOSIT
    TIME_DEPOSIT,
    /// US_TREASURY_BILL_OLD
    US_TREASURY_BILL_OLD,
    /// WARRANT
    WARRANT,
    /// CATS_TIGERS_AND_LIONS
    CATS_TIGERS_AND_LIONS,
    /// Unknown value
    Unknown(String),
}

impl From<String> for SecurityTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "BA" => SecurityTypeEnum::BANKERS_ACCEPTANCE,
            "CD" => SecurityTypeEnum::CERTIFICATE_OF_DEPOSIT,
            "CMO" => SecurityTypeEnum::COLLATERALIZED_MORTGAGE_OBLIGATION,
            "CORP" => SecurityTypeEnum::CORPORATE_BOND,
            "CP" => SecurityTypeEnum::COMMERCIAL_PAPER,
            "CPP" => SecurityTypeEnum::CORPORATE_PRIVATE_PLACEMENT,
            "CS" => SecurityTypeEnum::COMMON_STOCK,
            "FHA" => SecurityTypeEnum::FEDERAL_HOUSING_AUTHORITY,
            "FHL" => SecurityTypeEnum::FEDERAL_HOME_LOAN,
            "FN" => SecurityTypeEnum::FEDERAL_NATIONAL_MORTGAGE_ASSOCIATION,
            "FOR" => SecurityTypeEnum::FOREIGN_EXCHANGE_CONTRACT,
            "FUT" => SecurityTypeEnum::FUTURE,
            "GN" => SecurityTypeEnum::GOVERNMENT_NATIONAL_MORTGAGE_ASSOCIATION,
            "GOVT" => SecurityTypeEnum::TREASURIES_AGENCY_DEBENTURE,
            "MF" => SecurityTypeEnum::MUTUAL_FUND,
            "MIO" => SecurityTypeEnum::MORTGAGE_INTEREST_ONLY,
            "MPO" => SecurityTypeEnum::MORTGAGE_PRINCIPAL_ONLY,
            "MPP" => SecurityTypeEnum::MORTGAGE_PRIVATE_PLACEMENT,
            "MPT" => SecurityTypeEnum::MISCELLANEOUS_PASS_THROUGH,
            "MUNI" => SecurityTypeEnum::MUNICIPAL_BOND,
            "NONE" => SecurityTypeEnum::NO_SECURITY_TYPE,
            "OPT" => SecurityTypeEnum::OPTION,
            "PS" => SecurityTypeEnum::PREFERRED_STOCK,
            "RP" => SecurityTypeEnum::REPURCHASE_AGREEMENT,
            "RVRP" => SecurityTypeEnum::REVERSE_REPURCHASE_AGREEMENT,
            "SL" => SecurityTypeEnum::STUDENT_LOAN_MARKETING_ASSOCIATION,
            "TD" => SecurityTypeEnum::TIME_DEPOSIT,
            "USTB" => SecurityTypeEnum::US_TREASURY_BILL_OLD,
            "WAR" => SecurityTypeEnum::WARRANT,
            "ZOO" => SecurityTypeEnum::CATS_TIGERS_AND_LIONS,
            _ => SecurityTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for SecurityTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityTypeEnum::BANKERS_ACCEPTANCE => write!(f, "BA"),
            SecurityTypeEnum::CERTIFICATE_OF_DEPOSIT => write!(f, "CD"),
            SecurityTypeEnum::COLLATERALIZED_MORTGAGE_OBLIGATION => write!(f, "CMO"),
            SecurityTypeEnum::CORPORATE_BOND => write!(f, "CORP"),
            SecurityTypeEnum::COMMERCIAL_PAPER => write!(f, "CP"),
            SecurityTypeEnum::CORPORATE_PRIVATE_PLACEMENT => write!(f, "CPP"),
            SecurityTypeEnum::COMMON_STOCK => write!(f, "CS"),
            SecurityTypeEnum::FEDERAL_HOUSING_AUTHORITY => write!(f, "FHA"),
            SecurityTypeEnum::FEDERAL_HOME_LOAN => write!(f, "FHL"),
            SecurityTypeEnum::FEDERAL_NATIONAL_MORTGAGE_ASSOCIATION => write!(f, "FN"),
            SecurityTypeEnum::FOREIGN_EXCHANGE_CONTRACT => write!(f, "FOR"),
            SecurityTypeEnum::FUTURE => write!(f, "FUT"),
            SecurityTypeEnum::GOVERNMENT_NATIONAL_MORTGAGE_ASSOCIATION => write!(f, "GN"),
            SecurityTypeEnum::TREASURIES_AGENCY_DEBENTURE => write!(f, "GOVT"),
            SecurityTypeEnum::MUTUAL_FUND => write!(f, "MF"),
            SecurityTypeEnum::MORTGAGE_INTEREST_ONLY => write!(f, "MIO"),
            SecurityTypeEnum::MORTGAGE_PRINCIPAL_ONLY => write!(f, "MPO"),
            SecurityTypeEnum::MORTGAGE_PRIVATE_PLACEMENT => write!(f, "MPP"),
            SecurityTypeEnum::MISCELLANEOUS_PASS_THROUGH => write!(f, "MPT"),
            SecurityTypeEnum::MUNICIPAL_BOND => write!(f, "MUNI"),
            SecurityTypeEnum::NO_SECURITY_TYPE => write!(f, "NONE"),
            SecurityTypeEnum::OPTION => write!(f, "OPT"),
            SecurityTypeEnum::PREFERRED_STOCK => write!(f, "PS"),
            SecurityTypeEnum::REPURCHASE_AGREEMENT => write!(f, "RP"),
            SecurityTypeEnum::REVERSE_REPURCHASE_AGREEMENT => write!(f, "RVRP"),
            SecurityTypeEnum::STUDENT_LOAN_MARKETING_ASSOCIATION => write!(f, "SL"),
            SecurityTypeEnum::TIME_DEPOSIT => write!(f, "TD"),
            SecurityTypeEnum::US_TREASURY_BILL_OLD => write!(f, "USTB"),
            SecurityTypeEnum::WARRANT => write!(f, "WAR"),
            SecurityTypeEnum::CATS_TIGERS_AND_LIONS => write!(f, "ZOO"),
            SecurityTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum DKReasonEnum {
    /// UNKNOWN_SYMBOL
    UNKNOWN_SYMBOL,
    /// WRONG_SIDE
    WRONG_SIDE,
    /// QUANTITY_EXCEEDS_ORDER
    QUANTITY_EXCEEDS_ORDER,
    /// NO_MATCHING_ORDER
    NO_MATCHING_ORDER,
    /// PRICE_EXCEEDS_LIMIT
    PRICE_EXCEEDS_LIMIT,
    /// OTHER
    OTHER,
    /// Unknown value
    Unknown(String),
}

impl From<String> for DKReasonEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "A" => DKReasonEnum::UNKNOWN_SYMBOL,
            "B" => DKReasonEnum::WRONG_SIDE,
            "C" => DKReasonEnum::QUANTITY_EXCEEDS_ORDER,
            "D" => DKReasonEnum::NO_MATCHING_ORDER,
            "E" => DKReasonEnum::PRICE_EXCEEDS_LIMIT,
            "Z" => DKReasonEnum::OTHER,
            _ => DKReasonEnum::Unknown(s),
        }
    }
}

impl fmt::Display for DKReasonEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DKReasonEnum::UNKNOWN_SYMBOL => write!(f, "A"),
            DKReasonEnum::WRONG_SIDE => write!(f, "B"),
            DKReasonEnum::QUANTITY_EXCEEDS_ORDER => write!(f, "C"),
            DKReasonEnum::NO_MATCHING_ORDER => write!(f, "D"),
            DKReasonEnum::PRICE_EXCEEDS_LIMIT => write!(f, "E"),
            DKReasonEnum::OTHER => write!(f, "Z"),
            DKReasonEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ExecTypeEnum {
    /// NEW
    NEW,
    /// PARTIAL_FILL
    PARTIAL_FILL,
    /// FILL
    FILL,
    /// DONE_FOR_DAY
    DONE_FOR_DAY,
    /// CANCELED
    CANCELED,
    /// REPLACED
    REPLACED,
    /// PENDING_CANCEL
    PENDING_CANCEL,
    /// STOPPED
    STOPPED,
    /// REJECTED
    REJECTED,
    /// SUSPENDED
    SUSPENDED,
    /// PENDING_NEW
    PENDING_NEW,
    /// CALCULATED
    CALCULATED,
    /// EXPIRED
    EXPIRED,
    /// Unknown value
    Unknown(String),
}

impl From<String> for ExecTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => ExecTypeEnum::NEW,
            "1" => ExecTypeEnum::PARTIAL_FILL,
            "2" => ExecTypeEnum::FILL,
            "3" => ExecTypeEnum::DONE_FOR_DAY,
            "4" => ExecTypeEnum::CANCELED,
            "5" => ExecTypeEnum::REPLACED,
            "6" => ExecTypeEnum::PENDING_CANCEL,
            "7" => ExecTypeEnum::STOPPED,
            "8" => ExecTypeEnum::REJECTED,
            "9" => ExecTypeEnum::SUSPENDED,
            "A" => ExecTypeEnum::PENDING_NEW,
            "B" => ExecTypeEnum::CALCULATED,
            "C" => ExecTypeEnum::EXPIRED,
            _ => ExecTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for ExecTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecTypeEnum::NEW => write!(f, "0"),
            ExecTypeEnum::PARTIAL_FILL => write!(f, "1"),
            ExecTypeEnum::FILL => write!(f, "2"),
            ExecTypeEnum::DONE_FOR_DAY => write!(f, "3"),
            ExecTypeEnum::CANCELED => write!(f, "4"),
            ExecTypeEnum::REPLACED => write!(f, "5"),
            ExecTypeEnum::PENDING_CANCEL => write!(f, "6"),
            ExecTypeEnum::STOPPED => write!(f, "7"),
            ExecTypeEnum::REJECTED => write!(f, "8"),
            ExecTypeEnum::SUSPENDED => write!(f, "9"),
            ExecTypeEnum::PENDING_NEW => write!(f, "A"),
            ExecTypeEnum::CALCULATED => write!(f, "B"),
            ExecTypeEnum::EXPIRED => write!(f, "C"),
            ExecTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LocateReqdEnum {
    /// NO
    NO,
    /// YES
    YES,
    /// Unknown value
    Unknown(String),
}

impl From<String> for LocateReqdEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" => LocateReqdEnum::NO,
            "Y" => LocateReqdEnum::YES,
            _ => LocateReqdEnum::Unknown(s),
        }
    }
}

impl fmt::Display for LocateReqdEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocateReqdEnum::NO => write!(f, "N"),
            LocateReqdEnum::YES => write!(f, "Y"),
            LocateReqdEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SettlLocationEnum {
    /// CEDEL
    CEDEL,
    /// DEPOSITORY_TRUST_COMPANY
    DEPOSITORY_TRUST_COMPANY,
    /// EURO_CLEAR
    EURO_CLEAR,
    /// FEDERAL_BOOK_ENTRY
    FEDERAL_BOOK_ENTRY,
    /// LOCAL_MARKET_SETTLE_LOCATION
    LOCAL_MARKET_SETTLE_LOCATION,
    /// PHYSICAL
    PHYSICAL,
    /// PARTICIPANT_TRUST_COMPANY
    PARTICIPANT_TRUST_COMPANY,
    /// Unknown value
    Unknown(String),
}

impl From<String> for SettlLocationEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "CED" => SettlLocationEnum::CEDEL,
            "DTC" => SettlLocationEnum::DEPOSITORY_TRUST_COMPANY,
            "EUR" => SettlLocationEnum::EURO_CLEAR,
            "FED" => SettlLocationEnum::FEDERAL_BOOK_ENTRY,
            "ISO Country Code" => SettlLocationEnum::LOCAL_MARKET_SETTLE_LOCATION,
            "PNY" => SettlLocationEnum::PHYSICAL,
            "PTC" => SettlLocationEnum::PARTICIPANT_TRUST_COMPANY,
            _ => SettlLocationEnum::Unknown(s),
        }
    }
}

impl fmt::Display for SettlLocationEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettlLocationEnum::CEDEL => write!(f, "CED"),
            SettlLocationEnum::DEPOSITORY_TRUST_COMPANY => write!(f, "DTC"),
            SettlLocationEnum::EURO_CLEAR => write!(f, "EUR"),
            SettlLocationEnum::FEDERAL_BOOK_ENTRY => write!(f, "FED"),
            SettlLocationEnum::LOCAL_MARKET_SETTLE_LOCATION => write!(f, "ISO Country Code"),
            SettlLocationEnum::PHYSICAL => write!(f, "PNY"),
            SettlLocationEnum::PARTICIPANT_TRUST_COMPANY => write!(f, "PTC"),
            SettlLocationEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TimeInForceEnum {
    /// DAY
    DAY,
    /// GOOD_TILL_CANCEL
    GOOD_TILL_CANCEL,
    /// AT_THE_OPENING
    AT_THE_OPENING,
    /// IMMEDIATE_OR_CANCEL
    IMMEDIATE_OR_CANCEL,
    /// FILL_OR_KILL
    FILL_OR_KILL,
    /// GOOD_TILL_CROSSING
    GOOD_TILL_CROSSING,
    /// GOOD_TILL_DATE
    GOOD_TILL_DATE,
    /// Unknown value
    Unknown(String),
}

impl From<String> for TimeInForceEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => TimeInForceEnum::DAY,
            "1" => TimeInForceEnum::GOOD_TILL_CANCEL,
            "2" => TimeInForceEnum::AT_THE_OPENING,
            "3" => TimeInForceEnum::IMMEDIATE_OR_CANCEL,
            "4" => TimeInForceEnum::FILL_OR_KILL,
            "5" => TimeInForceEnum::GOOD_TILL_CROSSING,
            "6" => TimeInForceEnum::GOOD_TILL_DATE,
            _ => TimeInForceEnum::Unknown(s),
        }
    }
}

impl fmt::Display for TimeInForceEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeInForceEnum::DAY => write!(f, "0"),
            TimeInForceEnum::GOOD_TILL_CANCEL => write!(f, "1"),
            TimeInForceEnum::AT_THE_OPENING => write!(f, "2"),
            TimeInForceEnum::IMMEDIATE_OR_CANCEL => write!(f, "3"),
            TimeInForceEnum::FILL_OR_KILL => write!(f, "4"),
            TimeInForceEnum::GOOD_TILL_CROSSING => write!(f, "5"),
            TimeInForceEnum::GOOD_TILL_DATE => write!(f, "6"),
            TimeInForceEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
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
pub enum MsgTypeEnum {
    /// HEARTBEAT
    HEARTBEAT,
    /// TEST_REQUEST
    TEST_REQUEST,
    /// RESEND_REQUEST
    RESEND_REQUEST,
    /// REJECT
    REJECT,
    /// SEQUENCE_RESET
    SEQUENCE_RESET,
    /// LOGOUT
    LOGOUT,
    /// IOI
    IOI,
    /// ADVERTISEMENT
    ADVERTISEMENT,
    /// EXECUTION_REPORT
    EXECUTION_REPORT,
    /// ORDER_CANCEL_REJECT
    ORDER_CANCEL_REJECT,
    /// LOGON
    LOGON,
    /// NEWS
    NEWS,
    /// EMAIL
    EMAIL,
    /// NEW_ORDER_SINGLE
    NEW_ORDER_SINGLE,
    /// NEW_ORDER_LIST
    NEW_ORDER_LIST,
    /// ORDER_CANCEL_REQUEST
    ORDER_CANCEL_REQUEST,
    /// ORDER_CANCEL_REPLACE_REQUEST
    ORDER_CANCEL_REPLACE_REQUEST,
    /// ORDER_STATUS_REQUEST
    ORDER_STATUS_REQUEST,
    /// ALLOCATION_INSTRUCTION
    ALLOCATION_INSTRUCTION,
    /// LIST_CANCEL_REQUEST
    LIST_CANCEL_REQUEST,
    /// LIST_EXECUTE
    LIST_EXECUTE,
    /// LIST_STATUS_REQUEST
    LIST_STATUS_REQUEST,
    /// LIST_STATUS
    LIST_STATUS,
    /// ALLOCATION_INSTRUCTION_ACK
    ALLOCATION_INSTRUCTION_ACK,
    /// DONT_KNOW_TRADE
    DONT_KNOW_TRADE,
    /// QUOTE_REQUEST
    QUOTE_REQUEST,
    /// QUOTE
    QUOTE,
    /// SETTLEMENT_INSTRUCTIONS
    SETTLEMENT_INSTRUCTIONS,
    /// Unknown value
    Unknown(String),
}

impl From<String> for MsgTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => MsgTypeEnum::HEARTBEAT,
            "1" => MsgTypeEnum::TEST_REQUEST,
            "2" => MsgTypeEnum::RESEND_REQUEST,
            "3" => MsgTypeEnum::REJECT,
            "4" => MsgTypeEnum::SEQUENCE_RESET,
            "5" => MsgTypeEnum::LOGOUT,
            "6" => MsgTypeEnum::IOI,
            "7" => MsgTypeEnum::ADVERTISEMENT,
            "8" => MsgTypeEnum::EXECUTION_REPORT,
            "9" => MsgTypeEnum::ORDER_CANCEL_REJECT,
            "A" => MsgTypeEnum::LOGON,
            "B" => MsgTypeEnum::NEWS,
            "C" => MsgTypeEnum::EMAIL,
            "D" => MsgTypeEnum::NEW_ORDER_SINGLE,
            "E" => MsgTypeEnum::NEW_ORDER_LIST,
            "F" => MsgTypeEnum::ORDER_CANCEL_REQUEST,
            "G" => MsgTypeEnum::ORDER_CANCEL_REPLACE_REQUEST,
            "H" => MsgTypeEnum::ORDER_STATUS_REQUEST,
            "J" => MsgTypeEnum::ALLOCATION_INSTRUCTION,
            "K" => MsgTypeEnum::LIST_CANCEL_REQUEST,
            "L" => MsgTypeEnum::LIST_EXECUTE,
            "M" => MsgTypeEnum::LIST_STATUS_REQUEST,
            "N" => MsgTypeEnum::LIST_STATUS,
            "P" => MsgTypeEnum::ALLOCATION_INSTRUCTION_ACK,
            "Q" => MsgTypeEnum::DONT_KNOW_TRADE,
            "R" => MsgTypeEnum::QUOTE_REQUEST,
            "S" => MsgTypeEnum::QUOTE,
            "T" => MsgTypeEnum::SETTLEMENT_INSTRUCTIONS,
            _ => MsgTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for MsgTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MsgTypeEnum::HEARTBEAT => write!(f, "0"),
            MsgTypeEnum::TEST_REQUEST => write!(f, "1"),
            MsgTypeEnum::RESEND_REQUEST => write!(f, "2"),
            MsgTypeEnum::REJECT => write!(f, "3"),
            MsgTypeEnum::SEQUENCE_RESET => write!(f, "4"),
            MsgTypeEnum::LOGOUT => write!(f, "5"),
            MsgTypeEnum::IOI => write!(f, "6"),
            MsgTypeEnum::ADVERTISEMENT => write!(f, "7"),
            MsgTypeEnum::EXECUTION_REPORT => write!(f, "8"),
            MsgTypeEnum::ORDER_CANCEL_REJECT => write!(f, "9"),
            MsgTypeEnum::LOGON => write!(f, "A"),
            MsgTypeEnum::NEWS => write!(f, "B"),
            MsgTypeEnum::EMAIL => write!(f, "C"),
            MsgTypeEnum::NEW_ORDER_SINGLE => write!(f, "D"),
            MsgTypeEnum::NEW_ORDER_LIST => write!(f, "E"),
            MsgTypeEnum::ORDER_CANCEL_REQUEST => write!(f, "F"),
            MsgTypeEnum::ORDER_CANCEL_REPLACE_REQUEST => write!(f, "G"),
            MsgTypeEnum::ORDER_STATUS_REQUEST => write!(f, "H"),
            MsgTypeEnum::ALLOCATION_INSTRUCTION => write!(f, "J"),
            MsgTypeEnum::LIST_CANCEL_REQUEST => write!(f, "K"),
            MsgTypeEnum::LIST_EXECUTE => write!(f, "L"),
            MsgTypeEnum::LIST_STATUS_REQUEST => write!(f, "M"),
            MsgTypeEnum::LIST_STATUS => write!(f, "N"),
            MsgTypeEnum::ALLOCATION_INSTRUCTION_ACK => write!(f, "P"),
            MsgTypeEnum::DONT_KNOW_TRADE => write!(f, "Q"),
            MsgTypeEnum::QUOTE_REQUEST => write!(f, "R"),
            MsgTypeEnum::QUOTE => write!(f, "S"),
            MsgTypeEnum::SETTLEMENT_INSTRUCTIONS => write!(f, "T"),
            MsgTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum EmailTypeEnum {
    /// NEW
    NEW,
    /// REPLY
    REPLY,
    /// ADMIN_REPLY
    ADMIN_REPLY,
    /// Unknown value
    Unknown(String),
}

impl From<String> for EmailTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => EmailTypeEnum::NEW,
            "1" => EmailTypeEnum::REPLY,
            "2" => EmailTypeEnum::ADMIN_REPLY,
            _ => EmailTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for EmailTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailTypeEnum::NEW => write!(f, "0"),
            EmailTypeEnum::REPLY => write!(f, "1"),
            EmailTypeEnum::ADMIN_REPLY => write!(f, "2"),
            EmailTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IOITransTypeEnum {
    /// CANCEL
    CANCEL,
    /// NEW
    NEW,
    /// REPLACE
    REPLACE,
    /// Unknown value
    Unknown(String),
}

impl From<String> for IOITransTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "C" => IOITransTypeEnum::CANCEL,
            "N" => IOITransTypeEnum::NEW,
            "R" => IOITransTypeEnum::REPLACE,
            _ => IOITransTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for IOITransTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOITransTypeEnum::CANCEL => write!(f, "C"),
            IOITransTypeEnum::NEW => write!(f, "N"),
            IOITransTypeEnum::REPLACE => write!(f, "R"),
            IOITransTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ExecInstEnum {
    /// STAY_ON_OFFER_SIDE
    STAY_ON_OFFER_SIDE,
    /// NOT_HELD
    NOT_HELD,
    /// WORK
    WORK,
    /// GO_ALONG
    GO_ALONG,
    /// OVER_THE_DAY
    OVER_THE_DAY,
    /// HELD
    HELD,
    /// PARTICIPATE_DO_NOT_INITIATE
    PARTICIPATE_DO_NOT_INITIATE,
    /// STRICT_SCALE
    STRICT_SCALE,
    /// TRY_TO_SCALE
    TRY_TO_SCALE,
    /// STAY_ON_BID_SIDE
    STAY_ON_BID_SIDE,
    /// NO_CROSS
    NO_CROSS,
    /// OK_TO_CROSS
    OK_TO_CROSS,
    /// CALL_FIRST
    CALL_FIRST,
    /// PERCENT_OF_VOLUME
    PERCENT_OF_VOLUME,
    /// DO_NOT_INCREASE
    DO_NOT_INCREASE,
    /// DO_NOT_REDUCE
    DO_NOT_REDUCE,
    /// ALL_OR_NONE
    ALL_OR_NONE,
    /// INSTITUTIONS_ONLY
    INSTITUTIONS_ONLY,
    /// LAST_PEG
    LAST_PEG,
    /// MID_PRICE_PEG
    MID_PRICE_PEG,
    /// NON_NEGOTIABLE
    NON_NEGOTIABLE,
    /// OPENING_PEG
    OPENING_PEG,
    /// MARKET_PEG
    MARKET_PEG,
    /// PRIMARY_PEG
    PRIMARY_PEG,
    /// SUSPEND
    SUSPEND,
    /// CUSTOMER_DISPLAY_INSTRUCTION
    CUSTOMER_DISPLAY_INSTRUCTION,
    /// NETTING
    NETTING,
    /// Unknown value
    Unknown(String),
}

impl From<String> for ExecInstEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => ExecInstEnum::STAY_ON_OFFER_SIDE,
            "1" => ExecInstEnum::NOT_HELD,
            "2" => ExecInstEnum::WORK,
            "3" => ExecInstEnum::GO_ALONG,
            "4" => ExecInstEnum::OVER_THE_DAY,
            "5" => ExecInstEnum::HELD,
            "6" => ExecInstEnum::PARTICIPATE_DO_NOT_INITIATE,
            "7" => ExecInstEnum::STRICT_SCALE,
            "8" => ExecInstEnum::TRY_TO_SCALE,
            "9" => ExecInstEnum::STAY_ON_BID_SIDE,
            "A" => ExecInstEnum::NO_CROSS,
            "B" => ExecInstEnum::OK_TO_CROSS,
            "C" => ExecInstEnum::CALL_FIRST,
            "D" => ExecInstEnum::PERCENT_OF_VOLUME,
            "E" => ExecInstEnum::DO_NOT_INCREASE,
            "F" => ExecInstEnum::DO_NOT_REDUCE,
            "G" => ExecInstEnum::ALL_OR_NONE,
            "I" => ExecInstEnum::INSTITUTIONS_ONLY,
            "L" => ExecInstEnum::LAST_PEG,
            "M" => ExecInstEnum::MID_PRICE_PEG,
            "N" => ExecInstEnum::NON_NEGOTIABLE,
            "O" => ExecInstEnum::OPENING_PEG,
            "P" => ExecInstEnum::MARKET_PEG,
            "R" => ExecInstEnum::PRIMARY_PEG,
            "S" => ExecInstEnum::SUSPEND,
            "U" => ExecInstEnum::CUSTOMER_DISPLAY_INSTRUCTION,
            "V" => ExecInstEnum::NETTING,
            _ => ExecInstEnum::Unknown(s),
        }
    }
}

impl fmt::Display for ExecInstEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecInstEnum::STAY_ON_OFFER_SIDE => write!(f, "0"),
            ExecInstEnum::NOT_HELD => write!(f, "1"),
            ExecInstEnum::WORK => write!(f, "2"),
            ExecInstEnum::GO_ALONG => write!(f, "3"),
            ExecInstEnum::OVER_THE_DAY => write!(f, "4"),
            ExecInstEnum::HELD => write!(f, "5"),
            ExecInstEnum::PARTICIPATE_DO_NOT_INITIATE => write!(f, "6"),
            ExecInstEnum::STRICT_SCALE => write!(f, "7"),
            ExecInstEnum::TRY_TO_SCALE => write!(f, "8"),
            ExecInstEnum::STAY_ON_BID_SIDE => write!(f, "9"),
            ExecInstEnum::NO_CROSS => write!(f, "A"),
            ExecInstEnum::OK_TO_CROSS => write!(f, "B"),
            ExecInstEnum::CALL_FIRST => write!(f, "C"),
            ExecInstEnum::PERCENT_OF_VOLUME => write!(f, "D"),
            ExecInstEnum::DO_NOT_INCREASE => write!(f, "E"),
            ExecInstEnum::DO_NOT_REDUCE => write!(f, "F"),
            ExecInstEnum::ALL_OR_NONE => write!(f, "G"),
            ExecInstEnum::INSTITUTIONS_ONLY => write!(f, "I"),
            ExecInstEnum::LAST_PEG => write!(f, "L"),
            ExecInstEnum::MID_PRICE_PEG => write!(f, "M"),
            ExecInstEnum::NON_NEGOTIABLE => write!(f, "N"),
            ExecInstEnum::OPENING_PEG => write!(f, "O"),
            ExecInstEnum::MARKET_PEG => write!(f, "P"),
            ExecInstEnum::PRIMARY_PEG => write!(f, "R"),
            ExecInstEnum::SUSPEND => write!(f, "S"),
            ExecInstEnum::CUSTOMER_DISPLAY_INSTRUCTION => write!(f, "U"),
            ExecInstEnum::NETTING => write!(f, "V"),
            ExecInstEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ForexReqEnum {
    /// NO
    NO,
    /// YES
    YES,
    /// Unknown value
    Unknown(String),
}

impl From<String> for ForexReqEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" => ForexReqEnum::NO,
            "Y" => ForexReqEnum::YES,
            _ => ForexReqEnum::Unknown(s),
        }
    }
}

impl fmt::Display for ForexReqEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForexReqEnum::NO => write!(f, "N"),
            ForexReqEnum::YES => write!(f, "Y"),
            ForexReqEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IOIQualifierEnum {
    /// ALL_OR_NONE
    ALL_OR_NONE,
    /// AT_THE_CLOSE
    AT_THE_CLOSE,
    /// IN_TOUCH_WITH
    IN_TOUCH_WITH,
    /// LIMIT
    LIMIT,
    /// MORE_BEHIND
    MORE_BEHIND,
    /// AT_THE_OPEN
    AT_THE_OPEN,
    /// TAKING_A_POSITION
    TAKING_A_POSITION,
    /// AT_THE_MARKET
    AT_THE_MARKET,
    /// PORTFOLIO_SHOWN
    PORTFOLIO_SHOWN,
    /// THROUGH_THE_DAY
    THROUGH_THE_DAY,
    /// VERSUS
    VERSUS,
    /// INDICATION
    INDICATION,
    /// CROSSING_OPPORTUNITY
    CROSSING_OPPORTUNITY,
    /// AT_THE_MIDPOINT
    AT_THE_MIDPOINT,
    /// PRE_OPEN
    PRE_OPEN,
    /// Unknown value
    Unknown(String),
}

impl From<String> for IOIQualifierEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "A" => IOIQualifierEnum::ALL_OR_NONE,
            "C" => IOIQualifierEnum::AT_THE_CLOSE,
            "I" => IOIQualifierEnum::IN_TOUCH_WITH,
            "L" => IOIQualifierEnum::LIMIT,
            "M" => IOIQualifierEnum::MORE_BEHIND,
            "O" => IOIQualifierEnum::AT_THE_OPEN,
            "P" => IOIQualifierEnum::TAKING_A_POSITION,
            "Q" => IOIQualifierEnum::AT_THE_MARKET,
            "S" => IOIQualifierEnum::PORTFOLIO_SHOWN,
            "T" => IOIQualifierEnum::THROUGH_THE_DAY,
            "V" => IOIQualifierEnum::VERSUS,
            "W" => IOIQualifierEnum::INDICATION,
            "X" => IOIQualifierEnum::CROSSING_OPPORTUNITY,
            "Y" => IOIQualifierEnum::AT_THE_MIDPOINT,
            "Z" => IOIQualifierEnum::PRE_OPEN,
            _ => IOIQualifierEnum::Unknown(s),
        }
    }
}

impl fmt::Display for IOIQualifierEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOIQualifierEnum::ALL_OR_NONE => write!(f, "A"),
            IOIQualifierEnum::AT_THE_CLOSE => write!(f, "C"),
            IOIQualifierEnum::IN_TOUCH_WITH => write!(f, "I"),
            IOIQualifierEnum::LIMIT => write!(f, "L"),
            IOIQualifierEnum::MORE_BEHIND => write!(f, "M"),
            IOIQualifierEnum::AT_THE_OPEN => write!(f, "O"),
            IOIQualifierEnum::TAKING_A_POSITION => write!(f, "P"),
            IOIQualifierEnum::AT_THE_MARKET => write!(f, "Q"),
            IOIQualifierEnum::PORTFOLIO_SHOWN => write!(f, "S"),
            IOIQualifierEnum::THROUGH_THE_DAY => write!(f, "T"),
            IOIQualifierEnum::VERSUS => write!(f, "V"),
            IOIQualifierEnum::INDICATION => write!(f, "W"),
            IOIQualifierEnum::CROSSING_OPPORTUNITY => write!(f, "X"),
            IOIQualifierEnum::AT_THE_MIDPOINT => write!(f, "Y"),
            IOIQualifierEnum::PRE_OPEN => write!(f, "Z"),
            IOIQualifierEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Rule80AEnum {
    /// AGENCY_SINGLE_ORDER
    AGENCY_SINGLE_ORDER,
    /// SHORT_EXEMPT_TRANSACTION_A_TYPE
    SHORT_EXEMPT_TRANSACTION_A_TYPE,
    /// PROPRIETARY_NON_ALGO
    PROPRIETARY_NON_ALGO,
    /// PROGRAM_ORDER_MEMBER
    PROGRAM_ORDER_MEMBER,
    /// SHORT_EXEMPT_TRANSACTION_FOR_PRINCIPAL
    SHORT_EXEMPT_TRANSACTION_FOR_PRINCIPAL,
    /// SHORT_EXEMPT_TRANSACTION_W_TYPE
    SHORT_EXEMPT_TRANSACTION_W_TYPE,
    /// SHORT_EXEMPT_TRANSACTION_I_TYPE
    SHORT_EXEMPT_TRANSACTION_I_TYPE,
    /// INDIVIDUAL_INVESTOR
    INDIVIDUAL_INVESTOR,
    /// PROPRIETARY_ALGO
    PROPRIETARY_ALGO,
    /// AGENCY_ALGO
    AGENCY_ALGO,
    /// SHORT_EXEMPT_TRANSACTION_MEMBER_AFFLIATED
    SHORT_EXEMPT_TRANSACTION_MEMBER_AFFLIATED,
    /// PROGRAM_ORDER_OTHER_MEMBER
    PROGRAM_ORDER_OTHER_MEMBER,
    /// AGENT_FOR_OTHER_MEMBER
    AGENT_FOR_OTHER_MEMBER,
    /// PROPRIETARY_TRANSACTION_AFFILIATED
    PROPRIETARY_TRANSACTION_AFFILIATED,
    /// PRINCIPAL
    PRINCIPAL,
    /// TRANSACTION_NON_MEMBER
    TRANSACTION_NON_MEMBER,
    /// SPECIALIST_TRADES
    SPECIALIST_TRADES,
    /// TRANSACTION_UNAFFILIATED_MEMBER
    TRANSACTION_UNAFFILIATED_MEMBER,
    /// AGENCY_INDEX_ARB
    AGENCY_INDEX_ARB,
    /// ALL_OTHER_ORDERS_AS_AGENT_FOR_OTHER_MEMBER
    ALL_OTHER_ORDERS_AS_AGENT_FOR_OTHER_MEMBER,
    /// SHORT_EXEMPT_TRANSACTION_MEMBER_NOT_AFFLIATED
    SHORT_EXEMPT_TRANSACTION_MEMBER_NOT_AFFLIATED,
    /// AGENCY_NON_ALGO
    AGENCY_NON_ALGO,
    /// SHORT_EXEMPT_TRANSACTION_NON_MEMBER
    SHORT_EXEMPT_TRANSACTION_NON_MEMBER,
    /// Unknown value
    Unknown(String),
}

impl From<String> for Rule80AEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "A" => Rule80AEnum::AGENCY_SINGLE_ORDER,
            "B" => Rule80AEnum::SHORT_EXEMPT_TRANSACTION_A_TYPE,
            "C" => Rule80AEnum::PROPRIETARY_NON_ALGO,
            "D" => Rule80AEnum::PROGRAM_ORDER_MEMBER,
            "E" => Rule80AEnum::SHORT_EXEMPT_TRANSACTION_FOR_PRINCIPAL,
            "F" => Rule80AEnum::SHORT_EXEMPT_TRANSACTION_W_TYPE,
            "H" => Rule80AEnum::SHORT_EXEMPT_TRANSACTION_I_TYPE,
            "I" => Rule80AEnum::INDIVIDUAL_INVESTOR,
            "J" => Rule80AEnum::PROPRIETARY_ALGO,
            "K" => Rule80AEnum::AGENCY_ALGO,
            "L" => Rule80AEnum::SHORT_EXEMPT_TRANSACTION_MEMBER_AFFLIATED,
            "M" => Rule80AEnum::PROGRAM_ORDER_OTHER_MEMBER,
            "N" => Rule80AEnum::AGENT_FOR_OTHER_MEMBER,
            "O" => Rule80AEnum::PROPRIETARY_TRANSACTION_AFFILIATED,
            "P" => Rule80AEnum::PRINCIPAL,
            "R" => Rule80AEnum::TRANSACTION_NON_MEMBER,
            "S" => Rule80AEnum::SPECIALIST_TRADES,
            "T" => Rule80AEnum::TRANSACTION_UNAFFILIATED_MEMBER,
            "U" => Rule80AEnum::AGENCY_INDEX_ARB,
            "W" => Rule80AEnum::ALL_OTHER_ORDERS_AS_AGENT_FOR_OTHER_MEMBER,
            "X" => Rule80AEnum::SHORT_EXEMPT_TRANSACTION_MEMBER_NOT_AFFLIATED,
            "Y" => Rule80AEnum::AGENCY_NON_ALGO,
            "Z" => Rule80AEnum::SHORT_EXEMPT_TRANSACTION_NON_MEMBER,
            _ => Rule80AEnum::Unknown(s),
        }
    }
}

impl fmt::Display for Rule80AEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule80AEnum::AGENCY_SINGLE_ORDER => write!(f, "A"),
            Rule80AEnum::SHORT_EXEMPT_TRANSACTION_A_TYPE => write!(f, "B"),
            Rule80AEnum::PROPRIETARY_NON_ALGO => write!(f, "C"),
            Rule80AEnum::PROGRAM_ORDER_MEMBER => write!(f, "D"),
            Rule80AEnum::SHORT_EXEMPT_TRANSACTION_FOR_PRINCIPAL => write!(f, "E"),
            Rule80AEnum::SHORT_EXEMPT_TRANSACTION_W_TYPE => write!(f, "F"),
            Rule80AEnum::SHORT_EXEMPT_TRANSACTION_I_TYPE => write!(f, "H"),
            Rule80AEnum::INDIVIDUAL_INVESTOR => write!(f, "I"),
            Rule80AEnum::PROPRIETARY_ALGO => write!(f, "J"),
            Rule80AEnum::AGENCY_ALGO => write!(f, "K"),
            Rule80AEnum::SHORT_EXEMPT_TRANSACTION_MEMBER_AFFLIATED => write!(f, "L"),
            Rule80AEnum::PROGRAM_ORDER_OTHER_MEMBER => write!(f, "M"),
            Rule80AEnum::AGENT_FOR_OTHER_MEMBER => write!(f, "N"),
            Rule80AEnum::PROPRIETARY_TRANSACTION_AFFILIATED => write!(f, "O"),
            Rule80AEnum::PRINCIPAL => write!(f, "P"),
            Rule80AEnum::TRANSACTION_NON_MEMBER => write!(f, "R"),
            Rule80AEnum::SPECIALIST_TRADES => write!(f, "S"),
            Rule80AEnum::TRANSACTION_UNAFFILIATED_MEMBER => write!(f, "T"),
            Rule80AEnum::AGENCY_INDEX_ARB => write!(f, "U"),
            Rule80AEnum::ALL_OTHER_ORDERS_AS_AGENT_FOR_OTHER_MEMBER => write!(f, "W"),
            Rule80AEnum::SHORT_EXEMPT_TRANSACTION_MEMBER_NOT_AFFLIATED => write!(f, "X"),
            Rule80AEnum::AGENCY_NON_ALGO => write!(f, "Y"),
            Rule80AEnum::SHORT_EXEMPT_TRANSACTION_NON_MEMBER => write!(f, "Z"),
            Rule80AEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ProcessCodeEnum {
    /// REGULAR
    REGULAR,
    /// SOFT_DOLLAR
    SOFT_DOLLAR,
    /// STEP_IN
    STEP_IN,
    /// STEP_OUT
    STEP_OUT,
    /// SOFT_DOLLAR_STEP_IN
    SOFT_DOLLAR_STEP_IN,
    /// SOFT_DOLLAR_STEP_OUT
    SOFT_DOLLAR_STEP_OUT,
    /// PLAN_SPONSOR
    PLAN_SPONSOR,
    /// Unknown value
    Unknown(String),
}

impl From<String> for ProcessCodeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => ProcessCodeEnum::REGULAR,
            "1" => ProcessCodeEnum::SOFT_DOLLAR,
            "2" => ProcessCodeEnum::STEP_IN,
            "3" => ProcessCodeEnum::STEP_OUT,
            "4" => ProcessCodeEnum::SOFT_DOLLAR_STEP_IN,
            "5" => ProcessCodeEnum::SOFT_DOLLAR_STEP_OUT,
            "6" => ProcessCodeEnum::PLAN_SPONSOR,
            _ => ProcessCodeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for ProcessCodeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessCodeEnum::REGULAR => write!(f, "0"),
            ProcessCodeEnum::SOFT_DOLLAR => write!(f, "1"),
            ProcessCodeEnum::STEP_IN => write!(f, "2"),
            ProcessCodeEnum::STEP_OUT => write!(f, "3"),
            ProcessCodeEnum::SOFT_DOLLAR_STEP_IN => write!(f, "4"),
            ProcessCodeEnum::SOFT_DOLLAR_STEP_OUT => write!(f, "5"),
            ProcessCodeEnum::PLAN_SPONSOR => write!(f, "6"),
            ProcessCodeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum UrgencyEnum {
    /// NORMAL
    NORMAL,
    /// FLASH
    FLASH,
    /// BACKGROUND
    BACKGROUND,
    /// Unknown value
    Unknown(String),
}

impl From<String> for UrgencyEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => UrgencyEnum::NORMAL,
            "1" => UrgencyEnum::FLASH,
            "2" => UrgencyEnum::BACKGROUND,
            _ => UrgencyEnum::Unknown(s),
        }
    }
}

impl fmt::Display for UrgencyEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UrgencyEnum::NORMAL => write!(f, "0"),
            UrgencyEnum::FLASH => write!(f, "1"),
            UrgencyEnum::BACKGROUND => write!(f, "2"),
            UrgencyEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum NotifyBrokerOfCreditEnum {
    /// NO
    NO,
    /// YES
    YES,
    /// Unknown value
    Unknown(String),
}

impl From<String> for NotifyBrokerOfCreditEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" => NotifyBrokerOfCreditEnum::NO,
            "Y" => NotifyBrokerOfCreditEnum::YES,
            _ => NotifyBrokerOfCreditEnum::Unknown(s),
        }
    }
}

impl fmt::Display for NotifyBrokerOfCreditEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotifyBrokerOfCreditEnum::NO => write!(f, "N"),
            NotifyBrokerOfCreditEnum::YES => write!(f, "Y"),
            NotifyBrokerOfCreditEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SettlInstTransTypeEnum {
    /// CANCEL
    CANCEL,
    /// NEW
    NEW,
    /// REPLACE
    REPLACE,
    /// Unknown value
    Unknown(String),
}

impl From<String> for SettlInstTransTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "C" => SettlInstTransTypeEnum::CANCEL,
            "N" => SettlInstTransTypeEnum::NEW,
            "R" => SettlInstTransTypeEnum::REPLACE,
            _ => SettlInstTransTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for SettlInstTransTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettlInstTransTypeEnum::CANCEL => write!(f, "C"),
            SettlInstTransTypeEnum::NEW => write!(f, "N"),
            SettlInstTransTypeEnum::REPLACE => write!(f, "R"),
            SettlInstTransTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum HandlInstEnum {
    /// AUTOMATED_EXECUTION_NO_INTERVENTION
    AUTOMATED_EXECUTION_NO_INTERVENTION,
    /// AUTOMATED_EXECUTION_INTERVENTION_OK
    AUTOMATED_EXECUTION_INTERVENTION_OK,
    /// MANUAL_ORDER
    MANUAL_ORDER,
    /// Unknown value
    Unknown(String),
}

impl From<String> for HandlInstEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "1" => HandlInstEnum::AUTOMATED_EXECUTION_NO_INTERVENTION,
            "2" => HandlInstEnum::AUTOMATED_EXECUTION_INTERVENTION_OK,
            "3" => HandlInstEnum::MANUAL_ORDER,
            _ => HandlInstEnum::Unknown(s),
        }
    }
}

impl fmt::Display for HandlInstEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandlInstEnum::AUTOMATED_EXECUTION_NO_INTERVENTION => write!(f, "1"),
            HandlInstEnum::AUTOMATED_EXECUTION_INTERVENTION_OK => write!(f, "2"),
            HandlInstEnum::MANUAL_ORDER => write!(f, "3"),
            HandlInstEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IOIOthSvcEnum {
    /// AUTEX
    AUTEX,
    /// BRIDGE
    BRIDGE,
    /// Unknown value
    Unknown(String),
}

impl From<String> for IOIOthSvcEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "A" => IOIOthSvcEnum::AUTEX,
            "B" => IOIOthSvcEnum::BRIDGE,
            _ => IOIOthSvcEnum::Unknown(s),
        }
    }
}

impl fmt::Display for IOIOthSvcEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOIOthSvcEnum::AUTEX => write!(f, "A"),
            IOIOthSvcEnum::BRIDGE => write!(f, "B"),
            IOIOthSvcEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IDSourceEnum {
    /// CUSIP
    CUSIP,
    /// SEDOL
    SEDOL,
    /// QUIK
    QUIK,
    /// ISIN_NUMBER
    ISIN_NUMBER,
    /// RIC_CODE
    RIC_CODE,
    /// ISO_CURRENCY_CODE
    ISO_CURRENCY_CODE,
    /// ISO_COUNTRY_CODE
    ISO_COUNTRY_CODE,
    /// Unknown value
    Unknown(String),
}

impl From<String> for IDSourceEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "1" => IDSourceEnum::CUSIP,
            "2" => IDSourceEnum::SEDOL,
            "3" => IDSourceEnum::QUIK,
            "4" => IDSourceEnum::ISIN_NUMBER,
            "5" => IDSourceEnum::RIC_CODE,
            "6" => IDSourceEnum::ISO_CURRENCY_CODE,
            "7" => IDSourceEnum::ISO_COUNTRY_CODE,
            _ => IDSourceEnum::Unknown(s),
        }
    }
}

impl fmt::Display for IDSourceEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IDSourceEnum::CUSIP => write!(f, "1"),
            IDSourceEnum::SEDOL => write!(f, "2"),
            IDSourceEnum::QUIK => write!(f, "3"),
            IDSourceEnum::ISIN_NUMBER => write!(f, "4"),
            IDSourceEnum::RIC_CODE => write!(f, "5"),
            IDSourceEnum::ISO_CURRENCY_CODE => write!(f, "6"),
            IDSourceEnum::ISO_COUNTRY_CODE => write!(f, "7"),
            IDSourceEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AllocLinkTypeEnum {
    /// FX_NETTING
    FX_NETTING,
    /// FX_SWAP
    FX_SWAP,
    /// Unknown value
    Unknown(String),
}

impl From<String> for AllocLinkTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => AllocLinkTypeEnum::FX_NETTING,
            "1" => AllocLinkTypeEnum::FX_SWAP,
            _ => AllocLinkTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for AllocLinkTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocLinkTypeEnum::FX_NETTING => write!(f, "0"),
            AllocLinkTypeEnum::FX_SWAP => write!(f, "1"),
            AllocLinkTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum EncryptMethodEnum {
    /// NONE
    NONE,
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
    /// PEM
    PEM,
    /// Unknown value
    Unknown(String),
}

impl From<String> for EncryptMethodEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => EncryptMethodEnum::NONE,
            "1" => EncryptMethodEnum::PKCS,
            "2" => EncryptMethodEnum::DES,
            "3" => EncryptMethodEnum::PKCSDES,
            "4" => EncryptMethodEnum::PGPDES,
            "5" => EncryptMethodEnum::PGPDESMD5,
            "6" => EncryptMethodEnum::PEM,
            _ => EncryptMethodEnum::Unknown(s),
        }
    }
}

impl fmt::Display for EncryptMethodEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptMethodEnum::NONE => write!(f, "0"),
            EncryptMethodEnum::PKCS => write!(f, "1"),
            EncryptMethodEnum::DES => write!(f, "2"),
            EncryptMethodEnum::PKCSDES => write!(f, "3"),
            EncryptMethodEnum::PGPDES => write!(f, "4"),
            EncryptMethodEnum::PGPDESMD5 => write!(f, "5"),
            EncryptMethodEnum::PEM => write!(f, "6"),
            EncryptMethodEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CxlRejReasonEnum {
    /// TOO_LATE_TO_CANCEL
    TOO_LATE_TO_CANCEL,
    /// UNKNOWN_ORDER
    UNKNOWN_ORDER,
    /// Unknown value
    Unknown(String),
}

impl From<String> for CxlRejReasonEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => CxlRejReasonEnum::TOO_LATE_TO_CANCEL,
            "1" => CxlRejReasonEnum::UNKNOWN_ORDER,
            _ => CxlRejReasonEnum::Unknown(s),
        }
    }
}

impl fmt::Display for CxlRejReasonEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CxlRejReasonEnum::TOO_LATE_TO_CANCEL => write!(f, "0"),
            CxlRejReasonEnum::UNKNOWN_ORDER => write!(f, "1"),
            CxlRejReasonEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SideEnum {
    /// BUY
    BUY,
    /// SELL
    SELL,
    /// BUY_MINUS
    BUY_MINUS,
    /// SELL_PLUS
    SELL_PLUS,
    /// SELL_SHORT
    SELL_SHORT,
    /// SELL_SHORT_EXEMPT
    SELL_SHORT_EXEMPT,
    /// UNDISCLOSED
    UNDISCLOSED,
    /// CROSS
    CROSS,
    /// Unknown value
    Unknown(String),
}

impl From<String> for SideEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "1" => SideEnum::BUY,
            "2" => SideEnum::SELL,
            "3" => SideEnum::BUY_MINUS,
            "4" => SideEnum::SELL_PLUS,
            "5" => SideEnum::SELL_SHORT,
            "6" => SideEnum::SELL_SHORT_EXEMPT,
            "7" => SideEnum::UNDISCLOSED,
            "8" => SideEnum::CROSS,
            _ => SideEnum::Unknown(s),
        }
    }
}

impl fmt::Display for SideEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SideEnum::BUY => write!(f, "1"),
            SideEnum::SELL => write!(f, "2"),
            SideEnum::BUY_MINUS => write!(f, "3"),
            SideEnum::SELL_PLUS => write!(f, "4"),
            SideEnum::SELL_SHORT => write!(f, "5"),
            SideEnum::SELL_SHORT_EXEMPT => write!(f, "6"),
            SideEnum::UNDISCLOSED => write!(f, "7"),
            SideEnum::CROSS => write!(f, "8"),
            SideEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IOINaturalFlagEnum {
    /// NO
    NO,
    /// YES
    YES,
    /// Unknown value
    Unknown(String),
}

impl From<String> for IOINaturalFlagEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" => IOINaturalFlagEnum::NO,
            "Y" => IOINaturalFlagEnum::YES,
            _ => IOINaturalFlagEnum::Unknown(s),
        }
    }
}

impl fmt::Display for IOINaturalFlagEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOINaturalFlagEnum::NO => write!(f, "N"),
            IOINaturalFlagEnum::YES => write!(f, "Y"),
            IOINaturalFlagEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum StandInstDbTypeEnum {
    /// OTHER
    OTHER,
    /// DTCSID
    DTCSID,
    /// THOMSON_ALERT
    THOMSON_ALERT,
    /// A_GLOBAL_CUSTODIAN
    A_GLOBAL_CUSTODIAN,
    /// Unknown value
    Unknown(String),
}

impl From<String> for StandInstDbTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => StandInstDbTypeEnum::OTHER,
            "1" => StandInstDbTypeEnum::DTCSID,
            "2" => StandInstDbTypeEnum::THOMSON_ALERT,
            "3" => StandInstDbTypeEnum::A_GLOBAL_CUSTODIAN,
            _ => StandInstDbTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for StandInstDbTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StandInstDbTypeEnum::OTHER => write!(f, "0"),
            StandInstDbTypeEnum::DTCSID => write!(f, "1"),
            StandInstDbTypeEnum::THOMSON_ALERT => write!(f, "2"),
            StandInstDbTypeEnum::A_GLOBAL_CUSTODIAN => write!(f, "3"),
            StandInstDbTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CoveredOrUncoveredEnum {
    /// COVERED
    COVERED,
    /// UNCOVERED
    UNCOVERED,
    /// Unknown value
    Unknown(String),
}

impl From<String> for CoveredOrUncoveredEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => CoveredOrUncoveredEnum::COVERED,
            "1" => CoveredOrUncoveredEnum::UNCOVERED,
            _ => CoveredOrUncoveredEnum::Unknown(s),
        }
    }
}

impl fmt::Display for CoveredOrUncoveredEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoveredOrUncoveredEnum::COVERED => write!(f, "0"),
            CoveredOrUncoveredEnum::UNCOVERED => write!(f, "1"),
            CoveredOrUncoveredEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum OrdStatusEnum {
    /// NEW
    NEW,
    /// PARTIALLY_FILLED
    PARTIALLY_FILLED,
    /// FILLED
    FILLED,
    /// DONE_FOR_DAY
    DONE_FOR_DAY,
    /// CANCELED
    CANCELED,
    /// REPLACED
    REPLACED,
    /// PENDING_CANCEL
    PENDING_CANCEL,
    /// STOPPED
    STOPPED,
    /// REJECTED
    REJECTED,
    /// SUSPENDED
    SUSPENDED,
    /// PENDING_NEW
    PENDING_NEW,
    /// CALCULATED
    CALCULATED,
    /// EXPIRED
    EXPIRED,
    /// Unknown value
    Unknown(String),
}

impl From<String> for OrdStatusEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => OrdStatusEnum::NEW,
            "1" => OrdStatusEnum::PARTIALLY_FILLED,
            "2" => OrdStatusEnum::FILLED,
            "3" => OrdStatusEnum::DONE_FOR_DAY,
            "4" => OrdStatusEnum::CANCELED,
            "5" => OrdStatusEnum::REPLACED,
            "6" => OrdStatusEnum::PENDING_CANCEL,
            "7" => OrdStatusEnum::STOPPED,
            "8" => OrdStatusEnum::REJECTED,
            "9" => OrdStatusEnum::SUSPENDED,
            "A" => OrdStatusEnum::PENDING_NEW,
            "B" => OrdStatusEnum::CALCULATED,
            "C" => OrdStatusEnum::EXPIRED,
            _ => OrdStatusEnum::Unknown(s),
        }
    }
}

impl fmt::Display for OrdStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrdStatusEnum::NEW => write!(f, "0"),
            OrdStatusEnum::PARTIALLY_FILLED => write!(f, "1"),
            OrdStatusEnum::FILLED => write!(f, "2"),
            OrdStatusEnum::DONE_FOR_DAY => write!(f, "3"),
            OrdStatusEnum::CANCELED => write!(f, "4"),
            OrdStatusEnum::REPLACED => write!(f, "5"),
            OrdStatusEnum::PENDING_CANCEL => write!(f, "6"),
            OrdStatusEnum::STOPPED => write!(f, "7"),
            OrdStatusEnum::REJECTED => write!(f, "8"),
            OrdStatusEnum::SUSPENDED => write!(f, "9"),
            OrdStatusEnum::PENDING_NEW => write!(f, "A"),
            OrdStatusEnum::CALCULATED => write!(f, "B"),
            OrdStatusEnum::EXPIRED => write!(f, "C"),
            OrdStatusEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SettlInstSourceEnum {
    /// BROKER_CREDIT
    BROKER_CREDIT,
    /// INSTITUTION
    INSTITUTION,
    /// Unknown value
    Unknown(String),
}

impl From<String> for SettlInstSourceEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "1" => SettlInstSourceEnum::BROKER_CREDIT,
            "2" => SettlInstSourceEnum::INSTITUTION,
            _ => SettlInstSourceEnum::Unknown(s),
        }
    }
}

impl fmt::Display for SettlInstSourceEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettlInstSourceEnum::BROKER_CREDIT => write!(f, "1"),
            SettlInstSourceEnum::INSTITUTION => write!(f, "2"),
            SettlInstSourceEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ExecTransTypeEnum {
    /// NEW
    NEW,
    /// CANCEL
    CANCEL,
    /// CORRECT
    CORRECT,
    /// STATUS
    STATUS,
    /// Unknown value
    Unknown(String),
}

impl From<String> for ExecTransTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => ExecTransTypeEnum::NEW,
            "1" => ExecTransTypeEnum::CANCEL,
            "2" => ExecTransTypeEnum::CORRECT,
            "3" => ExecTransTypeEnum::STATUS,
            _ => ExecTransTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for ExecTransTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecTransTypeEnum::NEW => write!(f, "0"),
            ExecTransTypeEnum::CANCEL => write!(f, "1"),
            ExecTransTypeEnum::CORRECT => write!(f, "2"),
            ExecTransTypeEnum::STATUS => write!(f, "3"),
            ExecTransTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AllocHandlInstEnum {
    /// MATCH
    MATCH,
    /// FORWARD
    FORWARD,
    /// FORWARD_AND_MATCH
    FORWARD_AND_MATCH,
    /// Unknown value
    Unknown(String),
}

impl From<String> for AllocHandlInstEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "1" => AllocHandlInstEnum::MATCH,
            "2" => AllocHandlInstEnum::FORWARD,
            "3" => AllocHandlInstEnum::FORWARD_AND_MATCH,
            _ => AllocHandlInstEnum::Unknown(s),
        }
    }
}

impl fmt::Display for AllocHandlInstEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocHandlInstEnum::MATCH => write!(f, "1"),
            AllocHandlInstEnum::FORWARD => write!(f, "2"),
            AllocHandlInstEnum::FORWARD_AND_MATCH => write!(f, "3"),
            AllocHandlInstEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ReportToExchEnum {
    /// NO
    NO,
    /// YES
    YES,
    /// Unknown value
    Unknown(String),
}

impl From<String> for ReportToExchEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" => ReportToExchEnum::NO,
            "Y" => ReportToExchEnum::YES,
            _ => ReportToExchEnum::Unknown(s),
        }
    }
}

impl fmt::Display for ReportToExchEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportToExchEnum::NO => write!(f, "N"),
            ReportToExchEnum::YES => write!(f, "Y"),
            ReportToExchEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum OrdTypeEnum {
    /// MARKET
    MARKET,
    /// LIMIT
    LIMIT,
    /// STOP
    STOP,
    /// STOP_LIMIT
    STOP_LIMIT,
    /// MARKET_ON_CLOSE
    MARKET_ON_CLOSE,
    /// WITH_OR_WITHOUT
    WITH_OR_WITHOUT,
    /// LIMIT_OR_BETTER
    LIMIT_OR_BETTER,
    /// LIMIT_WITH_OR_WITHOUT
    LIMIT_WITH_OR_WITHOUT,
    /// ON_BASIS
    ON_BASIS,
    /// ON_CLOSE
    ON_CLOSE,
    /// LIMIT_ON_CLOSE
    LIMIT_ON_CLOSE,
    /// FOREX_MARKET
    FOREX_MARKET,
    /// PREVIOUSLY_QUOTED
    PREVIOUSLY_QUOTED,
    /// PREVIOUSLY_INDICATED
    PREVIOUSLY_INDICATED,
    /// FOREX_LIMIT
    FOREX_LIMIT,
    /// FOREX_SWAP
    FOREX_SWAP,
    /// FOREX_PREVIOUSLY_QUOTED
    FOREX_PREVIOUSLY_QUOTED,
    /// PEGGED
    PEGGED,
    /// Unknown value
    Unknown(String),
}

impl From<String> for OrdTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "1" => OrdTypeEnum::MARKET,
            "2" => OrdTypeEnum::LIMIT,
            "3" => OrdTypeEnum::STOP,
            "4" => OrdTypeEnum::STOP_LIMIT,
            "5" => OrdTypeEnum::MARKET_ON_CLOSE,
            "6" => OrdTypeEnum::WITH_OR_WITHOUT,
            "7" => OrdTypeEnum::LIMIT_OR_BETTER,
            "8" => OrdTypeEnum::LIMIT_WITH_OR_WITHOUT,
            "9" => OrdTypeEnum::ON_BASIS,
            "A" => OrdTypeEnum::ON_CLOSE,
            "B" => OrdTypeEnum::LIMIT_ON_CLOSE,
            "C" => OrdTypeEnum::FOREX_MARKET,
            "D" => OrdTypeEnum::PREVIOUSLY_QUOTED,
            "E" => OrdTypeEnum::PREVIOUSLY_INDICATED,
            "F" => OrdTypeEnum::FOREX_LIMIT,
            "G" => OrdTypeEnum::FOREX_SWAP,
            "H" => OrdTypeEnum::FOREX_PREVIOUSLY_QUOTED,
            "P" => OrdTypeEnum::PEGGED,
            _ => OrdTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for OrdTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrdTypeEnum::MARKET => write!(f, "1"),
            OrdTypeEnum::LIMIT => write!(f, "2"),
            OrdTypeEnum::STOP => write!(f, "3"),
            OrdTypeEnum::STOP_LIMIT => write!(f, "4"),
            OrdTypeEnum::MARKET_ON_CLOSE => write!(f, "5"),
            OrdTypeEnum::WITH_OR_WITHOUT => write!(f, "6"),
            OrdTypeEnum::LIMIT_OR_BETTER => write!(f, "7"),
            OrdTypeEnum::LIMIT_WITH_OR_WITHOUT => write!(f, "8"),
            OrdTypeEnum::ON_BASIS => write!(f, "9"),
            OrdTypeEnum::ON_CLOSE => write!(f, "A"),
            OrdTypeEnum::LIMIT_ON_CLOSE => write!(f, "B"),
            OrdTypeEnum::FOREX_MARKET => write!(f, "C"),
            OrdTypeEnum::PREVIOUSLY_QUOTED => write!(f, "D"),
            OrdTypeEnum::PREVIOUSLY_INDICATED => write!(f, "E"),
            OrdTypeEnum::FOREX_LIMIT => write!(f, "F"),
            OrdTypeEnum::FOREX_SWAP => write!(f, "G"),
            OrdTypeEnum::FOREX_PREVIOUSLY_QUOTED => write!(f, "H"),
            OrdTypeEnum::PEGGED => write!(f, "P"),
            OrdTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IOIQltyIndEnum {
    /// HIGH
    HIGH,
    /// LOW
    LOW,
    /// MEDIUM
    MEDIUM,
    /// Unknown value
    Unknown(String),
}

impl From<String> for IOIQltyIndEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "H" => IOIQltyIndEnum::HIGH,
            "L" => IOIQltyIndEnum::LOW,
            "M" => IOIQltyIndEnum::MEDIUM,
            _ => IOIQltyIndEnum::Unknown(s),
        }
    }
}

impl fmt::Display for IOIQltyIndEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOIQltyIndEnum::HIGH => write!(f, "H"),
            IOIQltyIndEnum::LOW => write!(f, "L"),
            IOIQltyIndEnum::MEDIUM => write!(f, "M"),
            IOIQltyIndEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum OrdRejReasonEnum {
    /// BROKER_CREDIT
    BROKER_CREDIT,
    /// UNKNOWN_SYMBOL
    UNKNOWN_SYMBOL,
    /// EXCHANGE_CLOSED
    EXCHANGE_CLOSED,
    /// ORDER_EXCEEDS_LIMIT
    ORDER_EXCEEDS_LIMIT,
    /// TOO_LATE_TO_ENTER
    TOO_LATE_TO_ENTER,
    /// UNKNOWN_ORDER
    UNKNOWN_ORDER,
    /// DUPLICATE_ORDER
    DUPLICATE_ORDER,
    /// Unknown value
    Unknown(String),
}

impl From<String> for OrdRejReasonEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => OrdRejReasonEnum::BROKER_CREDIT,
            "1" => OrdRejReasonEnum::UNKNOWN_SYMBOL,
            "2" => OrdRejReasonEnum::EXCHANGE_CLOSED,
            "3" => OrdRejReasonEnum::ORDER_EXCEEDS_LIMIT,
            "4" => OrdRejReasonEnum::TOO_LATE_TO_ENTER,
            "5" => OrdRejReasonEnum::UNKNOWN_ORDER,
            "6" => OrdRejReasonEnum::DUPLICATE_ORDER,
            _ => OrdRejReasonEnum::Unknown(s),
        }
    }
}

impl fmt::Display for OrdRejReasonEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrdRejReasonEnum::BROKER_CREDIT => write!(f, "0"),
            OrdRejReasonEnum::UNKNOWN_SYMBOL => write!(f, "1"),
            OrdRejReasonEnum::EXCHANGE_CLOSED => write!(f, "2"),
            OrdRejReasonEnum::ORDER_EXCEEDS_LIMIT => write!(f, "3"),
            OrdRejReasonEnum::TOO_LATE_TO_ENTER => write!(f, "4"),
            OrdRejReasonEnum::UNKNOWN_ORDER => write!(f, "5"),
            OrdRejReasonEnum::DUPLICATE_ORDER => write!(f, "6"),
            OrdRejReasonEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CustomerOrFirmEnum {
    /// CUSTOMER
    CUSTOMER,
    /// FIRM
    FIRM,
    /// Unknown value
    Unknown(String),
}

impl From<String> for CustomerOrFirmEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => CustomerOrFirmEnum::CUSTOMER,
            "1" => CustomerOrFirmEnum::FIRM,
            _ => CustomerOrFirmEnum::Unknown(s),
        }
    }
}

impl fmt::Display for CustomerOrFirmEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomerOrFirmEnum::CUSTOMER => write!(f, "0"),
            CustomerOrFirmEnum::FIRM => write!(f, "1"),
            CustomerOrFirmEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AdvSideEnum {
    /// BUY
    BUY,
    /// SELL
    SELL,
    /// TRADE
    TRADE,
    /// CROSS
    CROSS,
    /// Unknown value
    Unknown(String),
}

impl From<String> for AdvSideEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "B" => AdvSideEnum::BUY,
            "S" => AdvSideEnum::SELL,
            "T" => AdvSideEnum::TRADE,
            "X" => AdvSideEnum::CROSS,
            _ => AdvSideEnum::Unknown(s),
        }
    }
}

impl fmt::Display for AdvSideEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdvSideEnum::BUY => write!(f, "B"),
            AdvSideEnum::SELL => write!(f, "S"),
            AdvSideEnum::TRADE => write!(f, "T"),
            AdvSideEnum::CROSS => write!(f, "X"),
            AdvSideEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MiscFeeTypeEnum {
    /// REGULATORY
    REGULATORY,
    /// TAX
    TAX,
    /// LOCAL_COMMISSION
    LOCAL_COMMISSION,
    /// EXCHANGE_FEES
    EXCHANGE_FEES,
    /// STAMP
    STAMP,
    /// LEVY
    LEVY,
    /// OTHER
    OTHER,
    /// MARKUP
    MARKUP,
    /// Unknown value
    Unknown(String),
}

impl From<String> for MiscFeeTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "1" => MiscFeeTypeEnum::REGULATORY,
            "2" => MiscFeeTypeEnum::TAX,
            "3" => MiscFeeTypeEnum::LOCAL_COMMISSION,
            "4" => MiscFeeTypeEnum::EXCHANGE_FEES,
            "5" => MiscFeeTypeEnum::STAMP,
            "6" => MiscFeeTypeEnum::LEVY,
            "7" => MiscFeeTypeEnum::OTHER,
            "8" => MiscFeeTypeEnum::MARKUP,
            _ => MiscFeeTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for MiscFeeTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MiscFeeTypeEnum::REGULATORY => write!(f, "1"),
            MiscFeeTypeEnum::TAX => write!(f, "2"),
            MiscFeeTypeEnum::LOCAL_COMMISSION => write!(f, "3"),
            MiscFeeTypeEnum::EXCHANGE_FEES => write!(f, "4"),
            MiscFeeTypeEnum::STAMP => write!(f, "5"),
            MiscFeeTypeEnum::LEVY => write!(f, "6"),
            MiscFeeTypeEnum::OTHER => write!(f, "7"),
            MiscFeeTypeEnum::MARKUP => write!(f, "8"),
            MiscFeeTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LastCapacityEnum {
    /// AGENT
    AGENT,
    /// CROSS_AS_AGENT
    CROSS_AS_AGENT,
    /// CROSS_AS_PRINCIPAL
    CROSS_AS_PRINCIPAL,
    /// PRINCIPAL
    PRINCIPAL,
    /// Unknown value
    Unknown(String),
}

impl From<String> for LastCapacityEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "1" => LastCapacityEnum::AGENT,
            "2" => LastCapacityEnum::CROSS_AS_AGENT,
            "3" => LastCapacityEnum::CROSS_AS_PRINCIPAL,
            "4" => LastCapacityEnum::PRINCIPAL,
            _ => LastCapacityEnum::Unknown(s),
        }
    }
}

impl fmt::Display for LastCapacityEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LastCapacityEnum::AGENT => write!(f, "1"),
            LastCapacityEnum::CROSS_AS_AGENT => write!(f, "2"),
            LastCapacityEnum::CROSS_AS_PRINCIPAL => write!(f, "3"),
            LastCapacityEnum::PRINCIPAL => write!(f, "4"),
            LastCapacityEnum::Unknown(s) => write!(f, "{s}"),
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
#[allow(non_camel_case_types)]
pub enum AllocTransTypeEnum {
    /// NEW
    NEW,
    /// REPLACE
    REPLACE,
    /// CANCEL
    CANCEL,
    /// PRELIMINARY
    PRELIMINARY,
    /// CALCULATED
    CALCULATED,
    /// Unknown value
    Unknown(String),
}

impl From<String> for AllocTransTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => AllocTransTypeEnum::NEW,
            "1" => AllocTransTypeEnum::REPLACE,
            "2" => AllocTransTypeEnum::CANCEL,
            "3" => AllocTransTypeEnum::PRELIMINARY,
            "4" => AllocTransTypeEnum::CALCULATED,
            _ => AllocTransTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for AllocTransTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocTransTypeEnum::NEW => write!(f, "0"),
            AllocTransTypeEnum::REPLACE => write!(f, "1"),
            AllocTransTypeEnum::CANCEL => write!(f, "2"),
            AllocTransTypeEnum::PRELIMINARY => write!(f, "3"),
            AllocTransTypeEnum::CALCULATED => write!(f, "4"),
            AllocTransTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SettlInstModeEnum {
    /// DEFAULT
    DEFAULT,
    /// STANDING_INSTRUCTIONS_PROVIDED
    STANDING_INSTRUCTIONS_PROVIDED,
    /// SPECIFIC_ALLOCATION_ACCOUNT_OVERRIDING
    SPECIFIC_ALLOCATION_ACCOUNT_OVERRIDING,
    /// SPECIFIC_ALLOCATION_ACCOUNT_STANDING
    SPECIFIC_ALLOCATION_ACCOUNT_STANDING,
    /// Unknown value
    Unknown(String),
}

impl From<String> for SettlInstModeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => SettlInstModeEnum::DEFAULT,
            "1" => SettlInstModeEnum::STANDING_INSTRUCTIONS_PROVIDED,
            "2" => SettlInstModeEnum::SPECIFIC_ALLOCATION_ACCOUNT_OVERRIDING,
            "3" => SettlInstModeEnum::SPECIFIC_ALLOCATION_ACCOUNT_STANDING,
            _ => SettlInstModeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for SettlInstModeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettlInstModeEnum::DEFAULT => write!(f, "0"),
            SettlInstModeEnum::STANDING_INSTRUCTIONS_PROVIDED => write!(f, "1"),
            SettlInstModeEnum::SPECIFIC_ALLOCATION_ACCOUNT_OVERRIDING => write!(f, "2"),
            SettlInstModeEnum::SPECIFIC_ALLOCATION_ACCOUNT_STANDING => write!(f, "3"),
            SettlInstModeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CommTypeEnum {
    /// PER_UNIT
    PER_UNIT,
    /// PERCENT
    PERCENT,
    /// ABSOLUTE
    ABSOLUTE,
    /// Unknown value
    Unknown(String),
}

impl From<String> for CommTypeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "1" => CommTypeEnum::PER_UNIT,
            "2" => CommTypeEnum::PERCENT,
            "3" => CommTypeEnum::ABSOLUTE,
            _ => CommTypeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for CommTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommTypeEnum::PER_UNIT => write!(f, "1"),
            CommTypeEnum::PERCENT => write!(f, "2"),
            CommTypeEnum::ABSOLUTE => write!(f, "3"),
            CommTypeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PutOrCallEnum {
    /// PUT
    PUT,
    /// CALL
    CALL,
    /// Unknown value
    Unknown(String),
}

impl From<String> for PutOrCallEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => PutOrCallEnum::PUT,
            "1" => PutOrCallEnum::CALL,
            _ => PutOrCallEnum::Unknown(s),
        }
    }
}

impl fmt::Display for PutOrCallEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PutOrCallEnum::PUT => write!(f, "0"),
            PutOrCallEnum::CALL => write!(f, "1"),
            PutOrCallEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AllocStatusEnum {
    /// ACCEPTED
    ACCEPTED,
    /// BLOCK_LEVEL_REJECT
    BLOCK_LEVEL_REJECT,
    /// ACCOUNT_LEVEL_REJECT
    ACCOUNT_LEVEL_REJECT,
    /// RECEIVED
    RECEIVED,
    /// Unknown value
    Unknown(String),
}

impl From<String> for AllocStatusEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => AllocStatusEnum::ACCEPTED,
            "1" => AllocStatusEnum::BLOCK_LEVEL_REJECT,
            "2" => AllocStatusEnum::ACCOUNT_LEVEL_REJECT,
            "3" => AllocStatusEnum::RECEIVED,
            _ => AllocStatusEnum::Unknown(s),
        }
    }
}

impl fmt::Display for AllocStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocStatusEnum::ACCEPTED => write!(f, "0"),
            AllocStatusEnum::BLOCK_LEVEL_REJECT => write!(f, "1"),
            AllocStatusEnum::ACCOUNT_LEVEL_REJECT => write!(f, "2"),
            AllocStatusEnum::RECEIVED => write!(f, "3"),
            AllocStatusEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AllocRejCodeEnum {
    /// UNKNOWN_ACCOUNT
    UNKNOWN_ACCOUNT,
    /// INCORRECT_QUANTITY
    INCORRECT_QUANTITY,
    /// INCORRECT_AVERAGEG_PRICE
    INCORRECT_AVERAGEG_PRICE,
    /// UNKNOWN_EXECUTING_BROKER_MNEMONIC
    UNKNOWN_EXECUTING_BROKER_MNEMONIC,
    /// COMMISSION_DIFFERENCE
    COMMISSION_DIFFERENCE,
    /// UNKNOWN_ORDER_ID
    UNKNOWN_ORDER_ID,
    /// UNKNOWN_LIST_ID
    UNKNOWN_LIST_ID,
    /// OTHER_SEE_TEXT
    OTHER_SEE_TEXT,
    /// Unknown value
    Unknown(String),
}

impl From<String> for AllocRejCodeEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => AllocRejCodeEnum::UNKNOWN_ACCOUNT,
            "1" => AllocRejCodeEnum::INCORRECT_QUANTITY,
            "2" => AllocRejCodeEnum::INCORRECT_AVERAGEG_PRICE,
            "3" => AllocRejCodeEnum::UNKNOWN_EXECUTING_BROKER_MNEMONIC,
            "4" => AllocRejCodeEnum::COMMISSION_DIFFERENCE,
            "5" => AllocRejCodeEnum::UNKNOWN_ORDER_ID,
            "6" => AllocRejCodeEnum::UNKNOWN_LIST_ID,
            "7" => AllocRejCodeEnum::OTHER_SEE_TEXT,
            _ => AllocRejCodeEnum::Unknown(s),
        }
    }
}

impl fmt::Display for AllocRejCodeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocRejCodeEnum::UNKNOWN_ACCOUNT => write!(f, "0"),
            AllocRejCodeEnum::INCORRECT_QUANTITY => write!(f, "1"),
            AllocRejCodeEnum::INCORRECT_AVERAGEG_PRICE => write!(f, "2"),
            AllocRejCodeEnum::UNKNOWN_EXECUTING_BROKER_MNEMONIC => write!(f, "3"),
            AllocRejCodeEnum::COMMISSION_DIFFERENCE => write!(f, "4"),
            AllocRejCodeEnum::UNKNOWN_ORDER_ID => write!(f, "5"),
            AllocRejCodeEnum::UNKNOWN_LIST_ID => write!(f, "6"),
            AllocRejCodeEnum::OTHER_SEE_TEXT => write!(f, "7"),
            AllocRejCodeEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IOISharesEnum {
    /// LARGE
    LARGE,
    /// MEDIUM
    MEDIUM,
    /// SMALL
    SMALL,
    /// Unknown value
    Unknown(String),
}

impl From<String> for IOISharesEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "L" => IOISharesEnum::LARGE,
            "M" => IOISharesEnum::MEDIUM,
            "S" => IOISharesEnum::SMALL,
            _ => IOISharesEnum::Unknown(s),
        }
    }
}

impl fmt::Display for IOISharesEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOISharesEnum::LARGE => write!(f, "L"),
            IOISharesEnum::MEDIUM => write!(f, "M"),
            IOISharesEnum::SMALL => write!(f, "S"),
            IOISharesEnum::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagValue {
    /// TransactTime - Field 60
    TransactTime(String),
    /// NewSeqNo - Field 36
    NewSeqNo(i64),
    /// OrigSendingTime - Field 122
    OrigSendingTime(String),
    /// SettlCurrFxRate - Field 155
    SettlCurrFxRate(f64),
    /// MaturityDay - Field 205
    MaturityDay(u8),
    /// ListExecInst - Field 69
    ListExecInst(String),
    /// RelatdSym - Field 46
    RelatdSym(String),
    /// GapFillFlag - Field 123
    GapFillFlag(GapFillFlagEnum),
    /// AdvId - Field 2
    AdvId(String),
    /// Shares - Field 53
    Shares(i64),
    /// CheckSum - Field 10
    CheckSum(String),
    /// SignatureLength - Field 93
    SignatureLength(i64),
    /// SenderCompID - Field 49
    SenderCompID(String),
    /// NoOrders - Field 73
    NoOrders(i64),
    /// CashOrderQty - Field 152
    CashOrderQty(f64),
    /// BidSpotRate - Field 188
    BidSpotRate(f64),
    /// NoIOIQualifiers - Field 199
    NoIOIQualifiers(i64),
    /// AdvTransType - Field 5
    AdvTransType(AdvTransTypeEnum),
    /// Commission - Field 12
    Commission(f64),
    /// PossResend - Field 97
    PossResend(PossResendEnum),
    /// OpenClose - Field 77
    OpenClose(OpenCloseEnum),
    /// NoAllocs - Field 78
    NoAllocs(i64),
    /// QuoteID - Field 117
    QuoteID(String),
    /// BeginString - Field 8
    BeginString(String),
    /// SettlmntTyp - Field 63
    SettlmntTyp(SettlmntTypEnum),
    /// OfferPx - Field 133
    OfferPx(f64),
    /// EndSeqNo - Field 16
    EndSeqNo(i64),
    /// DeliverToLocationID - Field 145
    DeliverToLocationID(String),
    /// SecurityType - Field 167
    SecurityType(SecurityTypeEnum),
    /// MaturityMonthYear - Field 200
    MaturityMonthYear(String),
    /// StrikePrice - Field 202
    StrikePrice(f64),
    /// DKReason - Field 127
    DKReason(DKReasonEnum),
    /// ExecType - Field 150
    ExecType(ExecTypeEnum),
    /// Headline - Field 148
    Headline(String),
    /// LocateReqd - Field 114
    LocateReqd(LocateReqdEnum),
    /// SettlLocation - Field 166
    SettlLocation(SettlLocationEnum),
    /// NoMiscFees - Field 136
    NoMiscFees(i64),
    /// AllocAvgPx - Field 153
    AllocAvgPx(f64),
    /// AllocAccount - Field 79
    AllocAccount(String),
    /// Account - Field 1
    Account(String),
    /// TargetLocationID - Field 143
    TargetLocationID(String),
    /// TimeInForce - Field 59
    TimeInForce(TimeInForceEnum),
    /// CashSettlAgentCode - Field 183
    CashSettlAgentCode(String),
    /// AllocLinkID - Field 196
    AllocLinkID(String),
    /// PossDupFlag - Field 43
    PossDupFlag(PossDupFlagEnum),
    /// ValidUntilTime - Field 62
    ValidUntilTime(String),
    /// MinQty - Field 110
    MinQty(i64),
    /// LinesOfText - Field 33
    LinesOfText(i64),
    /// MsgType - Field 35
    MsgType(MsgTypeEnum),
    /// LastPx - Field 31
    LastPx(f64),
    /// TargetSubID - Field 57
    TargetSubID(String),
    /// ListSeqNo - Field 67
    ListSeqNo(i64),
    /// OrderQty - Field 38
    OrderQty(i64),
    /// AvgPrxPrecision - Field 74
    AvgPrxPrecision(i64),
    /// SecureDataLen - Field 90
    SecureDataLen(i64),
    /// SecurityDesc - Field 107
    SecurityDesc(String),
    /// SecuritySettlAgentName - Field 176
    SecuritySettlAgentName(String),
    /// MsgSeqNum - Field 34
    MsgSeqNum(i64),
    /// SecurityID - Field 48
    SecurityID(String),
    /// LeavesQty - Field 151
    LeavesQty(i64),
    /// SettlCurrFxRateCalc - Field 156
    SettlCurrFxRateCalc(String),
    /// RefAllocID - Field 72
    RefAllocID(String),
    /// CashSettlAgentContactName - Field 186
    CashSettlAgentContactName(String),
    /// AdvRefID - Field 3
    AdvRefID(String),
    /// Symbol - Field 55
    Symbol(String),
    /// SecuritySettlAgentContactPhone - Field 181
    SecuritySettlAgentContactPhone(String),
    /// EmailType - Field 94
    EmailType(EmailTypeEnum),
    /// IOITransType - Field 28
    IOITransType(IOITransTypeEnum),
    /// ExecInst - Field 18
    ExecInst(ExecInstEnum),
    /// SecuritySettlAgentContactName - Field 180
    SecuritySettlAgentContactName(String),
    /// StopPx - Field 99
    StopPx(f64),
    /// WaveNo - Field 105
    WaveNo(String),
    /// OnBehalfOfSubID - Field 116
    OnBehalfOfSubID(String),
    /// SecurityExchange - Field 207
    SecurityExchange(String),
    /// ExDestination - Field 100
    ExDestination(String),
    /// BeginSeqNo - Field 7
    BeginSeqNo(i64),
    /// IOIRefID - Field 26
    IOIRefID(String),
    /// CxlQty - Field 84
    CxlQty(i64),
    /// ForexReq - Field 121
    ForexReq(ForexReqEnum),
    /// SettlInstCode - Field 175
    SettlInstCode(String),
    /// SecuritySettlAgentAcctNum - Field 178
    SecuritySettlAgentAcctNum(String),
    /// CashSettlAgentAcctNum - Field 184
    CashSettlAgentAcctNum(String),
    /// FutSettDate2 - Field 193
    FutSettDate2(String),
    /// IOIQualifier - Field 104
    IOIQualifier(IOIQualifierEnum),
    /// Rule80A - Field 47
    Rule80A(Rule80AEnum),
    /// AccruedInterestRate - Field 158
    AccruedInterestRate(f64),
    /// SecureData - Field 91
    SecureData(Vec<u8>),
    /// EffectiveTime - Field 168
    EffectiveTime(String),
    /// ProcessCode - Field 81
    ProcessCode(ProcessCodeEnum),
    /// BidForwardPoints - Field 189
    BidForwardPoints(f64),
    /// Urgency - Field 61
    Urgency(UrgencyEnum),
    /// NotifyBrokerOfCredit - Field 208
    NotifyBrokerOfCredit(NotifyBrokerOfCreditEnum),
    /// OfferSpotRate - Field 190
    OfferSpotRate(f64),
    /// ExecID - Field 17
    ExecID(String),
    /// ListNoOrds - Field 68
    ListNoOrds(i64),
    /// OrigTime - Field 42
    OrigTime(String),
    /// SettlInstTransType - Field 163
    SettlInstTransType(SettlInstTransTypeEnum),
    /// BodyLength - Field 9
    BodyLength(i64),
    /// SettlInstID - Field 162
    SettlInstID(String),
    /// OnBehalfOfCompID - Field 115
    OnBehalfOfCompID(String),
    /// Price - Field 44
    Price(f64),
    /// OrigClOrdID - Field 41
    OrigClOrdID(String),
    /// HandlInst - Field 21
    HandlInst(HandlInstEnum),
    /// PrevClosePx - Field 140
    PrevClosePx(f64),
    /// IOIOthSvc - Field 24
    IOIOthSvc(IOIOthSvcEnum),
    /// Signature - Field 89
    Signature(Vec<u8>),
    /// TestReqID - Field 112
    TestReqID(String),
    /// BidSize - Field 134
    BidSize(i64),
    /// AllocNetMoney - Field 154
    AllocNetMoney(f64),
    /// OptAttribute - Field 206
    OptAttribute(String),
    /// IDSource - Field 22
    IDSource(IDSourceEnum),
    /// SecuritySettlAgentAcctName - Field 179
    SecuritySettlAgentAcctName(String),
    /// AllocLinkType - Field 197
    AllocLinkType(AllocLinkTypeEnum),
    /// EncryptMethod - Field 98
    EncryptMethod(EncryptMethodEnum),
    /// AllocID - Field 70
    AllocID(String),
    /// CxlRejReason - Field 102
    CxlRejReason(CxlRejReasonEnum),
    /// Side - Field 54
    Side(SideEnum),
    /// ClientID - Field 109
    ClientID(String),
    /// IOINaturalFlag - Field 130
    IOINaturalFlag(IOINaturalFlagEnum),
    /// ExecBroker - Field 76
    ExecBroker(String),
    /// StandInstDbType - Field 169
    StandInstDbType(StandInstDbTypeEnum),
    /// CoveredOrUncovered - Field 203
    CoveredOrUncovered(CoveredOrUncoveredEnum),
    /// MaxShow - Field 210
    MaxShow(i64),
    /// Currency - Field 15
    Currency(String),
    /// OrdStatus - Field 39
    OrdStatus(OrdStatusEnum),
    /// SettlInstSource - Field 165
    SettlInstSource(SettlInstSourceEnum),
    /// SettlBrkrCode - Field 174
    SettlBrkrCode(String),
    /// TargetCompID - Field 56
    TargetCompID(String),
    /// CumQty - Field 14
    CumQty(i64),
    /// ExpireTime - Field 126
    ExpireTime(String),
    /// TradeDate - Field 75
    TradeDate(String),
    /// Subject - Field 147
    Subject(String),
    /// ExecTransType - Field 20
    ExecTransType(ExecTransTypeEnum),
    /// SenderLocationID - Field 142
    SenderLocationID(String),
    /// SettlCurrency - Field 120
    SettlCurrency(String),
    /// AllocHandlInst - Field 209
    AllocHandlInst(AllocHandlInstEnum),
    /// PegDifference - Field 211
    PegDifference(f64),
    /// QuoteReqID - Field 131
    QuoteReqID(String),
    /// ReportToExch - Field 113
    ReportToExch(ReportToExchEnum),
    /// SenderSubID - Field 50
    SenderSubID(String),
    /// IOIid - Field 23
    IOIid(String),
    /// AllocText - Field 161
    AllocText(String),
    /// OrdType - Field 40
    OrdType(OrdTypeEnum),
    /// IOIQltyInd - Field 25
    IOIQltyInd(IOIQltyIndEnum),
    /// LastMkt - Field 30
    LastMkt(String),
    /// AllocShares - Field 80
    AllocShares(i64),
    /// RptSeq - Field 83
    RptSeq(i64),
    /// MaxFloor - Field 111
    MaxFloor(i64),
    /// OrdRejReason - Field 103
    OrdRejReason(OrdRejReasonEnum),
    /// NoExecs - Field 124
    NoExecs(i64),
    /// BidPx - Field 132
    BidPx(f64),
    /// Issuer - Field 106
    Issuer(String),
    /// OfferSize - Field 135
    OfferSize(i64),
    /// LastForwardPoints - Field 195
    LastForwardPoints(f64),
    /// CustomerOrFirm - Field 204
    CustomerOrFirm(CustomerOrFirmEnum),
    /// AdvSide - Field 4
    AdvSide(AdvSideEnum),
    /// MiscFeeType - Field 139
    MiscFeeType(MiscFeeTypeEnum),
    /// LastCapacity - Field 29
    LastCapacity(LastCapacityEnum),
    /// ResetSeqNumFlag - Field 141
    ResetSeqNumFlag(ResetSeqNumFlagEnum),
    /// NoRelatedSym - Field 146
    NoRelatedSym(i64),
    /// RefSeqNum - Field 45
    RefSeqNum(i64),
    /// AllocTransType - Field 71
    AllocTransType(AllocTransTypeEnum),
    /// HeartBtInt - Field 108
    HeartBtInt(i64),
    /// MiscFeeCurr - Field 138
    MiscFeeCurr(String),
    /// ClOrdID - Field 11
    ClOrdID(String),
    /// SymbolSfx - Field 65
    SymbolSfx(String),
    /// LastShares - Field 32
    LastShares(i64),
    /// URLLink - Field 149
    URLLink(String),
    /// OfferForwardPoints - Field 191
    OfferForwardPoints(f64),
    /// SettlCurrAmt - Field 119
    SettlCurrAmt(f64),
    /// AccruedInterestAmt - Field 159
    AccruedInterestAmt(f64),
    /// RawDataLength - Field 95
    RawDataLength(i64),
    /// OnBehalfOfLocationID - Field 144
    OnBehalfOfLocationID(String),
    /// SettlDepositoryCode - Field 173
    SettlDepositoryCode(String),
    /// SecuritySettlAgentCode - Field 177
    SecuritySettlAgentCode(String),
    /// CashSettlAgentName - Field 182
    CashSettlAgentName(String),
    /// AvgPx - Field 6
    AvgPx(f64),
    /// FutSettDate - Field 64
    FutSettDate(String),
    /// SettlInstMode - Field 160
    SettlInstMode(SettlInstModeEnum),
    /// CommType - Field 13
    CommType(CommTypeEnum),
    /// NetMoney - Field 118
    NetMoney(f64),
    /// StandInstDbID - Field 171
    StandInstDbID(String),
    /// RawData - Field 96
    RawData(Vec<u8>),
    /// Text - Field 58
    Text(String),
    /// OrderID - Field 37
    OrderID(String),
    /// DeliverToSubID - Field 129
    DeliverToSubID(String),
    /// EmailThreadID - Field 164
    EmailThreadID(String),
    /// StandInstDbName - Field 170
    StandInstDbName(String),
    /// SettlDeliveryType - Field 172
    SettlDeliveryType(i64),
    /// CashSettlAgentAcctName - Field 185
    CashSettlAgentAcctName(String),
    /// NoRpts - Field 82
    NoRpts(i64),
    /// MiscFeeAmt - Field 137
    MiscFeeAmt(f64),
    /// ListID - Field 66
    ListID(String),
    /// CashSettlAgentContactPhone - Field 187
    CashSettlAgentContactPhone(String),
    /// LastSpotRate - Field 194
    LastSpotRate(f64),
    /// SecondaryOrderID - Field 198
    SecondaryOrderID(String),
    /// NumDaysInterest - Field 157
    NumDaysInterest(i64),
    /// BrokerOfCredit - Field 92
    BrokerOfCredit(String),
    /// PutOrCall - Field 201
    PutOrCall(PutOrCallEnum),
    /// ExecRefID - Field 19
    ExecRefID(String),
    /// DeliverToCompID - Field 128
    DeliverToCompID(String),
    /// AllocStatus - Field 87
    AllocStatus(AllocStatusEnum),
    /// AllocRejCode - Field 88
    AllocRejCode(AllocRejCodeEnum),
    /// OrderQty2 - Field 192
    OrderQty2(f64),
    /// SendingTime - Field 52
    SendingTime(String),
    /// IOIShares - Field 27
    IOIShares(IOISharesEnum),
}

// Tag number constants
pub const TAG_TRANSACTTIME: u32 = 60;
pub const TAG_NEWSEQNO: u32 = 36;
pub const TAG_ORIGSENDINGTIME: u32 = 122;
pub const TAG_SETTLCURRFXRATE: u32 = 155;
pub const TAG_MATURITYDAY: u32 = 205;
pub const TAG_LISTEXECINST: u32 = 69;
pub const TAG_RELATDSYM: u32 = 46;
pub const TAG_GAPFILLFLAG: u32 = 123;
pub const TAG_ADVID: u32 = 2;
pub const TAG_SHARES: u32 = 53;
pub const TAG_CHECKSUM: u32 = 10;
pub const TAG_SIGNATURELENGTH: u32 = 93;
pub const TAG_SENDERCOMPID: u32 = 49;
pub const TAG_NOORDERS: u32 = 73;
pub const TAG_CASHORDERQTY: u32 = 152;
pub const TAG_BIDSPOTRATE: u32 = 188;
pub const TAG_NOIOIQUALIFIERS: u32 = 199;
pub const TAG_ADVTRANSTYPE: u32 = 5;
pub const TAG_COMMISSION: u32 = 12;
pub const TAG_POSSRESEND: u32 = 97;
pub const TAG_OPENCLOSE: u32 = 77;
pub const TAG_NOALLOCS: u32 = 78;
pub const TAG_QUOTEID: u32 = 117;
pub const TAG_BEGINSTRING: u32 = 8;
pub const TAG_SETTLMNTTYP: u32 = 63;
pub const TAG_OFFERPX: u32 = 133;
pub const TAG_ENDSEQNO: u32 = 16;
pub const TAG_DELIVERTOLOCATIONID: u32 = 145;
pub const TAG_SECURITYTYPE: u32 = 167;
pub const TAG_MATURITYMONTHYEAR: u32 = 200;
pub const TAG_STRIKEPRICE: u32 = 202;
pub const TAG_DKREASON: u32 = 127;
pub const TAG_EXECTYPE: u32 = 150;
pub const TAG_HEADLINE: u32 = 148;
pub const TAG_LOCATEREQD: u32 = 114;
pub const TAG_SETTLLOCATION: u32 = 166;
pub const TAG_NOMISCFEES: u32 = 136;
pub const TAG_ALLOCAVGPX: u32 = 153;
pub const TAG_ALLOCACCOUNT: u32 = 79;
pub const TAG_ACCOUNT: u32 = 1;
pub const TAG_TARGETLOCATIONID: u32 = 143;
pub const TAG_TIMEINFORCE: u32 = 59;
pub const TAG_CASHSETTLAGENTCODE: u32 = 183;
pub const TAG_ALLOCLINKID: u32 = 196;
pub const TAG_POSSDUPFLAG: u32 = 43;
pub const TAG_VALIDUNTILTIME: u32 = 62;
pub const TAG_MINQTY: u32 = 110;
pub const TAG_LINESOFTEXT: u32 = 33;
pub const TAG_MSGTYPE: u32 = 35;
pub const TAG_LASTPX: u32 = 31;
pub const TAG_TARGETSUBID: u32 = 57;
pub const TAG_LISTSEQNO: u32 = 67;
pub const TAG_ORDERQTY: u32 = 38;
pub const TAG_AVGPRXPRECISION: u32 = 74;
pub const TAG_SECUREDATALEN: u32 = 90;
pub const TAG_SECURITYDESC: u32 = 107;
pub const TAG_SECURITYSETTLAGENTNAME: u32 = 176;
pub const TAG_MSGSEQNUM: u32 = 34;
pub const TAG_SECURITYID: u32 = 48;
pub const TAG_LEAVESQTY: u32 = 151;
pub const TAG_SETTLCURRFXRATECALC: u32 = 156;
pub const TAG_REFALLOCID: u32 = 72;
pub const TAG_CASHSETTLAGENTCONTACTNAME: u32 = 186;
pub const TAG_ADVREFID: u32 = 3;
pub const TAG_SYMBOL: u32 = 55;
pub const TAG_SECURITYSETTLAGENTCONTACTPHONE: u32 = 181;
pub const TAG_EMAILTYPE: u32 = 94;
pub const TAG_IOITRANSTYPE: u32 = 28;
pub const TAG_EXECINST: u32 = 18;
pub const TAG_SECURITYSETTLAGENTCONTACTNAME: u32 = 180;
pub const TAG_STOPPX: u32 = 99;
pub const TAG_WAVENO: u32 = 105;
pub const TAG_ONBEHALFOFSUBID: u32 = 116;
pub const TAG_SECURITYEXCHANGE: u32 = 207;
pub const TAG_EXDESTINATION: u32 = 100;
pub const TAG_BEGINSEQNO: u32 = 7;
pub const TAG_IOIREFID: u32 = 26;
pub const TAG_CXLQTY: u32 = 84;
pub const TAG_FOREXREQ: u32 = 121;
pub const TAG_SETTLINSTCODE: u32 = 175;
pub const TAG_SECURITYSETTLAGENTACCTNUM: u32 = 178;
pub const TAG_CASHSETTLAGENTACCTNUM: u32 = 184;
pub const TAG_FUTSETTDATE2: u32 = 193;
pub const TAG_IOIQUALIFIER: u32 = 104;
pub const TAG_RULE80A: u32 = 47;
pub const TAG_ACCRUEDINTERESTRATE: u32 = 158;
pub const TAG_SECUREDATA: u32 = 91;
pub const TAG_EFFECTIVETIME: u32 = 168;
pub const TAG_PROCESSCODE: u32 = 81;
pub const TAG_BIDFORWARDPOINTS: u32 = 189;
pub const TAG_URGENCY: u32 = 61;
pub const TAG_NOTIFYBROKEROFCREDIT: u32 = 208;
pub const TAG_OFFERSPOTRATE: u32 = 190;
pub const TAG_EXECID: u32 = 17;
pub const TAG_LISTNOORDS: u32 = 68;
pub const TAG_ORIGTIME: u32 = 42;
pub const TAG_SETTLINSTTRANSTYPE: u32 = 163;
pub const TAG_BODYLENGTH: u32 = 9;
pub const TAG_SETTLINSTID: u32 = 162;
pub const TAG_ONBEHALFOFCOMPID: u32 = 115;
pub const TAG_PRICE: u32 = 44;
pub const TAG_ORIGCLORDID: u32 = 41;
pub const TAG_HANDLINST: u32 = 21;
pub const TAG_PREVCLOSEPX: u32 = 140;
pub const TAG_IOIOTHSVC: u32 = 24;
pub const TAG_SIGNATURE: u32 = 89;
pub const TAG_TESTREQID: u32 = 112;
pub const TAG_BIDSIZE: u32 = 134;
pub const TAG_ALLOCNETMONEY: u32 = 154;
pub const TAG_OPTATTRIBUTE: u32 = 206;
pub const TAG_IDSOURCE: u32 = 22;
pub const TAG_SECURITYSETTLAGENTACCTNAME: u32 = 179;
pub const TAG_ALLOCLINKTYPE: u32 = 197;
pub const TAG_ENCRYPTMETHOD: u32 = 98;
pub const TAG_ALLOCID: u32 = 70;
pub const TAG_CXLREJREASON: u32 = 102;
pub const TAG_SIDE: u32 = 54;
pub const TAG_CLIENTID: u32 = 109;
pub const TAG_IOINATURALFLAG: u32 = 130;
pub const TAG_EXECBROKER: u32 = 76;
pub const TAG_STANDINSTDBTYPE: u32 = 169;
pub const TAG_COVEREDORUNCOVERED: u32 = 203;
pub const TAG_MAXSHOW: u32 = 210;
pub const TAG_CURRENCY: u32 = 15;
pub const TAG_ORDSTATUS: u32 = 39;
pub const TAG_SETTLINSTSOURCE: u32 = 165;
pub const TAG_SETTLBRKRCODE: u32 = 174;
pub const TAG_TARGETCOMPID: u32 = 56;
pub const TAG_CUMQTY: u32 = 14;
pub const TAG_EXPIRETIME: u32 = 126;
pub const TAG_TRADEDATE: u32 = 75;
pub const TAG_SUBJECT: u32 = 147;
pub const TAG_EXECTRANSTYPE: u32 = 20;
pub const TAG_SENDERLOCATIONID: u32 = 142;
pub const TAG_SETTLCURRENCY: u32 = 120;
pub const TAG_ALLOCHANDLINST: u32 = 209;
pub const TAG_PEGDIFFERENCE: u32 = 211;
pub const TAG_QUOTEREQID: u32 = 131;
pub const TAG_REPORTTOEXCH: u32 = 113;
pub const TAG_SENDERSUBID: u32 = 50;
pub const TAG_IOIID: u32 = 23;
pub const TAG_ALLOCTEXT: u32 = 161;
pub const TAG_ORDTYPE: u32 = 40;
pub const TAG_IOIQLTYIND: u32 = 25;
pub const TAG_LASTMKT: u32 = 30;
pub const TAG_ALLOCSHARES: u32 = 80;
pub const TAG_RPTSEQ: u32 = 83;
pub const TAG_MAXFLOOR: u32 = 111;
pub const TAG_ORDREJREASON: u32 = 103;
pub const TAG_NOEXECS: u32 = 124;
pub const TAG_BIDPX: u32 = 132;
pub const TAG_ISSUER: u32 = 106;
pub const TAG_OFFERSIZE: u32 = 135;
pub const TAG_LASTFORWARDPOINTS: u32 = 195;
pub const TAG_CUSTOMERORFIRM: u32 = 204;
pub const TAG_ADVSIDE: u32 = 4;
pub const TAG_MISCFEETYPE: u32 = 139;
pub const TAG_LASTCAPACITY: u32 = 29;
pub const TAG_RESETSEQNUMFLAG: u32 = 141;
pub const TAG_NORELATEDSYM: u32 = 146;
pub const TAG_REFSEQNUM: u32 = 45;
pub const TAG_ALLOCTRANSTYPE: u32 = 71;
pub const TAG_HEARTBTINT: u32 = 108;
pub const TAG_MISCFEECURR: u32 = 138;
pub const TAG_CLORDID: u32 = 11;
pub const TAG_SYMBOLSFX: u32 = 65;
pub const TAG_LASTSHARES: u32 = 32;
pub const TAG_URLLINK: u32 = 149;
pub const TAG_OFFERFORWARDPOINTS: u32 = 191;
pub const TAG_SETTLCURRAMT: u32 = 119;
pub const TAG_ACCRUEDINTERESTAMT: u32 = 159;
pub const TAG_RAWDATALENGTH: u32 = 95;
pub const TAG_ONBEHALFOFLOCATIONID: u32 = 144;
pub const TAG_SETTLDEPOSITORYCODE: u32 = 173;
pub const TAG_SECURITYSETTLAGENTCODE: u32 = 177;
pub const TAG_CASHSETTLAGENTNAME: u32 = 182;
pub const TAG_AVGPX: u32 = 6;
pub const TAG_FUTSETTDATE: u32 = 64;
pub const TAG_SETTLINSTMODE: u32 = 160;
pub const TAG_COMMTYPE: u32 = 13;
pub const TAG_NETMONEY: u32 = 118;
pub const TAG_STANDINSTDBID: u32 = 171;
pub const TAG_RAWDATA: u32 = 96;
pub const TAG_TEXT: u32 = 58;
pub const TAG_ORDERID: u32 = 37;
pub const TAG_DELIVERTOSUBID: u32 = 129;
pub const TAG_EMAILTHREADID: u32 = 164;
pub const TAG_STANDINSTDBNAME: u32 = 170;
pub const TAG_SETTLDELIVERYTYPE: u32 = 172;
pub const TAG_CASHSETTLAGENTACCTNAME: u32 = 185;
pub const TAG_NORPTS: u32 = 82;
pub const TAG_MISCFEEAMT: u32 = 137;
pub const TAG_LISTID: u32 = 66;
pub const TAG_CASHSETTLAGENTCONTACTPHONE: u32 = 187;
pub const TAG_LASTSPOTRATE: u32 = 194;
pub const TAG_SECONDARYORDERID: u32 = 198;
pub const TAG_NUMDAYSINTEREST: u32 = 157;
pub const TAG_BROKEROFCREDIT: u32 = 92;
pub const TAG_PUTORCALL: u32 = 201;
pub const TAG_EXECREFID: u32 = 19;
pub const TAG_DELIVERTOCOMPID: u32 = 128;
pub const TAG_ALLOCSTATUS: u32 = 87;
pub const TAG_ALLOCREJCODE: u32 = 88;
pub const TAG_ORDERQTY2: u32 = 192;
pub const TAG_SENDINGTIME: u32 = 52;
pub const TAG_IOISHARES: u32 = 27;

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    /// Advertisement - '7'
    Advertisement,
    /// AllocationInstructionAck - 'P'
    AllocationInstructionAck,
    /// DontKnowTrade - 'Q'
    DontKnowTrade,
    /// Quote - 'S'
    Quote,
    /// SettlementInstructions - 'T'
    SettlementInstructions,
    /// Email - 'C'
    Email,
    /// Heartbeat - '0'
    Heartbeat,
    /// Reject - '3'
    Reject,
    /// OrderCancelReject - '9'
    OrderCancelReject,
    /// ResendRequest - '2'
    ResendRequest,
    /// OrderCancelReplaceRequest - 'G'
    OrderCancelReplaceRequest,
    /// ListCancelRequest - 'K'
    ListCancelRequest,
    /// NewOrderList - 'E'
    NewOrderList,
    /// QuoteRequest - 'R'
    QuoteRequest,
    /// Allocation - 'J'
    Allocation,
    /// ListStatus - 'N'
    ListStatus,
    /// ExecutionReport - '8'
    ExecutionReport,
    /// OrderStatusRequest - 'H'
    OrderStatusRequest,
    /// IOI - '6'
    IOI,
    /// NewOrderSingle - 'D'
    NewOrderSingle,
    /// TestRequest - '1'
    TestRequest,
    /// SequenceReset - '4'
    SequenceReset,
    /// Logout - '5'
    Logout,
    /// ListExecute - 'L'
    ListExecute,
    /// News - 'B'
    News,
    /// ListStatusRequest - 'M'
    ListStatusRequest,
    /// Logon - 'A'
    Logon,
    /// OrderCancelRequest - 'F'
    OrderCancelRequest,
    /// Unknown message type
    Unknown(String),
}

impl From<String> for MessageType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "7" => MessageType::Advertisement,
            "P" => MessageType::AllocationInstructionAck,
            "Q" => MessageType::DontKnowTrade,
            "S" => MessageType::Quote,
            "T" => MessageType::SettlementInstructions,
            "C" => MessageType::Email,
            "0" => MessageType::Heartbeat,
            "3" => MessageType::Reject,
            "9" => MessageType::OrderCancelReject,
            "2" => MessageType::ResendRequest,
            "G" => MessageType::OrderCancelReplaceRequest,
            "K" => MessageType::ListCancelRequest,
            "E" => MessageType::NewOrderList,
            "R" => MessageType::QuoteRequest,
            "J" => MessageType::Allocation,
            "N" => MessageType::ListStatus,
            "8" => MessageType::ExecutionReport,
            "H" => MessageType::OrderStatusRequest,
            "6" => MessageType::IOI,
            "D" => MessageType::NewOrderSingle,
            "1" => MessageType::TestRequest,
            "4" => MessageType::SequenceReset,
            "5" => MessageType::Logout,
            "L" => MessageType::ListExecute,
            "B" => MessageType::News,
            "M" => MessageType::ListStatusRequest,
            "A" => MessageType::Logon,
            "F" => MessageType::OrderCancelRequest,
            _ => MessageType::Unknown(s),
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Advertisement => write!(f, "7"),
            MessageType::AllocationInstructionAck => write!(f, "P"),
            MessageType::DontKnowTrade => write!(f, "Q"),
            MessageType::Quote => write!(f, "S"),
            MessageType::SettlementInstructions => write!(f, "T"),
            MessageType::Email => write!(f, "C"),
            MessageType::Heartbeat => write!(f, "0"),
            MessageType::Reject => write!(f, "3"),
            MessageType::OrderCancelReject => write!(f, "9"),
            MessageType::ResendRequest => write!(f, "2"),
            MessageType::OrderCancelReplaceRequest => write!(f, "G"),
            MessageType::ListCancelRequest => write!(f, "K"),
            MessageType::NewOrderList => write!(f, "E"),
            MessageType::QuoteRequest => write!(f, "R"),
            MessageType::Allocation => write!(f, "J"),
            MessageType::ListStatus => write!(f, "N"),
            MessageType::ExecutionReport => write!(f, "8"),
            MessageType::OrderStatusRequest => write!(f, "H"),
            MessageType::IOI => write!(f, "6"),
            MessageType::NewOrderSingle => write!(f, "D"),
            MessageType::TestRequest => write!(f, "1"),
            MessageType::SequenceReset => write!(f, "4"),
            MessageType::Logout => write!(f, "5"),
            MessageType::ListExecute => write!(f, "L"),
            MessageType::News => write!(f, "B"),
            MessageType::ListStatusRequest => write!(f, "M"),
            MessageType::Logon => write!(f, "A"),
            MessageType::OrderCancelRequest => write!(f, "F"),
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
pub struct Advertisement {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub adv_id: String,
    pub adv_trans_type: AdvTransTypeEnum,
    pub adv_ref_id: Option<String>,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub adv_side: AdvSideEnum,
    pub shares: i64,
    pub price: Option<f64>,
    pub currency: Option<String>,
    pub trade_date: Option<String>,
    pub transact_time: Option<String>,
    pub text: Option<String>,
    pub urllink: Option<String>,
    pub last_mkt: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<Advertisement> for FixMessage {
    fn from(msg: Advertisement) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_ADVID, TagValue::AdvId(msg.adv_id));
        fields.insert(TAG_ADVTRANSTYPE, TagValue::AdvTransType(msg.adv_trans_type));
        if let Some(value) = msg.adv_ref_id {
            fields.insert(TAG_ADVREFID, TagValue::AdvRefID(value));
        }
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        fields.insert(TAG_ADVSIDE, TagValue::AdvSide(msg.adv_side));
        fields.insert(TAG_SHARES, TagValue::Shares(msg.shares));
        if let Some(value) = msg.price {
            fields.insert(TAG_PRICE, TagValue::Price(value));
        }
        if let Some(value) = msg.currency {
            fields.insert(TAG_CURRENCY, TagValue::Currency(value));
        }
        if let Some(value) = msg.trade_date {
            fields.insert(TAG_TRADEDATE, TagValue::TradeDate(value));
        }
        if let Some(value) = msg.transact_time {
            fields.insert(TAG_TRANSACTTIME, TagValue::TransactTime(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.urllink {
            fields.insert(TAG_URLLINK, TagValue::URLLink(value));
        }
        if let Some(value) = msg.last_mkt {
            fields.insert(TAG_LASTMKT, TagValue::LastMkt(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::Advertisement,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AllocationInstructionAck {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub client_id: Option<String>,
    pub exec_broker: Option<String>,
    pub alloc_id: String,
    pub trade_date: String,
    pub transact_time: Option<String>,
    pub alloc_status: AllocStatusEnum,
    pub alloc_rej_code: Option<AllocRejCodeEnum>,
    pub text: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<AllocationInstructionAck> for FixMessage {
    fn from(msg: AllocationInstructionAck) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        if let Some(value) = msg.client_id {
            fields.insert(TAG_CLIENTID, TagValue::ClientID(value));
        }
        if let Some(value) = msg.exec_broker {
            fields.insert(TAG_EXECBROKER, TagValue::ExecBroker(value));
        }
        fields.insert(TAG_ALLOCID, TagValue::AllocID(msg.alloc_id));
        fields.insert(TAG_TRADEDATE, TagValue::TradeDate(msg.trade_date));
        if let Some(value) = msg.transact_time {
            fields.insert(TAG_TRANSACTTIME, TagValue::TransactTime(value));
        }
        fields.insert(TAG_ALLOCSTATUS, TagValue::AllocStatus(msg.alloc_status));
        if let Some(value) = msg.alloc_rej_code {
            fields.insert(TAG_ALLOCREJCODE, TagValue::AllocRejCode(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::AllocationInstructionAck,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DontKnowTrade {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub order_id: Option<String>,
    pub exec_id: Option<String>,
    pub dkreason: DKReasonEnum,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub side: SideEnum,
    pub order_qty: Option<i64>,
    pub cash_order_qty: Option<f64>,
    pub last_shares: Option<i64>,
    pub last_px: Option<f64>,
    pub text: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<DontKnowTrade> for FixMessage {
    fn from(msg: DontKnowTrade) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        if let Some(value) = msg.order_id {
            fields.insert(TAG_ORDERID, TagValue::OrderID(value));
        }
        if let Some(value) = msg.exec_id {
            fields.insert(TAG_EXECID, TagValue::ExecID(value));
        }
        fields.insert(TAG_DKREASON, TagValue::DKReason(msg.dkreason));
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        fields.insert(TAG_SIDE, TagValue::Side(msg.side));
        if let Some(value) = msg.order_qty {
            fields.insert(TAG_ORDERQTY, TagValue::OrderQty(value));
        }
        if let Some(value) = msg.cash_order_qty {
            fields.insert(TAG_CASHORDERQTY, TagValue::CashOrderQty(value));
        }
        if let Some(value) = msg.last_shares {
            fields.insert(TAG_LASTSHARES, TagValue::LastShares(value));
        }
        if let Some(value) = msg.last_px {
            fields.insert(TAG_LASTPX, TagValue::LastPx(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::DontKnowTrade,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Quote {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub quote_req_id: Option<String>,
    pub quote_id: String,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub bid_px: Option<f64>,
    pub offer_px: Option<f64>,
    pub bid_size: Option<i64>,
    pub offer_size: Option<i64>,
    pub valid_until_time: Option<String>,
    pub bid_spot_rate: Option<f64>,
    pub offer_spot_rate: Option<f64>,
    pub bid_forward_points: Option<f64>,
    pub offer_forward_points: Option<f64>,
    pub transact_time: Option<String>,
    pub fut_sett_date: Option<String>,
    pub ord_type: Option<OrdTypeEnum>,
    pub fut_sett_date2: Option<String>,
    pub order_qty2: Option<f64>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<Quote> for FixMessage {
    fn from(msg: Quote) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        if let Some(value) = msg.quote_req_id {
            fields.insert(TAG_QUOTEREQID, TagValue::QuoteReqID(value));
        }
        fields.insert(TAG_QUOTEID, TagValue::QuoteID(msg.quote_id));
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        if let Some(value) = msg.bid_px {
            fields.insert(TAG_BIDPX, TagValue::BidPx(value));
        }
        if let Some(value) = msg.offer_px {
            fields.insert(TAG_OFFERPX, TagValue::OfferPx(value));
        }
        if let Some(value) = msg.bid_size {
            fields.insert(TAG_BIDSIZE, TagValue::BidSize(value));
        }
        if let Some(value) = msg.offer_size {
            fields.insert(TAG_OFFERSIZE, TagValue::OfferSize(value));
        }
        if let Some(value) = msg.valid_until_time {
            fields.insert(TAG_VALIDUNTILTIME, TagValue::ValidUntilTime(value));
        }
        if let Some(value) = msg.bid_spot_rate {
            fields.insert(TAG_BIDSPOTRATE, TagValue::BidSpotRate(value));
        }
        if let Some(value) = msg.offer_spot_rate {
            fields.insert(TAG_OFFERSPOTRATE, TagValue::OfferSpotRate(value));
        }
        if let Some(value) = msg.bid_forward_points {
            fields.insert(TAG_BIDFORWARDPOINTS, TagValue::BidForwardPoints(value));
        }
        if let Some(value) = msg.offer_forward_points {
            fields.insert(TAG_OFFERFORWARDPOINTS, TagValue::OfferForwardPoints(value));
        }
        if let Some(value) = msg.transact_time {
            fields.insert(TAG_TRANSACTTIME, TagValue::TransactTime(value));
        }
        if let Some(value) = msg.fut_sett_date {
            fields.insert(TAG_FUTSETTDATE, TagValue::FutSettDate(value));
        }
        if let Some(value) = msg.ord_type {
            fields.insert(TAG_ORDTYPE, TagValue::OrdType(value));
        }
        if let Some(value) = msg.fut_sett_date2 {
            fields.insert(TAG_FUTSETTDATE2, TagValue::FutSettDate2(value));
        }
        if let Some(value) = msg.order_qty2 {
            fields.insert(TAG_ORDERQTY2, TagValue::OrderQty2(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::Quote,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettlementInstructions {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub settl_inst_id: String,
    pub settl_inst_trans_type: SettlInstTransTypeEnum,
    pub settl_inst_mode: SettlInstModeEnum,
    pub settl_inst_source: SettlInstSourceEnum,
    pub alloc_account: String,
    pub settl_location: Option<SettlLocationEnum>,
    pub trade_date: Option<String>,
    pub alloc_id: Option<String>,
    pub last_mkt: Option<String>,
    pub side: Option<SideEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub effective_time: Option<String>,
    pub transact_time: String,
    pub client_id: Option<String>,
    pub exec_broker: Option<String>,
    pub stand_inst_db_type: Option<StandInstDbTypeEnum>,
    pub stand_inst_db_name: Option<String>,
    pub stand_inst_db_id: Option<String>,
    pub settl_delivery_type: Option<i64>,
    pub settl_depository_code: Option<String>,
    pub settl_brkr_code: Option<String>,
    pub settl_inst_code: Option<String>,
    pub security_settl_agent_name: Option<String>,
    pub security_settl_agent_code: Option<String>,
    pub security_settl_agent_acct_num: Option<String>,
    pub security_settl_agent_acct_name: Option<String>,
    pub security_settl_agent_contact_name: Option<String>,
    pub security_settl_agent_contact_phone: Option<String>,
    pub cash_settl_agent_name: Option<String>,
    pub cash_settl_agent_code: Option<String>,
    pub cash_settl_agent_acct_num: Option<String>,
    pub cash_settl_agent_acct_name: Option<String>,
    pub cash_settl_agent_contact_name: Option<String>,
    pub cash_settl_agent_contact_phone: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<SettlementInstructions> for FixMessage {
    fn from(msg: SettlementInstructions) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_SETTLINSTID, TagValue::SettlInstID(msg.settl_inst_id));
        fields.insert(TAG_SETTLINSTTRANSTYPE, TagValue::SettlInstTransType(msg.settl_inst_trans_type));
        fields.insert(TAG_SETTLINSTMODE, TagValue::SettlInstMode(msg.settl_inst_mode));
        fields.insert(TAG_SETTLINSTSOURCE, TagValue::SettlInstSource(msg.settl_inst_source));
        fields.insert(TAG_ALLOCACCOUNT, TagValue::AllocAccount(msg.alloc_account));
        if let Some(value) = msg.settl_location {
            fields.insert(TAG_SETTLLOCATION, TagValue::SettlLocation(value));
        }
        if let Some(value) = msg.trade_date {
            fields.insert(TAG_TRADEDATE, TagValue::TradeDate(value));
        }
        if let Some(value) = msg.alloc_id {
            fields.insert(TAG_ALLOCID, TagValue::AllocID(value));
        }
        if let Some(value) = msg.last_mkt {
            fields.insert(TAG_LASTMKT, TagValue::LastMkt(value));
        }
        if let Some(value) = msg.side {
            fields.insert(TAG_SIDE, TagValue::Side(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.effective_time {
            fields.insert(TAG_EFFECTIVETIME, TagValue::EffectiveTime(value));
        }
        fields.insert(TAG_TRANSACTTIME, TagValue::TransactTime(msg.transact_time));
        if let Some(value) = msg.client_id {
            fields.insert(TAG_CLIENTID, TagValue::ClientID(value));
        }
        if let Some(value) = msg.exec_broker {
            fields.insert(TAG_EXECBROKER, TagValue::ExecBroker(value));
        }
        if let Some(value) = msg.stand_inst_db_type {
            fields.insert(TAG_STANDINSTDBTYPE, TagValue::StandInstDbType(value));
        }
        if let Some(value) = msg.stand_inst_db_name {
            fields.insert(TAG_STANDINSTDBNAME, TagValue::StandInstDbName(value));
        }
        if let Some(value) = msg.stand_inst_db_id {
            fields.insert(TAG_STANDINSTDBID, TagValue::StandInstDbID(value));
        }
        if let Some(value) = msg.settl_delivery_type {
            fields.insert(TAG_SETTLDELIVERYTYPE, TagValue::SettlDeliveryType(value));
        }
        if let Some(value) = msg.settl_depository_code {
            fields.insert(TAG_SETTLDEPOSITORYCODE, TagValue::SettlDepositoryCode(value));
        }
        if let Some(value) = msg.settl_brkr_code {
            fields.insert(TAG_SETTLBRKRCODE, TagValue::SettlBrkrCode(value));
        }
        if let Some(value) = msg.settl_inst_code {
            fields.insert(TAG_SETTLINSTCODE, TagValue::SettlInstCode(value));
        }
        if let Some(value) = msg.security_settl_agent_name {
            fields.insert(TAG_SECURITYSETTLAGENTNAME, TagValue::SecuritySettlAgentName(value));
        }
        if let Some(value) = msg.security_settl_agent_code {
            fields.insert(TAG_SECURITYSETTLAGENTCODE, TagValue::SecuritySettlAgentCode(value));
        }
        if let Some(value) = msg.security_settl_agent_acct_num {
            fields.insert(TAG_SECURITYSETTLAGENTACCTNUM, TagValue::SecuritySettlAgentAcctNum(value));
        }
        if let Some(value) = msg.security_settl_agent_acct_name {
            fields.insert(TAG_SECURITYSETTLAGENTACCTNAME, TagValue::SecuritySettlAgentAcctName(value));
        }
        if let Some(value) = msg.security_settl_agent_contact_name {
            fields.insert(TAG_SECURITYSETTLAGENTCONTACTNAME, TagValue::SecuritySettlAgentContactName(value));
        }
        if let Some(value) = msg.security_settl_agent_contact_phone {
            fields.insert(TAG_SECURITYSETTLAGENTCONTACTPHONE, TagValue::SecuritySettlAgentContactPhone(value));
        }
        if let Some(value) = msg.cash_settl_agent_name {
            fields.insert(TAG_CASHSETTLAGENTNAME, TagValue::CashSettlAgentName(value));
        }
        if let Some(value) = msg.cash_settl_agent_code {
            fields.insert(TAG_CASHSETTLAGENTCODE, TagValue::CashSettlAgentCode(value));
        }
        if let Some(value) = msg.cash_settl_agent_acct_num {
            fields.insert(TAG_CASHSETTLAGENTACCTNUM, TagValue::CashSettlAgentAcctNum(value));
        }
        if let Some(value) = msg.cash_settl_agent_acct_name {
            fields.insert(TAG_CASHSETTLAGENTACCTNAME, TagValue::CashSettlAgentAcctName(value));
        }
        if let Some(value) = msg.cash_settl_agent_contact_name {
            fields.insert(TAG_CASHSETTLAGENTCONTACTNAME, TagValue::CashSettlAgentContactName(value));
        }
        if let Some(value) = msg.cash_settl_agent_contact_phone {
            fields.insert(TAG_CASHSETTLAGENTCONTACTPHONE, TagValue::CashSettlAgentContactPhone(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::SettlementInstructions,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Email {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub email_thread_id: String,
    pub email_type: EmailTypeEnum,
    pub orig_time: Option<String>,
    pub subject: String,
    pub order_id: Option<String>,
    pub cl_ord_id: Option<String>,
    pub raw_data_length: Option<i64>,
    pub raw_data: Option<Vec<u8>>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<Email> for FixMessage {
    fn from(msg: Email) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_EMAILTHREADID, TagValue::EmailThreadID(msg.email_thread_id));
        fields.insert(TAG_EMAILTYPE, TagValue::EmailType(msg.email_type));
        if let Some(value) = msg.orig_time {
            fields.insert(TAG_ORIGTIME, TagValue::OrigTime(value));
        }
        fields.insert(TAG_SUBJECT, TagValue::Subject(msg.subject));
        if let Some(value) = msg.order_id {
            fields.insert(TAG_ORDERID, TagValue::OrderID(value));
        }
        if let Some(value) = msg.cl_ord_id {
            fields.insert(TAG_CLORDID, TagValue::ClOrdID(value));
        }
        if let Some(value) = msg.raw_data_length {
            fields.insert(TAG_RAWDATALENGTH, TagValue::RawDataLength(value));
        }
        if let Some(value) = msg.raw_data {
            fields.insert(TAG_RAWDATA, TagValue::RawData(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::Email,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Heartbeat {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
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
pub struct Reject {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub ref_seq_num: i64,
    pub text: Option<String>,
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
        fields.insert(TAG_REFSEQNUM, TagValue::RefSeqNum(msg.ref_seq_num));
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
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
pub struct OrderCancelReject {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub order_id: String,
    pub secondary_order_id: Option<String>,
    pub cl_ord_id: String,
    pub orig_cl_ord_id: String,
    pub ord_status: OrdStatusEnum,
    pub client_id: Option<String>,
    pub exec_broker: Option<String>,
    pub list_id: Option<String>,
    pub cxl_rej_reason: Option<CxlRejReasonEnum>,
    pub text: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<OrderCancelReject> for FixMessage {
    fn from(msg: OrderCancelReject) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_ORDERID, TagValue::OrderID(msg.order_id));
        if let Some(value) = msg.secondary_order_id {
            fields.insert(TAG_SECONDARYORDERID, TagValue::SecondaryOrderID(value));
        }
        fields.insert(TAG_CLORDID, TagValue::ClOrdID(msg.cl_ord_id));
        fields.insert(TAG_ORIGCLORDID, TagValue::OrigClOrdID(msg.orig_cl_ord_id));
        fields.insert(TAG_ORDSTATUS, TagValue::OrdStatus(msg.ord_status));
        if let Some(value) = msg.client_id {
            fields.insert(TAG_CLIENTID, TagValue::ClientID(value));
        }
        if let Some(value) = msg.exec_broker {
            fields.insert(TAG_EXECBROKER, TagValue::ExecBroker(value));
        }
        if let Some(value) = msg.list_id {
            fields.insert(TAG_LISTID, TagValue::ListID(value));
        }
        if let Some(value) = msg.cxl_rej_reason {
            fields.insert(TAG_CXLREJREASON, TagValue::CxlRejReason(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::OrderCancelReject,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResendRequest {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
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
pub struct OrderCancelReplaceRequest {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub order_id: Option<String>,
    pub client_id: Option<String>,
    pub exec_broker: Option<String>,
    pub orig_cl_ord_id: String,
    pub cl_ord_id: String,
    pub list_id: Option<String>,
    pub account: Option<String>,
    pub settlmnt_typ: Option<SettlmntTypEnum>,
    pub fut_sett_date: Option<String>,
    pub handl_inst: HandlInstEnum,
    pub exec_inst: Option<ExecInstEnum>,
    pub min_qty: Option<i64>,
    pub max_floor: Option<i64>,
    pub ex_destination: Option<String>,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub side: SideEnum,
    pub order_qty: Option<i64>,
    pub cash_order_qty: Option<f64>,
    pub ord_type: OrdTypeEnum,
    pub price: Option<f64>,
    pub stop_px: Option<f64>,
    pub peg_difference: Option<f64>,
    pub currency: Option<String>,
    pub time_in_force: Option<TimeInForceEnum>,
    pub expire_time: Option<String>,
    pub commission: Option<f64>,
    pub comm_type: Option<CommTypeEnum>,
    pub rule80a: Option<Rule80AEnum>,
    pub forex_req: Option<ForexReqEnum>,
    pub settl_currency: Option<String>,
    pub text: Option<String>,
    pub fut_sett_date2: Option<String>,
    pub order_qty2: Option<f64>,
    pub open_close: Option<OpenCloseEnum>,
    pub covered_or_uncovered: Option<CoveredOrUncoveredEnum>,
    pub customer_or_firm: Option<CustomerOrFirmEnum>,
    pub max_show: Option<i64>,
    pub locate_reqd: Option<LocateReqdEnum>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<OrderCancelReplaceRequest> for FixMessage {
    fn from(msg: OrderCancelReplaceRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        if let Some(value) = msg.order_id {
            fields.insert(TAG_ORDERID, TagValue::OrderID(value));
        }
        if let Some(value) = msg.client_id {
            fields.insert(TAG_CLIENTID, TagValue::ClientID(value));
        }
        if let Some(value) = msg.exec_broker {
            fields.insert(TAG_EXECBROKER, TagValue::ExecBroker(value));
        }
        fields.insert(TAG_ORIGCLORDID, TagValue::OrigClOrdID(msg.orig_cl_ord_id));
        fields.insert(TAG_CLORDID, TagValue::ClOrdID(msg.cl_ord_id));
        if let Some(value) = msg.list_id {
            fields.insert(TAG_LISTID, TagValue::ListID(value));
        }
        if let Some(value) = msg.account {
            fields.insert(TAG_ACCOUNT, TagValue::Account(value));
        }
        if let Some(value) = msg.settlmnt_typ {
            fields.insert(TAG_SETTLMNTTYP, TagValue::SettlmntTyp(value));
        }
        if let Some(value) = msg.fut_sett_date {
            fields.insert(TAG_FUTSETTDATE, TagValue::FutSettDate(value));
        }
        fields.insert(TAG_HANDLINST, TagValue::HandlInst(msg.handl_inst));
        if let Some(value) = msg.exec_inst {
            fields.insert(TAG_EXECINST, TagValue::ExecInst(value));
        }
        if let Some(value) = msg.min_qty {
            fields.insert(TAG_MINQTY, TagValue::MinQty(value));
        }
        if let Some(value) = msg.max_floor {
            fields.insert(TAG_MAXFLOOR, TagValue::MaxFloor(value));
        }
        if let Some(value) = msg.ex_destination {
            fields.insert(TAG_EXDESTINATION, TagValue::ExDestination(value));
        }
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        fields.insert(TAG_SIDE, TagValue::Side(msg.side));
        if let Some(value) = msg.order_qty {
            fields.insert(TAG_ORDERQTY, TagValue::OrderQty(value));
        }
        if let Some(value) = msg.cash_order_qty {
            fields.insert(TAG_CASHORDERQTY, TagValue::CashOrderQty(value));
        }
        fields.insert(TAG_ORDTYPE, TagValue::OrdType(msg.ord_type));
        if let Some(value) = msg.price {
            fields.insert(TAG_PRICE, TagValue::Price(value));
        }
        if let Some(value) = msg.stop_px {
            fields.insert(TAG_STOPPX, TagValue::StopPx(value));
        }
        if let Some(value) = msg.peg_difference {
            fields.insert(TAG_PEGDIFFERENCE, TagValue::PegDifference(value));
        }
        if let Some(value) = msg.currency {
            fields.insert(TAG_CURRENCY, TagValue::Currency(value));
        }
        if let Some(value) = msg.time_in_force {
            fields.insert(TAG_TIMEINFORCE, TagValue::TimeInForce(value));
        }
        if let Some(value) = msg.expire_time {
            fields.insert(TAG_EXPIRETIME, TagValue::ExpireTime(value));
        }
        if let Some(value) = msg.commission {
            fields.insert(TAG_COMMISSION, TagValue::Commission(value));
        }
        if let Some(value) = msg.comm_type {
            fields.insert(TAG_COMMTYPE, TagValue::CommType(value));
        }
        if let Some(value) = msg.rule80a {
            fields.insert(TAG_RULE80A, TagValue::Rule80A(value));
        }
        if let Some(value) = msg.forex_req {
            fields.insert(TAG_FOREXREQ, TagValue::ForexReq(value));
        }
        if let Some(value) = msg.settl_currency {
            fields.insert(TAG_SETTLCURRENCY, TagValue::SettlCurrency(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.fut_sett_date2 {
            fields.insert(TAG_FUTSETTDATE2, TagValue::FutSettDate2(value));
        }
        if let Some(value) = msg.order_qty2 {
            fields.insert(TAG_ORDERQTY2, TagValue::OrderQty2(value));
        }
        if let Some(value) = msg.open_close {
            fields.insert(TAG_OPENCLOSE, TagValue::OpenClose(value));
        }
        if let Some(value) = msg.covered_or_uncovered {
            fields.insert(TAG_COVEREDORUNCOVERED, TagValue::CoveredOrUncovered(value));
        }
        if let Some(value) = msg.customer_or_firm {
            fields.insert(TAG_CUSTOMERORFIRM, TagValue::CustomerOrFirm(value));
        }
        if let Some(value) = msg.max_show {
            fields.insert(TAG_MAXSHOW, TagValue::MaxShow(value));
        }
        if let Some(value) = msg.locate_reqd {
            fields.insert(TAG_LOCATEREQD, TagValue::LocateReqd(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::OrderCancelReplaceRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListCancelRequest {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub list_id: String,
    pub wave_no: Option<String>,
    pub text: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<ListCancelRequest> for FixMessage {
    fn from(msg: ListCancelRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_LISTID, TagValue::ListID(msg.list_id));
        if let Some(value) = msg.wave_no {
            fields.insert(TAG_WAVENO, TagValue::WaveNo(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::ListCancelRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewOrderList {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub list_id: String,
    pub wave_no: Option<String>,
    pub list_seq_no: i64,
    pub list_no_ords: i64,
    pub list_exec_inst: Option<String>,
    pub cl_ord_id: String,
    pub client_id: Option<String>,
    pub exec_broker: Option<String>,
    pub account: Option<String>,
    pub settlmnt_typ: Option<SettlmntTypEnum>,
    pub fut_sett_date: Option<String>,
    pub handl_inst: HandlInstEnum,
    pub exec_inst: Option<ExecInstEnum>,
    pub min_qty: Option<i64>,
    pub max_floor: Option<i64>,
    pub ex_destination: Option<String>,
    pub process_code: Option<ProcessCodeEnum>,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub prev_close_px: Option<f64>,
    pub side: SideEnum,
    pub locate_reqd: Option<LocateReqdEnum>,
    pub order_qty: i64,
    pub ord_type: OrdTypeEnum,
    pub price: Option<f64>,
    pub stop_px: Option<f64>,
    pub peg_difference: Option<f64>,
    pub currency: Option<String>,
    pub time_in_force: Option<TimeInForceEnum>,
    pub expire_time: Option<String>,
    pub commission: Option<f64>,
    pub comm_type: Option<CommTypeEnum>,
    pub rule80a: Option<Rule80AEnum>,
    pub forex_req: Option<ForexReqEnum>,
    pub settl_currency: Option<String>,
    pub text: Option<String>,
    pub fut_sett_date2: Option<String>,
    pub order_qty2: Option<f64>,
    pub open_close: Option<OpenCloseEnum>,
    pub covered_or_uncovered: Option<CoveredOrUncoveredEnum>,
    pub customer_or_firm: Option<CustomerOrFirmEnum>,
    pub max_show: Option<i64>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<NewOrderList> for FixMessage {
    fn from(msg: NewOrderList) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_LISTID, TagValue::ListID(msg.list_id));
        if let Some(value) = msg.wave_no {
            fields.insert(TAG_WAVENO, TagValue::WaveNo(value));
        }
        fields.insert(TAG_LISTSEQNO, TagValue::ListSeqNo(msg.list_seq_no));
        fields.insert(TAG_LISTNOORDS, TagValue::ListNoOrds(msg.list_no_ords));
        if let Some(value) = msg.list_exec_inst {
            fields.insert(TAG_LISTEXECINST, TagValue::ListExecInst(value));
        }
        fields.insert(TAG_CLORDID, TagValue::ClOrdID(msg.cl_ord_id));
        if let Some(value) = msg.client_id {
            fields.insert(TAG_CLIENTID, TagValue::ClientID(value));
        }
        if let Some(value) = msg.exec_broker {
            fields.insert(TAG_EXECBROKER, TagValue::ExecBroker(value));
        }
        if let Some(value) = msg.account {
            fields.insert(TAG_ACCOUNT, TagValue::Account(value));
        }
        if let Some(value) = msg.settlmnt_typ {
            fields.insert(TAG_SETTLMNTTYP, TagValue::SettlmntTyp(value));
        }
        if let Some(value) = msg.fut_sett_date {
            fields.insert(TAG_FUTSETTDATE, TagValue::FutSettDate(value));
        }
        fields.insert(TAG_HANDLINST, TagValue::HandlInst(msg.handl_inst));
        if let Some(value) = msg.exec_inst {
            fields.insert(TAG_EXECINST, TagValue::ExecInst(value));
        }
        if let Some(value) = msg.min_qty {
            fields.insert(TAG_MINQTY, TagValue::MinQty(value));
        }
        if let Some(value) = msg.max_floor {
            fields.insert(TAG_MAXFLOOR, TagValue::MaxFloor(value));
        }
        if let Some(value) = msg.ex_destination {
            fields.insert(TAG_EXDESTINATION, TagValue::ExDestination(value));
        }
        if let Some(value) = msg.process_code {
            fields.insert(TAG_PROCESSCODE, TagValue::ProcessCode(value));
        }
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        if let Some(value) = msg.prev_close_px {
            fields.insert(TAG_PREVCLOSEPX, TagValue::PrevClosePx(value));
        }
        fields.insert(TAG_SIDE, TagValue::Side(msg.side));
        if let Some(value) = msg.locate_reqd {
            fields.insert(TAG_LOCATEREQD, TagValue::LocateReqd(value));
        }
        fields.insert(TAG_ORDERQTY, TagValue::OrderQty(msg.order_qty));
        fields.insert(TAG_ORDTYPE, TagValue::OrdType(msg.ord_type));
        if let Some(value) = msg.price {
            fields.insert(TAG_PRICE, TagValue::Price(value));
        }
        if let Some(value) = msg.stop_px {
            fields.insert(TAG_STOPPX, TagValue::StopPx(value));
        }
        if let Some(value) = msg.peg_difference {
            fields.insert(TAG_PEGDIFFERENCE, TagValue::PegDifference(value));
        }
        if let Some(value) = msg.currency {
            fields.insert(TAG_CURRENCY, TagValue::Currency(value));
        }
        if let Some(value) = msg.time_in_force {
            fields.insert(TAG_TIMEINFORCE, TagValue::TimeInForce(value));
        }
        if let Some(value) = msg.expire_time {
            fields.insert(TAG_EXPIRETIME, TagValue::ExpireTime(value));
        }
        if let Some(value) = msg.commission {
            fields.insert(TAG_COMMISSION, TagValue::Commission(value));
        }
        if let Some(value) = msg.comm_type {
            fields.insert(TAG_COMMTYPE, TagValue::CommType(value));
        }
        if let Some(value) = msg.rule80a {
            fields.insert(TAG_RULE80A, TagValue::Rule80A(value));
        }
        if let Some(value) = msg.forex_req {
            fields.insert(TAG_FOREXREQ, TagValue::ForexReq(value));
        }
        if let Some(value) = msg.settl_currency {
            fields.insert(TAG_SETTLCURRENCY, TagValue::SettlCurrency(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.fut_sett_date2 {
            fields.insert(TAG_FUTSETTDATE2, TagValue::FutSettDate2(value));
        }
        if let Some(value) = msg.order_qty2 {
            fields.insert(TAG_ORDERQTY2, TagValue::OrderQty2(value));
        }
        if let Some(value) = msg.open_close {
            fields.insert(TAG_OPENCLOSE, TagValue::OpenClose(value));
        }
        if let Some(value) = msg.covered_or_uncovered {
            fields.insert(TAG_COVEREDORUNCOVERED, TagValue::CoveredOrUncovered(value));
        }
        if let Some(value) = msg.customer_or_firm {
            fields.insert(TAG_CUSTOMERORFIRM, TagValue::CustomerOrFirm(value));
        }
        if let Some(value) = msg.max_show {
            fields.insert(TAG_MAXSHOW, TagValue::MaxShow(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::NewOrderList,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QuoteRequest {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub quote_req_id: String,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub prev_close_px: Option<f64>,
    pub side: Option<SideEnum>,
    pub order_qty: Option<i64>,
    pub fut_sett_date: Option<String>,
    pub ord_type: Option<OrdTypeEnum>,
    pub fut_sett_date2: Option<String>,
    pub order_qty2: Option<f64>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<QuoteRequest> for FixMessage {
    fn from(msg: QuoteRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_QUOTEREQID, TagValue::QuoteReqID(msg.quote_req_id));
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        if let Some(value) = msg.prev_close_px {
            fields.insert(TAG_PREVCLOSEPX, TagValue::PrevClosePx(value));
        }
        if let Some(value) = msg.side {
            fields.insert(TAG_SIDE, TagValue::Side(value));
        }
        if let Some(value) = msg.order_qty {
            fields.insert(TAG_ORDERQTY, TagValue::OrderQty(value));
        }
        if let Some(value) = msg.fut_sett_date {
            fields.insert(TAG_FUTSETTDATE, TagValue::FutSettDate(value));
        }
        if let Some(value) = msg.ord_type {
            fields.insert(TAG_ORDTYPE, TagValue::OrdType(value));
        }
        if let Some(value) = msg.fut_sett_date2 {
            fields.insert(TAG_FUTSETTDATE2, TagValue::FutSettDate2(value));
        }
        if let Some(value) = msg.order_qty2 {
            fields.insert(TAG_ORDERQTY2, TagValue::OrderQty2(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::QuoteRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Allocation {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub alloc_id: String,
    pub alloc_trans_type: AllocTransTypeEnum,
    pub ref_alloc_id: Option<String>,
    pub alloc_link_id: Option<String>,
    pub alloc_link_type: Option<AllocLinkTypeEnum>,
    pub side: SideEnum,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub shares: i64,
    pub last_mkt: Option<String>,
    pub avg_px: f64,
    pub currency: Option<String>,
    pub avg_prx_precision: Option<i64>,
    pub trade_date: String,
    pub transact_time: Option<String>,
    pub settlmnt_typ: Option<SettlmntTypEnum>,
    pub fut_sett_date: Option<String>,
    pub net_money: Option<f64>,
    pub open_close: Option<OpenCloseEnum>,
    pub text: Option<String>,
    pub num_days_interest: Option<i64>,
    pub accrued_interest_rate: Option<f64>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<Allocation> for FixMessage {
    fn from(msg: Allocation) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_ALLOCID, TagValue::AllocID(msg.alloc_id));
        fields.insert(TAG_ALLOCTRANSTYPE, TagValue::AllocTransType(msg.alloc_trans_type));
        if let Some(value) = msg.ref_alloc_id {
            fields.insert(TAG_REFALLOCID, TagValue::RefAllocID(value));
        }
        if let Some(value) = msg.alloc_link_id {
            fields.insert(TAG_ALLOCLINKID, TagValue::AllocLinkID(value));
        }
        if let Some(value) = msg.alloc_link_type {
            fields.insert(TAG_ALLOCLINKTYPE, TagValue::AllocLinkType(value));
        }
        fields.insert(TAG_SIDE, TagValue::Side(msg.side));
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        fields.insert(TAG_SHARES, TagValue::Shares(msg.shares));
        if let Some(value) = msg.last_mkt {
            fields.insert(TAG_LASTMKT, TagValue::LastMkt(value));
        }
        fields.insert(TAG_AVGPX, TagValue::AvgPx(msg.avg_px));
        if let Some(value) = msg.currency {
            fields.insert(TAG_CURRENCY, TagValue::Currency(value));
        }
        if let Some(value) = msg.avg_prx_precision {
            fields.insert(TAG_AVGPRXPRECISION, TagValue::AvgPrxPrecision(value));
        }
        fields.insert(TAG_TRADEDATE, TagValue::TradeDate(msg.trade_date));
        if let Some(value) = msg.transact_time {
            fields.insert(TAG_TRANSACTTIME, TagValue::TransactTime(value));
        }
        if let Some(value) = msg.settlmnt_typ {
            fields.insert(TAG_SETTLMNTTYP, TagValue::SettlmntTyp(value));
        }
        if let Some(value) = msg.fut_sett_date {
            fields.insert(TAG_FUTSETTDATE, TagValue::FutSettDate(value));
        }
        if let Some(value) = msg.net_money {
            fields.insert(TAG_NETMONEY, TagValue::NetMoney(value));
        }
        if let Some(value) = msg.open_close {
            fields.insert(TAG_OPENCLOSE, TagValue::OpenClose(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.num_days_interest {
            fields.insert(TAG_NUMDAYSINTEREST, TagValue::NumDaysInterest(value));
        }
        if let Some(value) = msg.accrued_interest_rate {
            fields.insert(TAG_ACCRUEDINTERESTRATE, TagValue::AccruedInterestRate(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::Allocation,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListStatus {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub list_id: String,
    pub wave_no: Option<String>,
    pub no_rpts: i64,
    pub rpt_seq: i64,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<ListStatus> for FixMessage {
    fn from(msg: ListStatus) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_LISTID, TagValue::ListID(msg.list_id));
        if let Some(value) = msg.wave_no {
            fields.insert(TAG_WAVENO, TagValue::WaveNo(value));
        }
        fields.insert(TAG_NORPTS, TagValue::NoRpts(msg.no_rpts));
        fields.insert(TAG_RPTSEQ, TagValue::RptSeq(msg.rpt_seq));
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::ListStatus,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionReport {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub order_id: String,
    pub secondary_order_id: Option<String>,
    pub cl_ord_id: Option<String>,
    pub orig_cl_ord_id: Option<String>,
    pub client_id: Option<String>,
    pub exec_broker: Option<String>,
    pub list_id: Option<String>,
    pub exec_id: String,
    pub exec_trans_type: ExecTransTypeEnum,
    pub exec_ref_id: Option<String>,
    pub exec_type: ExecTypeEnum,
    pub ord_status: OrdStatusEnum,
    pub ord_rej_reason: Option<OrdRejReasonEnum>,
    pub account: Option<String>,
    pub settlmnt_typ: Option<SettlmntTypEnum>,
    pub fut_sett_date: Option<String>,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub side: SideEnum,
    pub order_qty: i64,
    pub ord_type: Option<OrdTypeEnum>,
    pub price: Option<f64>,
    pub stop_px: Option<f64>,
    pub peg_difference: Option<f64>,
    pub currency: Option<String>,
    pub time_in_force: Option<TimeInForceEnum>,
    pub expire_time: Option<String>,
    pub exec_inst: Option<ExecInstEnum>,
    pub rule80a: Option<Rule80AEnum>,
    pub last_shares: i64,
    pub last_px: f64,
    pub last_spot_rate: Option<f64>,
    pub last_forward_points: Option<f64>,
    pub last_mkt: Option<String>,
    pub last_capacity: Option<LastCapacityEnum>,
    pub leaves_qty: i64,
    pub cum_qty: i64,
    pub avg_px: f64,
    pub trade_date: Option<String>,
    pub transact_time: Option<String>,
    pub report_to_exch: Option<ReportToExchEnum>,
    pub commission: Option<f64>,
    pub comm_type: Option<CommTypeEnum>,
    pub settl_curr_amt: Option<f64>,
    pub settl_currency: Option<String>,
    pub settl_curr_fx_rate: Option<f64>,
    pub settl_curr_fx_rate_calc: Option<String>,
    pub text: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<ExecutionReport> for FixMessage {
    fn from(msg: ExecutionReport) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_ORDERID, TagValue::OrderID(msg.order_id));
        if let Some(value) = msg.secondary_order_id {
            fields.insert(TAG_SECONDARYORDERID, TagValue::SecondaryOrderID(value));
        }
        if let Some(value) = msg.cl_ord_id {
            fields.insert(TAG_CLORDID, TagValue::ClOrdID(value));
        }
        if let Some(value) = msg.orig_cl_ord_id {
            fields.insert(TAG_ORIGCLORDID, TagValue::OrigClOrdID(value));
        }
        if let Some(value) = msg.client_id {
            fields.insert(TAG_CLIENTID, TagValue::ClientID(value));
        }
        if let Some(value) = msg.exec_broker {
            fields.insert(TAG_EXECBROKER, TagValue::ExecBroker(value));
        }
        if let Some(value) = msg.list_id {
            fields.insert(TAG_LISTID, TagValue::ListID(value));
        }
        fields.insert(TAG_EXECID, TagValue::ExecID(msg.exec_id));
        fields.insert(TAG_EXECTRANSTYPE, TagValue::ExecTransType(msg.exec_trans_type));
        if let Some(value) = msg.exec_ref_id {
            fields.insert(TAG_EXECREFID, TagValue::ExecRefID(value));
        }
        fields.insert(TAG_EXECTYPE, TagValue::ExecType(msg.exec_type));
        fields.insert(TAG_ORDSTATUS, TagValue::OrdStatus(msg.ord_status));
        if let Some(value) = msg.ord_rej_reason {
            fields.insert(TAG_ORDREJREASON, TagValue::OrdRejReason(value));
        }
        if let Some(value) = msg.account {
            fields.insert(TAG_ACCOUNT, TagValue::Account(value));
        }
        if let Some(value) = msg.settlmnt_typ {
            fields.insert(TAG_SETTLMNTTYP, TagValue::SettlmntTyp(value));
        }
        if let Some(value) = msg.fut_sett_date {
            fields.insert(TAG_FUTSETTDATE, TagValue::FutSettDate(value));
        }
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        fields.insert(TAG_SIDE, TagValue::Side(msg.side));
        fields.insert(TAG_ORDERQTY, TagValue::OrderQty(msg.order_qty));
        if let Some(value) = msg.ord_type {
            fields.insert(TAG_ORDTYPE, TagValue::OrdType(value));
        }
        if let Some(value) = msg.price {
            fields.insert(TAG_PRICE, TagValue::Price(value));
        }
        if let Some(value) = msg.stop_px {
            fields.insert(TAG_STOPPX, TagValue::StopPx(value));
        }
        if let Some(value) = msg.peg_difference {
            fields.insert(TAG_PEGDIFFERENCE, TagValue::PegDifference(value));
        }
        if let Some(value) = msg.currency {
            fields.insert(TAG_CURRENCY, TagValue::Currency(value));
        }
        if let Some(value) = msg.time_in_force {
            fields.insert(TAG_TIMEINFORCE, TagValue::TimeInForce(value));
        }
        if let Some(value) = msg.expire_time {
            fields.insert(TAG_EXPIRETIME, TagValue::ExpireTime(value));
        }
        if let Some(value) = msg.exec_inst {
            fields.insert(TAG_EXECINST, TagValue::ExecInst(value));
        }
        if let Some(value) = msg.rule80a {
            fields.insert(TAG_RULE80A, TagValue::Rule80A(value));
        }
        fields.insert(TAG_LASTSHARES, TagValue::LastShares(msg.last_shares));
        fields.insert(TAG_LASTPX, TagValue::LastPx(msg.last_px));
        if let Some(value) = msg.last_spot_rate {
            fields.insert(TAG_LASTSPOTRATE, TagValue::LastSpotRate(value));
        }
        if let Some(value) = msg.last_forward_points {
            fields.insert(TAG_LASTFORWARDPOINTS, TagValue::LastForwardPoints(value));
        }
        if let Some(value) = msg.last_mkt {
            fields.insert(TAG_LASTMKT, TagValue::LastMkt(value));
        }
        if let Some(value) = msg.last_capacity {
            fields.insert(TAG_LASTCAPACITY, TagValue::LastCapacity(value));
        }
        fields.insert(TAG_LEAVESQTY, TagValue::LeavesQty(msg.leaves_qty));
        fields.insert(TAG_CUMQTY, TagValue::CumQty(msg.cum_qty));
        fields.insert(TAG_AVGPX, TagValue::AvgPx(msg.avg_px));
        if let Some(value) = msg.trade_date {
            fields.insert(TAG_TRADEDATE, TagValue::TradeDate(value));
        }
        if let Some(value) = msg.transact_time {
            fields.insert(TAG_TRANSACTTIME, TagValue::TransactTime(value));
        }
        if let Some(value) = msg.report_to_exch {
            fields.insert(TAG_REPORTTOEXCH, TagValue::ReportToExch(value));
        }
        if let Some(value) = msg.commission {
            fields.insert(TAG_COMMISSION, TagValue::Commission(value));
        }
        if let Some(value) = msg.comm_type {
            fields.insert(TAG_COMMTYPE, TagValue::CommType(value));
        }
        if let Some(value) = msg.settl_curr_amt {
            fields.insert(TAG_SETTLCURRAMT, TagValue::SettlCurrAmt(value));
        }
        if let Some(value) = msg.settl_currency {
            fields.insert(TAG_SETTLCURRENCY, TagValue::SettlCurrency(value));
        }
        if let Some(value) = msg.settl_curr_fx_rate {
            fields.insert(TAG_SETTLCURRFXRATE, TagValue::SettlCurrFxRate(value));
        }
        if let Some(value) = msg.settl_curr_fx_rate_calc {
            fields.insert(TAG_SETTLCURRFXRATECALC, TagValue::SettlCurrFxRateCalc(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::ExecutionReport,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderStatusRequest {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub order_id: Option<String>,
    pub cl_ord_id: String,
    pub client_id: Option<String>,
    pub exec_broker: Option<String>,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub side: SideEnum,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<OrderStatusRequest> for FixMessage {
    fn from(msg: OrderStatusRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        if let Some(value) = msg.order_id {
            fields.insert(TAG_ORDERID, TagValue::OrderID(value));
        }
        fields.insert(TAG_CLORDID, TagValue::ClOrdID(msg.cl_ord_id));
        if let Some(value) = msg.client_id {
            fields.insert(TAG_CLIENTID, TagValue::ClientID(value));
        }
        if let Some(value) = msg.exec_broker {
            fields.insert(TAG_EXECBROKER, TagValue::ExecBroker(value));
        }
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        fields.insert(TAG_SIDE, TagValue::Side(msg.side));
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::OrderStatusRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IOI {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub ioiid: String,
    pub ioitrans_type: IOITransTypeEnum,
    pub ioiref_id: Option<String>,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub side: SideEnum,
    pub ioishares: IOISharesEnum,
    pub price: Option<f64>,
    pub currency: Option<String>,
    pub valid_until_time: Option<String>,
    pub ioiqlty_ind: Option<IOIQltyIndEnum>,
    pub ioioth_svc: Option<IOIOthSvcEnum>,
    pub ioinatural_flag: Option<IOINaturalFlagEnum>,
    pub text: Option<String>,
    pub transact_time: Option<String>,
    pub urllink: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<IOI> for FixMessage {
    fn from(msg: IOI) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_IOIID, TagValue::IOIid(msg.ioiid));
        fields.insert(TAG_IOITRANSTYPE, TagValue::IOITransType(msg.ioitrans_type));
        if let Some(value) = msg.ioiref_id {
            fields.insert(TAG_IOIREFID, TagValue::IOIRefID(value));
        }
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        fields.insert(TAG_SIDE, TagValue::Side(msg.side));
        fields.insert(TAG_IOISHARES, TagValue::IOIShares(msg.ioishares));
        if let Some(value) = msg.price {
            fields.insert(TAG_PRICE, TagValue::Price(value));
        }
        if let Some(value) = msg.currency {
            fields.insert(TAG_CURRENCY, TagValue::Currency(value));
        }
        if let Some(value) = msg.valid_until_time {
            fields.insert(TAG_VALIDUNTILTIME, TagValue::ValidUntilTime(value));
        }
        if let Some(value) = msg.ioiqlty_ind {
            fields.insert(TAG_IOIQLTYIND, TagValue::IOIQltyInd(value));
        }
        if let Some(value) = msg.ioioth_svc {
            fields.insert(TAG_IOIOTHSVC, TagValue::IOIOthSvc(value));
        }
        if let Some(value) = msg.ioinatural_flag {
            fields.insert(TAG_IOINATURALFLAG, TagValue::IOINaturalFlag(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.transact_time {
            fields.insert(TAG_TRANSACTTIME, TagValue::TransactTime(value));
        }
        if let Some(value) = msg.urllink {
            fields.insert(TAG_URLLINK, TagValue::URLLink(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::IOI,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewOrderSingle {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub cl_ord_id: String,
    pub client_id: Option<String>,
    pub exec_broker: Option<String>,
    pub account: Option<String>,
    pub settlmnt_typ: Option<SettlmntTypEnum>,
    pub fut_sett_date: Option<String>,
    pub handl_inst: HandlInstEnum,
    pub exec_inst: Option<ExecInstEnum>,
    pub min_qty: Option<i64>,
    pub max_floor: Option<i64>,
    pub ex_destination: Option<String>,
    pub process_code: Option<ProcessCodeEnum>,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub prev_close_px: Option<f64>,
    pub side: SideEnum,
    pub locate_reqd: Option<LocateReqdEnum>,
    pub order_qty: Option<i64>,
    pub cash_order_qty: Option<f64>,
    pub ord_type: OrdTypeEnum,
    pub price: Option<f64>,
    pub stop_px: Option<f64>,
    pub currency: Option<String>,
    pub ioiid: Option<String>,
    pub quote_id: Option<String>,
    pub time_in_force: Option<TimeInForceEnum>,
    pub expire_time: Option<String>,
    pub commission: Option<f64>,
    pub comm_type: Option<CommTypeEnum>,
    pub rule80a: Option<Rule80AEnum>,
    pub forex_req: Option<ForexReqEnum>,
    pub settl_currency: Option<String>,
    pub text: Option<String>,
    pub fut_sett_date2: Option<String>,
    pub order_qty2: Option<f64>,
    pub open_close: Option<OpenCloseEnum>,
    pub covered_or_uncovered: Option<CoveredOrUncoveredEnum>,
    pub customer_or_firm: Option<CustomerOrFirmEnum>,
    pub max_show: Option<i64>,
    pub peg_difference: Option<f64>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<NewOrderSingle> for FixMessage {
    fn from(msg: NewOrderSingle) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_CLORDID, TagValue::ClOrdID(msg.cl_ord_id));
        if let Some(value) = msg.client_id {
            fields.insert(TAG_CLIENTID, TagValue::ClientID(value));
        }
        if let Some(value) = msg.exec_broker {
            fields.insert(TAG_EXECBROKER, TagValue::ExecBroker(value));
        }
        if let Some(value) = msg.account {
            fields.insert(TAG_ACCOUNT, TagValue::Account(value));
        }
        if let Some(value) = msg.settlmnt_typ {
            fields.insert(TAG_SETTLMNTTYP, TagValue::SettlmntTyp(value));
        }
        if let Some(value) = msg.fut_sett_date {
            fields.insert(TAG_FUTSETTDATE, TagValue::FutSettDate(value));
        }
        fields.insert(TAG_HANDLINST, TagValue::HandlInst(msg.handl_inst));
        if let Some(value) = msg.exec_inst {
            fields.insert(TAG_EXECINST, TagValue::ExecInst(value));
        }
        if let Some(value) = msg.min_qty {
            fields.insert(TAG_MINQTY, TagValue::MinQty(value));
        }
        if let Some(value) = msg.max_floor {
            fields.insert(TAG_MAXFLOOR, TagValue::MaxFloor(value));
        }
        if let Some(value) = msg.ex_destination {
            fields.insert(TAG_EXDESTINATION, TagValue::ExDestination(value));
        }
        if let Some(value) = msg.process_code {
            fields.insert(TAG_PROCESSCODE, TagValue::ProcessCode(value));
        }
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        if let Some(value) = msg.prev_close_px {
            fields.insert(TAG_PREVCLOSEPX, TagValue::PrevClosePx(value));
        }
        fields.insert(TAG_SIDE, TagValue::Side(msg.side));
        if let Some(value) = msg.locate_reqd {
            fields.insert(TAG_LOCATEREQD, TagValue::LocateReqd(value));
        }
        if let Some(value) = msg.order_qty {
            fields.insert(TAG_ORDERQTY, TagValue::OrderQty(value));
        }
        if let Some(value) = msg.cash_order_qty {
            fields.insert(TAG_CASHORDERQTY, TagValue::CashOrderQty(value));
        }
        fields.insert(TAG_ORDTYPE, TagValue::OrdType(msg.ord_type));
        if let Some(value) = msg.price {
            fields.insert(TAG_PRICE, TagValue::Price(value));
        }
        if let Some(value) = msg.stop_px {
            fields.insert(TAG_STOPPX, TagValue::StopPx(value));
        }
        if let Some(value) = msg.currency {
            fields.insert(TAG_CURRENCY, TagValue::Currency(value));
        }
        if let Some(value) = msg.ioiid {
            fields.insert(TAG_IOIID, TagValue::IOIid(value));
        }
        if let Some(value) = msg.quote_id {
            fields.insert(TAG_QUOTEID, TagValue::QuoteID(value));
        }
        if let Some(value) = msg.time_in_force {
            fields.insert(TAG_TIMEINFORCE, TagValue::TimeInForce(value));
        }
        if let Some(value) = msg.expire_time {
            fields.insert(TAG_EXPIRETIME, TagValue::ExpireTime(value));
        }
        if let Some(value) = msg.commission {
            fields.insert(TAG_COMMISSION, TagValue::Commission(value));
        }
        if let Some(value) = msg.comm_type {
            fields.insert(TAG_COMMTYPE, TagValue::CommType(value));
        }
        if let Some(value) = msg.rule80a {
            fields.insert(TAG_RULE80A, TagValue::Rule80A(value));
        }
        if let Some(value) = msg.forex_req {
            fields.insert(TAG_FOREXREQ, TagValue::ForexReq(value));
        }
        if let Some(value) = msg.settl_currency {
            fields.insert(TAG_SETTLCURRENCY, TagValue::SettlCurrency(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.fut_sett_date2 {
            fields.insert(TAG_FUTSETTDATE2, TagValue::FutSettDate2(value));
        }
        if let Some(value) = msg.order_qty2 {
            fields.insert(TAG_ORDERQTY2, TagValue::OrderQty2(value));
        }
        if let Some(value) = msg.open_close {
            fields.insert(TAG_OPENCLOSE, TagValue::OpenClose(value));
        }
        if let Some(value) = msg.covered_or_uncovered {
            fields.insert(TAG_COVEREDORUNCOVERED, TagValue::CoveredOrUncovered(value));
        }
        if let Some(value) = msg.customer_or_firm {
            fields.insert(TAG_CUSTOMERORFIRM, TagValue::CustomerOrFirm(value));
        }
        if let Some(value) = msg.max_show {
            fields.insert(TAG_MAXSHOW, TagValue::MaxShow(value));
        }
        if let Some(value) = msg.peg_difference {
            fields.insert(TAG_PEGDIFFERENCE, TagValue::PegDifference(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::NewOrderSingle,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestRequest {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
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
pub struct SequenceReset {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
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

#[derive(Debug, Clone)]
pub struct Logout {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub text: Option<String>,
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
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
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
pub struct ListExecute {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub list_id: String,
    pub wave_no: Option<String>,
    pub text: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<ListExecute> for FixMessage {
    fn from(msg: ListExecute) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_LISTID, TagValue::ListID(msg.list_id));
        if let Some(value) = msg.wave_no {
            fields.insert(TAG_WAVENO, TagValue::WaveNo(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::ListExecute,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct News {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub orig_time: Option<String>,
    pub urgency: Option<UrgencyEnum>,
    pub headline: String,
    pub urllink: Option<String>,
    pub raw_data_length: Option<i64>,
    pub raw_data: Option<Vec<u8>>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<News> for FixMessage {
    fn from(msg: News) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        if let Some(value) = msg.orig_time {
            fields.insert(TAG_ORIGTIME, TagValue::OrigTime(value));
        }
        if let Some(value) = msg.urgency {
            fields.insert(TAG_URGENCY, TagValue::Urgency(value));
        }
        fields.insert(TAG_HEADLINE, TagValue::Headline(msg.headline));
        if let Some(value) = msg.urllink {
            fields.insert(TAG_URLLINK, TagValue::URLLink(value));
        }
        if let Some(value) = msg.raw_data_length {
            fields.insert(TAG_RAWDATALENGTH, TagValue::RawDataLength(value));
        }
        if let Some(value) = msg.raw_data {
            fields.insert(TAG_RAWDATA, TagValue::RawData(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::News,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListStatusRequest {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub list_id: String,
    pub wave_no: Option<String>,
    pub text: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<ListStatusRequest> for FixMessage {
    fn from(msg: ListStatusRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_LISTID, TagValue::ListID(msg.list_id));
        if let Some(value) = msg.wave_no {
            fields.insert(TAG_WAVENO, TagValue::WaveNo(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::ListStatusRequest,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Logon {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub encrypt_method: EncryptMethodEnum,
    pub heart_bt_int: i64,
    pub raw_data_length: Option<i64>,
    pub raw_data: Option<Vec<u8>>,
    pub reset_seq_num_flag: Option<ResetSeqNumFlagEnum>,
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
pub struct OrderCancelRequest {
    // Header fields
    pub begin_string: String,
    pub body_length: i64,
    pub msg_type: MsgTypeEnum,
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
    pub sending_time: String,
    pub orig_sending_time: Option<String>,
    // Message fields
    pub orig_cl_ord_id: String,
    pub order_id: Option<String>,
    pub cl_ord_id: String,
    pub list_id: Option<String>,
    pub client_id: Option<String>,
    pub exec_broker: Option<String>,
    pub symbol: String,
    pub symbol_sfx: Option<String>,
    pub security_id: Option<String>,
    pub idsource: Option<IDSourceEnum>,
    pub security_type: Option<SecurityTypeEnum>,
    pub maturity_month_year: Option<String>,
    pub maturity_day: Option<u8>,
    pub put_or_call: Option<PutOrCallEnum>,
    pub strike_price: Option<f64>,
    pub opt_attribute: Option<String>,
    pub security_exchange: Option<String>,
    pub issuer: Option<String>,
    pub security_desc: Option<String>,
    pub side: SideEnum,
    pub order_qty: Option<i64>,
    pub cash_order_qty: Option<f64>,
    pub text: Option<String>,
    // Trailer fields
    pub signature_length: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub check_sum: String,
}

impl From<OrderCancelRequest> for FixMessage {
    fn from(msg: OrderCancelRequest) -> Self {
        let mut fields = std::collections::HashMap::new();
        fields.insert(TAG_BEGINSTRING, TagValue::BeginString(msg.begin_string));
        fields.insert(TAG_BODYLENGTH, TagValue::BodyLength(msg.body_length));
        fields.insert(TAG_MSGTYPE, TagValue::MsgType(msg.msg_type));
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
        fields.insert(TAG_ORIGCLORDID, TagValue::OrigClOrdID(msg.orig_cl_ord_id));
        if let Some(value) = msg.order_id {
            fields.insert(TAG_ORDERID, TagValue::OrderID(value));
        }
        fields.insert(TAG_CLORDID, TagValue::ClOrdID(msg.cl_ord_id));
        if let Some(value) = msg.list_id {
            fields.insert(TAG_LISTID, TagValue::ListID(value));
        }
        if let Some(value) = msg.client_id {
            fields.insert(TAG_CLIENTID, TagValue::ClientID(value));
        }
        if let Some(value) = msg.exec_broker {
            fields.insert(TAG_EXECBROKER, TagValue::ExecBroker(value));
        }
        fields.insert(TAG_SYMBOL, TagValue::Symbol(msg.symbol));
        if let Some(value) = msg.symbol_sfx {
            fields.insert(TAG_SYMBOLSFX, TagValue::SymbolSfx(value));
        }
        if let Some(value) = msg.security_id {
            fields.insert(TAG_SECURITYID, TagValue::SecurityID(value));
        }
        if let Some(value) = msg.idsource {
            fields.insert(TAG_IDSOURCE, TagValue::IDSource(value));
        }
        if let Some(value) = msg.security_type {
            fields.insert(TAG_SECURITYTYPE, TagValue::SecurityType(value));
        }
        if let Some(value) = msg.maturity_month_year {
            fields.insert(TAG_MATURITYMONTHYEAR, TagValue::MaturityMonthYear(value));
        }
        if let Some(value) = msg.maturity_day {
            fields.insert(TAG_MATURITYDAY, TagValue::MaturityDay(value));
        }
        if let Some(value) = msg.put_or_call {
            fields.insert(TAG_PUTORCALL, TagValue::PutOrCall(value));
        }
        if let Some(value) = msg.strike_price {
            fields.insert(TAG_STRIKEPRICE, TagValue::StrikePrice(value));
        }
        if let Some(value) = msg.opt_attribute {
            fields.insert(TAG_OPTATTRIBUTE, TagValue::OptAttribute(value));
        }
        if let Some(value) = msg.security_exchange {
            fields.insert(TAG_SECURITYEXCHANGE, TagValue::SecurityExchange(value));
        }
        if let Some(value) = msg.issuer {
            fields.insert(TAG_ISSUER, TagValue::Issuer(value));
        }
        if let Some(value) = msg.security_desc {
            fields.insert(TAG_SECURITYDESC, TagValue::SecurityDesc(value));
        }
        fields.insert(TAG_SIDE, TagValue::Side(msg.side));
        if let Some(value) = msg.order_qty {
            fields.insert(TAG_ORDERQTY, TagValue::OrderQty(value));
        }
        if let Some(value) = msg.cash_order_qty {
            fields.insert(TAG_CASHORDERQTY, TagValue::CashOrderQty(value));
        }
        if let Some(value) = msg.text {
            fields.insert(TAG_TEXT, TagValue::Text(value));
        }
        if let Some(value) = msg.signature_length {
            fields.insert(TAG_SIGNATURELENGTH, TagValue::SignatureLength(value));
        }
        if let Some(value) = msg.signature {
            fields.insert(TAG_SIGNATURE, TagValue::Signature(value));
        }
        fields.insert(TAG_CHECKSUM, TagValue::CheckSum(msg.check_sum));
        FixMessage {
            msg_type: MessageType::OrderCancelRequest,
            fields,
        }
    }
}

