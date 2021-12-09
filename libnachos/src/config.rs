pub trait Configuration {
    fn protocol(&self) -> crate::Protocol;
    fn get(&self, key: &str) -> Option<String>;

    fn resolution(&self) -> (i32, i32);

    fn render(&self, version: &str, args: Vec<std::borrow::Cow<str>>) -> String {
        libguac::guac_proto::Connect {
            version: version.to_string(),
            args: args
                .iter()
                .map(|key| self.get(key).unwrap_or_default())
                .collect()
        }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Protocol;
    use super::*;

    struct TestConfig {
        hostname: String,
        port: u8,
        username: String,
        password: String,
    }

    impl Configuration for TestConfig {
        fn protocol(&self) -> Protocol {
            Protocol::SSH
        }

        fn get(&self, key: &str) -> Option<String> {
            match key {
                "hostname" => Some(self.hostname.clone()),
                "port" => Some(self.port.to_string()),
                "username" => Some(self.username.clone()),
                "password" => Some(self.password.clone()),
                _ => None
            }
        }

        fn resolution() -> (i32, i32) {
            (1024, 768)
        }
    }

    #[test]
    fn test_configuration() {
        let c = TestConfig {
            hostname: "10.0.1.42".to_string(),
            port: 22,
            username: "nachos".to_string(),
            password: "N4cH0s".to_string()
        };
        assert_eq!(c.get("hostname"), Some("10.0.1.42".to_string()));
        assert_eq!(c.get("port"), Some("22".to_string()));
        assert_eq!(c.get("username"), Some("nachos".to_string()));
        assert_eq!(c.get("password"), Some("N4cH0s".to_string()));
        assert_eq!(c.get("chipper"), None);
    }

    #[test]
    fn test_config_rendering() {
        let version = libguac::guac_version::V1_3_0;
        let args = vec!["hostname", "port", "username", "password", "chipper"]
            .into_iter()
            .map(std::borrow::Cow::Borrowed)
            .collect();
        let expected = "7.connect,13.VERSION_1_3_0,9.10.0.1.42,2.22,6.nachos,6.N4cH0s,0.;";
        let c = TestConfig {
            hostname: "10.0.1.42".to_string(),
            port: 22,
            username: "nachos".to_string(),
            password: "N4cH0s".to_string()
        };
        assert_eq!(c.render(version.as_ref(), args), expected);
    }
}

