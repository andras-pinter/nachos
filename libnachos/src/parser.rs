use crate::error::ParserError;
use crate::socket::Socket;

pub struct Parser {
    guac_parser: libguac::guac_parser,
}

impl Parser {
    pub fn new() -> Result<Self, ParserError> {
        Ok(Parser {
            guac_parser: libguac::guac_parser::alloc().ok_or(ParserError::UnableToCreate)?,
        })
    }

    pub fn parse(&self, buffer: &mut str) {
        let mut len = buffer.len();
        let mut segment = buffer.as_mut();

        while len > 0 {
            let parsed = self.guac_parser.append(
                segment.as_mut(),
                len
            );
            if parsed == 0 {
                break;
            }
            len -= parsed;
            segment = segment[parsed..].as_mut();
        }
    }

    pub fn read(&self, socket: &Socket, timeout: u128) -> Result<(), ParserError> {
        self.guac_parser.read(socket.as_ref(), timeout as i32)
            .map_err(ParserError::ParsingError)
    }

    #[allow(dead_code)]
    pub fn opcode<'a>(&self) -> Option<std::borrow::Cow<'a, str>> {
        self.guac_parser.opcode()
    }

    #[allow(dead_code)]
    pub fn args<'a>(&self) -> Vec<std::borrow::Cow<'a, str>> {
        self.guac_parser.args()
            .into_iter()
            .collect()
    }
}

impl AsRef<libguac::guac_parser> for Parser {
    fn as_ref(&self) -> &libguac::guac_parser {
        &self.guac_parser
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    impl Parser {
        pub fn args_len(&self) -> isize {
            self.guac_parser.argc()
        }
    }

    #[test]
    fn test_parser() {
        let mut buffer = String::from("4.test,8.testdata,5.zxcvb,13.guacamoletest;");
        let parser = Parser::new();
        assert!(parser.is_ok());
        let parser = parser.unwrap();
        assert!(parser.parse(&mut buffer).is_ok());
    }

    #[test]
    fn test_parser_opcode() {
        let mut buffer = String::from("4.test,8.testdata,5.zxcvb,13.guacamoletest;");
        let parser = Parser::new()
            .expect("Failed to get parser");
        assert!(parser.parse(&mut buffer).is_ok());
        let opcode = parser.opcode();
        assert!(opcode.is_some());
        assert_eq!(opcode.unwrap(), "test");
    }

    #[test]
    fn test_parser_number_of_args() {
        let mut buffer = String::from("4.test,8.testdata,5.zxcvb,13.guacamoletest;");
        let parser = Parser::new()
            .expect("Failed to get parser");
        assert!(parser.parse(&mut buffer).is_ok());
        assert_eq!(parser.args_len(), 3);
    }

    #[test]
    fn test_parser_args() {
        let mut buffer = String::from("4.test,8.testdata,5.zxcvb,13.guacamoletest;");
        let expected = vec!["testdata", "zxcvb", "guacamoletest"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        let parser = Parser::new()
            .expect("Failed to get parser");
        assert!(parser.parse(&mut buffer).is_ok());
        assert_eq!(parser.args(), expected);
    }

    #[test]
    fn test_parser_with_extra_bytes() {
        let mut buffer = String::from("4.test,8.testdata,5.zxcvb,13.guacamoletest;XXX");
        let parser = Parser::new();
        assert!(parser.is_ok());
        let parser = parser.unwrap();
        assert!(parser.parse(&mut buffer).is_ok());
        assert_eq!(parser.args_len(), 3);
    }
}
