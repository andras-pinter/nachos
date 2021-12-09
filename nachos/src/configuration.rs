use libnachos::{Protocol, Configuration};

pub struct Config {
    pub protocol: Protocol,
    pub hostname: String,
    pub port: usize,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            protocol: Protocol::SSH,
            hostname: "".to_string(),
            port: 0,
            username: None,
            password: None
        }
    }
}

impl Configuration for Config {
    fn protocol(&self) -> Protocol {
        self.protocol.clone()
    }

    fn get(&self, key: &str) -> Option<String> {
        match key {
            "hostname" => Some(self.hostname.clone()),
            "port" => Some(self.port.to_string()),
            "username" => self.username.clone(),
            "password" => self.password.clone(),
            _ => None
        }
    }

    fn resolution(&self) -> (i32, i32) {
        (1024, 768)
    }
}
