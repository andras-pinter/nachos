use crate::{guac_socket, GuacError};

pub struct guac_parser(pub(crate) *mut libguac_sys::guac_parser);

impl guac_parser {
    pub fn alloc() -> Option<Self> {
        unsafe {
            let parser = libguac_sys::guac_parser_alloc();

            if parser.is_null() {
                return None
            }

            Some(guac_parser(parser))
        }
    }

    pub fn append(&self, buffer: &mut str, length: usize) -> usize {
        assert!(length < i32::MAX as usize, "Invalid length instruction");
        use std::os::raw::c_void;
        unsafe {
            libguac_sys::guac_parser_append(self.0, buffer as *mut _ as *mut c_void, length as i32) as usize
        }
    }

    pub fn opcode<'a>(&self) -> Option<std::borrow::Cow<'a, str>> {
        unsafe {
            if (*self.0).opcode.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr((*self.0).opcode).to_string_lossy())
            }
        }
    }

    pub fn argc(&self) -> isize {
        unsafe {
            (*self.0).argc as isize
        }
    }

    pub fn args<'a>(&self) -> Vec<std::borrow::Cow<'a, str>> {
        let mut args = Vec::new();
        unsafe {
            let array_ptr: *mut *mut std::os::raw::c_char = (*self.0).argv;
            for offset in 0..self.argc() {
                let val = array_ptr.offset(offset);
                if !val.is_null() {
                    args.push(std::ffi::CStr::from_ptr(*val).to_string_lossy());
                }
            }
        };

        args
    }

    pub fn args_mut(&self) -> Vec<*mut i8> {
        self.args_mut_skip(0)
    }

    pub fn args_mut_skip(&self, skip: usize) -> Vec<*mut i8> {
        self.args()
            .into_iter()
            .skip(skip)
            .filter_map(|arg| std::ffi::CString::new(arg.to_string()).ok())
            .map(|cstr| cstr.into_raw())
            .collect()
    }

    pub fn read(&self, socket: &guac_socket, timeout: i32) -> Result<(), GuacError> {
        let ret = unsafe {
            libguac_sys::guac_parser_read(self.0, socket.0, timeout)
        };

        match ret {
            0 => Ok(()),
            _ => Err(GuacError::read()),
        }
    }
}

unsafe impl Send for guac_parser {}

unsafe impl Sync for guac_parser {}

impl Drop for guac_parser {
    fn drop(&mut self) {
        println!("guac_parser freed");
        // WARNING Do not implement Drop for `guac_parser` it handled libguac internally!
    }
}
