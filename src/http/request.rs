use super::error::ParseError;
use super::methods::Methods;
use super::string_parser::StringParser;

use std::str;

#[derive(Debug)]
pub struct Request<'buff> {
    pub method: Methods,
    pub path: &'buff str,
    pub query: Option<&'buff str>,

}

impl<'buff> TryFrom<&[u8]> for Request<'buff> {
    type Error = ParseError;

    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {

        // After Impl From Trait
        // ? = Observe on Result<T,E> if T assign to variable if E return it
        let raw_str = str::from_utf8(buff)?;

        let (method, raw_str) = raw_str.get_next_word().ok_or(ParseError::InvalidRequest)?;
        let (mut path, raw_str) = raw_str.get_next_word().ok_or(ParseError::InvalidRequest)?;
        let (protocol_version, _) = raw_str.get_next_word().ok_or(ParseError::InvalidRequest)?;

        if protocol_version != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method = method.parse::<Methods>()?;

        let query = path.get_query();
        path.trim_path();

        return Ok(Request {
            method,
            path,
            query
        });
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

    #[test]
    fn test_path_separate_into_query() {
        let mut path = "/home?key1=value&key2=value2";

        let expected_path = "/home";
        let expected_query = Some("key1=value&key2=value2".to_string());

        let query = path.separate_query();

        assert_eq!(expected_path, &path[..]);
        assert_eq!(expected_query, query);

        assert!(true);
    }

    #[test]
    fn test_get_query() {
        let path = "/home?key1=value&key2=value2";

        let expected_query = Some("key1=value&key2=value2");

        let query = path.get_query();
        assert_eq!(expected_query, query);

        assert!(true);
    }

    #[test]
    fn test_trim_path() {
        let mut path = "/home?key1=value&key2=value2";

        let expected_home = "/home";

        path.trim_path();
        assert_eq!(expected_home, path);
        assert_eq!(None, path.get_query());

        assert!(true);
    }
}

