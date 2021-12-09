use crate::{guac_parser, guac_user, GuacError};

pub enum guac_client_state {
    RUNNING,
    STOPPING,
}

impl From<std::os::raw::c_uint> for guac_client_state {
    fn from(raw: std::os::raw::c_uint) -> Self {
        match raw {
            0 => Self::RUNNING,
            _ => Self::STOPPING,
        }
    }
}

pub struct guac_client(pub(crate) *mut libguac_sys::guac_client);

impl guac_client {
    pub fn alloc() -> Option<Self> {
        unsafe {
            let client = libguac_sys::guac_client_alloc();

            if client.is_null() {
                return None
            }

            Some(guac_client(client))
        }
    }

    pub fn load_plugin(&self, protocol: &str) -> Result<(), GuacError> {
        unsafe {
            let proto = std::ffi::CString::new(protocol)
                .map_err(|_| "Invalid string")?;
            match libguac_sys::guac_client_load_plugin(self.0, proto.as_ptr()) {
                0 => Ok(()),
                _ => Err(GuacError::read()),
            }
        }
    }

    pub fn add_user(&self, user: &guac_user, parser: &guac_parser) -> Result<(), GuacError> {
        let ret = unsafe {
            libguac_sys::guac_client_add_user(
                self.0,
                user.0,
                parser.argc() as i32 - 1,
                parser.args_mut_skip(1).as_mut_ptr(),
            )
        };

        match ret {
            0 => Ok(()),
            _ => Err(GuacError::from("Failed to add user")),
        }
    }

    pub fn connection_id<'a>(&self) -> std::borrow::Cow<'a, str> {
        let raw = unsafe {
            if (*self.0).connection_id.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr((*self.0).connection_id))
            }
        };

        raw.unwrap_or_default().to_string_lossy()
    }

    pub fn args<'a>(&self) -> Vec<std::borrow::Cow<'a, str>> {
        let mut args = Vec::new();
        unsafe {
            let array_ptr: *mut *const std::os::raw::c_char = (*self.0).args;
            for offset in 0.. {
                let val = *(array_ptr.offset(offset));
                if val.is_null() {
                    break;
                }
                args.push(std::ffi::CStr::from_ptr(val).to_string_lossy());
            }
        };

        args
    }

    pub fn state(&self) -> guac_client_state {
        guac_client_state::from(unsafe {
            (*self.0).state
        })
    }
}

impl Drop for guac_client {
    fn drop(&mut self) {
        println!("guac_client freed");
        // WARNING Do not implement Drop for `guac_client` it handled libguac internally!
    }
}

unsafe impl Send for guac_client {}

unsafe impl Sync for guac_client {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_alloc() {
        let c = guac_client::alloc();
        assert!(c.is_some());
    }

    #[test]
    fn test_client_connection_id() {
        let c = guac_client::alloc();
        assert!(c.is_some());
        let c = c.unwrap();
        assert!(c.connection_id().starts_with('$'));
        assert_eq!(c.connection_id().len(), 37);
        assert_eq!(c.connection_id(), c.connection_id());
    }
}
