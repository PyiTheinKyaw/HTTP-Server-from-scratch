use super::error::ParseError;
use super::methods::Methods;
use super::string_parser::StringParser;

use std::str;
pub struct Request {
    pub method: Methods,
    pub path: String,
    pub query: Option<String>,

}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {

        // After Impl From Trait
        // ? = Observe on Result<T,E> if T assign to variable if E return it
        let raw_str = str::from_utf8(buff)?;
        let (method, raw_str) = raw_str.get_next_word().ok_or(ParseError::InvalidRequest)?;
        let (path, raw_str) = raw_str.get_next_word().ok_or(ParseError::InvalidRequest)?;
        let (protocol_version, _) = raw_str.get_next_word().ok_or(ParseError::InvalidRequest)?;

        if protocol_version != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method = method.parse::<Methods>()?;


        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::http::string_parser::StringParser;

    #[test]
    fn test_string_parser() {
        let http_header = String::from("GET / HTTP/1.1");
        let result = &http_header[..];

        assert_eq!(result.get_next_word(), Some(("GET", "/ HTTP/1.1")));
        let input = "";
        let parser = &input;

        assert_eq!(parser.get_next_word(), None);
    }
}

