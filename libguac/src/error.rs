use crate::status::GuacStatus;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct GuacError {
    status: GuacStatus,
    description: &'static str,
}

impl Display for GuacError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description, self.status)
    }
}

impl Error for GuacError {}

impl GuacError {
    pub fn read() -> Self {
        GuacError {
            status: GuacStatus::from(Self::code()),
            description: Self::message(),
        }
    }

    pub fn is_status(&self, status: GuacStatus) -> bool {
        self.status == status
    }

    fn message() -> &'static str {
        unsafe {
            let guac_error_message = libguac_sys::__guac_error_message();
            if (*guac_error_message).is_null() {
                "Unknown error"
            } else {
                std::ffi::CStr::from_ptr(*guac_error_message)
                    .to_str()
                    .unwrap_or_default()
            }
        }
    }

    fn code() -> u32 {
        unsafe {
            *libguac_sys::__guac_error()
        }
    }
}

impl Into<&'static str> for GuacError {
    fn into(self) -> &'static str {
        self.description
    }
}

impl From<&'static str> for GuacError {
    fn from(description: &'static str) -> Self {
        GuacError {
            description,
            status: GuacStatus::InternalError,
        }
    }
}
