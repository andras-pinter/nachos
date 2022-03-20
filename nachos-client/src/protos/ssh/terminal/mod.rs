mod bindings;

pub struct Terminal {
    term: bindings::Terminal,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            term: bindings::Terminal::new(),
        }
    }

    pub fn open(&self, parent: &'static str) {
        match gloo_utils::document().get_element_by_id(parent) {
            Some(parent) => self.term.open(parent),
            None => gloo_console::error!("No such element by id: {}", parent),
        }
    }

    pub fn write(&self, data: Vec<u8>) {
        self.term.write(data)
    }
}
