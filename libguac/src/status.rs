use std::os::raw::c_uint;

#[derive(Debug, PartialEq)]
pub enum GuacStatus {
    Success = 0,
    NoMemory = 1,
    Closed = 2,
    Timeout = 3,
    SeeErrno = 4,
    IoError = 5,
    InvalidArgument = 6,
    InternalError = 7,
    NoSpace = 8,
    InputTooLarge = 9,
    ResultTooLarge = 10,
    PermissionDenied = 11,
    Busy = 12,
    NotAvailable = 13,
    NotSupported = 14,
    NotImplemented = 15,
    TryAgain = 16,
    ProtocolError = 17,
    NotFound = 18,
    Cancelled = 19,
    OutOfRange = 20,
    Refused = 21,
    TooMany = 22,
    WouldBlock = 23,

    Unknown,
}

impl From<c_uint> for GuacStatus {
    fn from(code: c_uint) -> Self {
        match code {
            0 => GuacStatus::Success,
            1 => GuacStatus::NoMemory,
            2 => GuacStatus::Closed,
            3 => GuacStatus::Timeout,
            4 => GuacStatus::SeeErrno,
            5 => GuacStatus::IoError,
            6 => GuacStatus::InvalidArgument,
            7 => GuacStatus::InternalError,
            8 => GuacStatus::NoSpace,
            9 => GuacStatus::InputTooLarge,
            10 => GuacStatus::ResultTooLarge,
            11 => GuacStatus::PermissionDenied,
            12 => GuacStatus::Busy,
            13 => GuacStatus::NotAvailable,
            14 => GuacStatus::NotSupported,
            15 => GuacStatus::NotImplemented,
            16 => GuacStatus::TryAgain,
            17 => GuacStatus::ProtocolError,
            18 => GuacStatus::NotFound,
            19 => GuacStatus::Cancelled,
            20 => GuacStatus::OutOfRange,
            21 => GuacStatus::Refused,
            22 => GuacStatus::TooMany,
            23 => GuacStatus::WouldBlock,

            _ => GuacStatus::Unknown,
        }
    }
}

impl std::fmt::Display for GuacStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
