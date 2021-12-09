use crate::{guac_client, guac_parser, guac_socket, GuacError};
use crate::proto::guac_version;

pub struct guac_user_info(*mut libguac_sys::guac_user_info);

impl guac_user_info {
    pub fn set_width(&self, width: i32) {
        unsafe {
            (*self.0).optimal_width = width;
        }
    }

    pub fn set_height(&self, height: i32) {
        unsafe {
            (*self.0).optimal_height = height;
        }
    }

    pub fn set_optimal_resolution(&self, dpi: i32) {
        unsafe {
            (*self.0).optimal_resolution = dpi;
        }
    }
}

pub struct guac_user(pub(crate) *mut libguac_sys::guac_user);

impl guac_user {
    pub fn alloc() -> Option<Self> {
        unsafe {
            let user = libguac_sys::guac_user_alloc();

            if user.is_null() {
                return None
            }

            Some(guac_user(user))
        }
    }

    pub fn info(&self) -> guac_user_info {
        guac_user_info(unsafe { &mut (*self.0).info })
    }

    pub fn set_socket(&self, socket: &guac_socket) {
        unsafe {
            (*self.0).socket = socket.0
        }
    }

    pub fn set_client(&self, client: &guac_client) {
        unsafe {
            (*self.0).client = client.0
        }
    }

    pub fn set_protocol_version(&self, version: guac_version) {
        unsafe {
            (*self.0).info.protocol_version = version.into()
        }
    }

    pub fn get_protocol_version(&self) -> guac_version {
        unsafe {
            guac_version::from((*self.0).info.protocol_version)
        }
    }

    pub fn set_owner(&self, owner: bool) {
        unsafe {
            (*self.0).owner = if owner {
                1
            } else {
                0
            }
        }
    }

    pub fn handle_instruction<'a>(&self, parser: &guac_parser) -> Result<(), GuacError> {
        let raw = unsafe {
            libguac_sys::guac_user_handle_instruction(
                self.0,
                parser.opcode().ok_or("Empty opcode")?.as_ptr() as *const _,
                parser.argc() as i32,
                parser.args_mut().as_mut_ptr(),
            )
        };

        match raw {
            0 => Ok(()),
            _ => Err(GuacError::from("Failed to handle instruction"))
        }
    }

    pub fn active(&self) -> i32 {
        unsafe {
            (*self.0).active
        }
    }

    pub fn stop(&self) {
        unsafe {
            libguac_sys::guac_user_stop(self.0)
        }
    }
}

unsafe impl Send for guac_user {}

unsafe impl Sync for guac_user {}

impl Drop for guac_user {
    fn drop(&mut self) {
        println!("guac_user freed");
        // WARNING Do not implement Drop for `guac_user` it handled libguac internally!
    }
}
