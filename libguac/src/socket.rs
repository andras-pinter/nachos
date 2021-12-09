pub struct guac_socket(pub(crate) *mut libguac_sys::guac_socket);

impl guac_socket {
    pub fn open(fd: i32) -> Option<Self> {
        unsafe {
            let socket = libguac_sys::guac_socket_open(fd);

            if socket.is_null() {
                return None
            }

            Some(guac_socket(socket))
        }
    }

    pub fn require_keep_alive(&self) {
        unsafe {
            libguac_sys::guac_socket_require_keep_alive(self.0);
        }
    }
}

unsafe impl Send for guac_socket {}

unsafe impl Sync for guac_socket {}

impl Drop for guac_socket {
    fn drop(&mut self) {
        println!("guac_socket freed");
        // WARNING Do not implement Drop for `guac_socket` it handled libguac internally!
    }
}
