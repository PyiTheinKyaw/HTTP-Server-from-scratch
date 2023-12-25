use std::fmt::Display;
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;
use super::response::status_code::StatusCode;

pub mod status_code;

#[derive(Debug)]
pub struct Response<'res> {
    status_code: StatusCode,
    server: &'res str,
    protocol_version: &'res str,
    body: Option<String>,
}

impl<'res> Response<'res> {
    pub fn new(status_code: StatusCode, server: &'res str, protocol_version: &'res str, body: Option<String>) -> Self {
        Self { status_code, server, protocol_version, body}
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
           self.protocol_version,
           self.status_code,
           self.status_code.reason_phrase(),
           self.server,
           body
        )
    }
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
