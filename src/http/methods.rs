use std::str::FromStr;
use super::error::ParseError;

#[derive(Debug, PartialEq)]
pub enum Methods {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl FromStr for Methods {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Methods::GET),
            "HEAD" => Ok(Methods::HEAD),
            "POST" => Ok(Methods::POST),
            "PUT" => Ok(Methods::PUT),
            "DELETE" => Ok(Methods::DELETE),
            "CONNECT" => Ok(Methods::CONNECT),
            "OPTIONS" => Ok(Methods::OPTIONS),
            "TRACE" => Ok(Methods::TRACE),
            "PATCH" => Ok(Methods::PATCH),
            _ => Err(ParseError::InvalidMethod)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http::methods::Methods;

    #[test]
    fn test_methods() {
        let expected = Ok(Methods::GET);
        assert_eq!(expected, "GET".parse::<Methods>());
    }
}