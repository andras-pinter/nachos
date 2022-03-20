#[derive(Clone)]
pub enum Api {
    Base,
    Session,
    Tunnel,
}

impl AsRef<str> for Api {
    fn as_ref(&self) -> &str {
        match self {
            Api::Base => "api",
            Api::Session => "session",
            Api::Tunnel => "_tunnel",
        }
    }
}

impl Api {
    pub fn call<T>(self, value: T) -> String
    where
        T: AsRef<str>,
    {
        self.call1(value)
    }

    pub fn call0<T>(self) -> String
    where
        T: AsRef<str>,
    {
        let mut buff = Self::init_buffer();
        buff.push_str(self.as_ref());
        buff.push('/');

        buff
    }

    pub fn call1<T>(self, value: T) -> String
    where
        T: AsRef<str>,
    {
        let mut buff = Self::init_buffer();
        buff.push_str(self.as_ref());
        buff.push('/');
        buff.push_str(value.as_ref());

        buff
    }

    pub fn calln<T>(self, values: &[T]) -> String
    where
        T: AsRef<str>,
    {
        let mut buff = Self::init_buffer();
        buff.push_str(self.as_ref());
        buff.push('/');
        for val in values {
            buff.push_str(val.as_ref());
            buff.push('/');
        }

        buff
    }

    pub fn tunnel(session_id: &str) -> String {
        let mut buffer = Self::init_buffer();
        buffer.push_str(Self::Session.as_ref());
        buffer.push('/');
        buffer.push_str(session_id);
        buffer.push('/');
        buffer.push_str(Self::Tunnel.as_ref());

        buffer
    }

    #[inline]
    fn init_buffer() -> String {
        let mut buff = String::new();
        buff.push('/');
        buff.push_str(Self::Base.as_ref());
        buff.push('/');

        buff
    }
}
