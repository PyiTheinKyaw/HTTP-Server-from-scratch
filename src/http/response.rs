use std::io::{Result as IoResult, Write};
use crate::http::response::properties::ResponseProperties;
use super::response::status_code::StatusCode;

pub mod status_code;
mod properties;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body}
    }

    pub fn send(&self, tcp_stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(_body) => _body,
            None => ""
        };

        write!(
            tcp_stream,
           "{} {} {}\r\n\
            Server: {}\r\n
            \r\n\
            {}",
           Self::PROTOCOL_VERSION,
           self.status_code,
           self.status_code.reason_phrase(),
           Self::SERVER_NAME,
           body
        )
    }
}

impl<'rp> ResponseProperties<'rp> for Response {
    const SERVER_NAME: &'rp str = "OuOu";
    const PROTOCOL_VERSION: &'rp str = "HTTP/1.1";
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_status_code() -> std::io::Result<()>{
        let mut f = File::create("foo.txt")?;
        f.write_all(&1234_u32.to_be_bytes())?;
        assert!(true);
        Ok(())
    }
}
