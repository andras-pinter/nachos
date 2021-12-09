pub enum guac_version {
    V1_3_0,
    V1_1_0,
    V1_0_0,
    Unknown,
}

impl Into<u32> for guac_version {
    fn into(self) -> u32 {
        match self {
            guac_version::V1_0_0 => libguac_sys::guac_protocol_version_GUAC_PROTOCOL_VERSION_1_0_0,
            guac_version::V1_1_0 => libguac_sys::guac_protocol_version_GUAC_PROTOCOL_VERSION_1_1_0,
            guac_version::V1_3_0 => libguac_sys::guac_protocol_version_GUAC_PROTOCOL_VERSION_1_3_0,
            _ => libguac_sys::guac_protocol_version_GUAC_PROTOCOL_VERSION_UNKNOWN,
        }
    }
}

impl From<u32> for guac_version {
    fn from(guac_protocol_version: u32) -> Self {
        match guac_protocol_version {
            libguac_sys::guac_protocol_version_GUAC_PROTOCOL_VERSION_1_0_0 => guac_version::V1_0_0,
            libguac_sys::guac_protocol_version_GUAC_PROTOCOL_VERSION_1_1_0 => guac_version::V1_1_0,
            libguac_sys::guac_protocol_version_GUAC_PROTOCOL_VERSION_1_3_0 => guac_version::V1_3_0,
            _ => guac_version::Unknown,
        }
    }
}

impl AsRef<str> for guac_version {
    fn as_ref(&self) -> &str {
        match self {
            guac_version::V1_0_0 | guac_version::Unknown => "VERSION_1_0_0",
            guac_version::V1_1_0 => "VERSION_1_1_0",
            guac_version::V1_3_0 => "VERSION_1_3_0",
        }
    }
}

pub enum guac_proto<'a> {
    Connect{
        version: String,
        args: Vec<String>,
    },
    Ready(&'a str),
}

impl<'a> ToString for guac_proto<'a> {
    fn to_string(&self) -> String {
        use guac_proto::*;

        match self {
            Connect { version, args } => {
                let mut buffer = String::from("7.connect,");
                buffer.push_str(&version.len().to_string());
                buffer.push('.');
                buffer.push_str(version);
                buffer.push(',');

                for arg in args.iter() {
                    buffer.push_str(&arg.len().to_string());
                    buffer.push('.');
                    buffer.push_str(arg);
                    buffer.push(',');
                }
                buffer.pop();
                buffer.push(';');
                buffer
            },
            Ready(connection_id) => format!("0.,{}.{};", connection_id.len().to_string(), connection_id)
        }
    }
}
