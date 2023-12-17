use super::error::ParseError;
use super::methods::Methods;

use std::str;
pub struct Request {
    pub path: String,
    pub query: Option<String>,
    pub method: Methods,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buff) {
        //     Ok(raw_str) => {},
        //     Err(e) => {
        //         return Err(ParseError::InvalidEncoding)
        //     }
        // }

        // match str::from_utf8(buff).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(raw_str) => {},
        //     Err(e) => return Err(e)
        // }

        // let raw_str = str::from_utf8(buff).or(Err(ParseError::InvalidEncoding))?;

        // After Impl From Trait
        let raw_str = str::from_utf8(buff)?;


        unimplemented!()
    }
}
