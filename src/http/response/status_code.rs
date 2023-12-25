pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::NotFound => "Not Found",
            Self::BadRequest => "Bad Request"
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http::response::status_code::StatusCode;

    #[test]
    fn test_status_code() {
        StatusCode::Ok.reason_phrase();
        assert!(true);
    }
}
