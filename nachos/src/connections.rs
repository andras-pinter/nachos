use common::Connection;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ConnectionStore(HashMap<String, Connection>);

impl ConnectionStore {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::default()))
    }

    pub fn contains(&self, session_id: &str) -> bool {
        self.0.contains_key(session_id)
    }

    pub fn get(&self, session_id: &str) -> Option<&Connection> {
        self.0.get(session_id)
    }

    #[cfg(all(feature = "default", not(feature = "testConnections")))]
    pub fn remove(&mut self, session_id: &str) -> Option<Connection> {
        self.0.remove(session_id)
    }

    #[cfg(feature = "testConnections")]
    pub fn remove(&mut self, session_id: &str) -> Option<Connection> {
        self.0.get(session_id).cloned()
    }
}

impl Default for ConnectionStore {
    #[cfg(all(feature = "default", not(feature = "testConnections")))]
    fn default() -> Self {
        ConnectionStore(HashMap::default())
    }

    #[cfg(feature = "testConnections")]
    fn default() -> Self {
        let mut session_db = HashMap::default();
        let ssh_cfg = common::SshConfig {
            hostname: "10.0.1.8".to_string(),
            port: 22,
        };
        session_db.insert("testSsh".to_string(), Connection::SSH(ssh_cfg));

        ConnectionStore(session_db)
    }
}
