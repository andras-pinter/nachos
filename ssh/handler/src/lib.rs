mod configuration;

pub use configuration::SshConfiguration;
use libnachos::Handler;

#[derive(Clone, PartialEq, Eq)]
pub struct Ssh;

impl Handler for Ssh {
    type Configuration = SshConfiguration;
}
