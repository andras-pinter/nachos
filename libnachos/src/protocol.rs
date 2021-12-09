#[derive(Clone)]
pub enum Protocol {
    SSH,
    VNC,
    RDP,
}

impl Into<&'static str> for Protocol {
    fn into(self) -> &'static str {
        match self {
            Protocol::SSH => "ssh",
            Protocol::VNC => "vnc",
            Protocol::RDP => "rdp",
        }
    }
}
