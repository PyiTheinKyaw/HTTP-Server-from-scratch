use super::error::ParseError;
use super::methods::Methods;
use super::string_parser::StringParser;


use std::str;
use crate::http::query_string::QueryString;

#[derive(Debug)]
pub struct Request<'buff> {
    pub method: Methods,
    pub path: &'buff str,
    pub query: Option<QueryString<'buff>>,
}

impl<'buff> TryFrom<&'buff [u8]> for Request<'buff> {
    type Error = ParseError;

    fn try_from(buffer: &'buff [u8]) -> Result<Self, Self::Error> {

        // 1. Parse the raw bytes into a string slice:
        let converted_str = str::from_utf8(buffer)?;

        // 2. Use StringParser methods to extract request components:
        let (method, _raw_str) = converted_str.get_next_word().ok_or(Self::Error::InvalidMethod)?;
        let (mut path, _raw_str) = _raw_str.get_next_word().ok_or(Self::Error::InvalidRequest)?;
        let (protocol_version, _raw_str) = _raw_str.get_next_word().ok_or(Self::Error::InvalidEncoding)?;

        if protocol_version != "HTTP/1.1" {
            return Err(
                Self::Error::InvalidProtocol
            )
        }

        let mut query = None;
        if let Some((_path, _query)) = path.get_path_query() {
            query = Some(_query);
            path = _path;
        }

        let http_method = method.parse::<Methods>()?;

        let mut query_string= None;
        if let Some(qs) = query {
            query_string = Some(QueryString::from(qs));
        }


        // 3. Create and return the Request instance:
        Ok(Request {
            method: http_method,
            path,
            query: query_string
        })
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

