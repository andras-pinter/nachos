use std::hash::{Hash, Hasher};
use uuid::Uuid;

pub struct Session<H>
where H: crate::Handler,
{
    session_id: Uuid,
    config: H::Configuration,
    handler: H,
}

impl<H> Session<H>
where H: crate::Handler,
{
    pub fn new(handler: H, config: H::Configuration) -> Self {
        Session {
            session_id: uuid::Uuid::new_v4(),
            config,
            handler,
        }
    }
}

impl<Ha> Hash for Session<Ha>
where Ha: crate::Handler,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.session_id.hash(state);
    }
}

impl<H> PartialEq for Session<H>
where H: crate::Handler,
{
    fn eq(&self, other: &Self) -> bool {
        self.session_id == other.session_id
    }
}

impl<H: crate::Handler> Eq for Session<H> {}
