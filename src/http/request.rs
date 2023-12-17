use super::methods::Methods;

pub struct Request {
    pub path: String,
    pub query: Option<String>,
    pub method: Methods
}
