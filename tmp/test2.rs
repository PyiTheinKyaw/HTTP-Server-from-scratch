#[derive(Debug)]
enum ParseError {
    InvalidRequest,
}

pub trait StringParser {
    fn get_next_word(&self) -> Option<(&str, &str)>;
    fn trim_path(&self) -> Option<&str>;
}

impl StringParser for &str {
    fn get_next_word(&self) -> Option<(&str, &str)> {
        for (i, c) in self.chars().enumerate() {
            if c == ' ' || c == '\r' {
                return Some((&self[..i], &self[i + 1..]));
            }
        }
        None
    }

    fn trim_path(&self) -> Option<&str> {
        for (i, c) in self.chars().enumerate() {
            if c == '?' {
                return Some(&self[..i]);
            }
        }
        Some(self)
    }
}

#[derive(Debug)]
pub struct Request<'buff> {
    pub path: &'buff str,
}

fn main() {
    let buffer = "GET /path/to/resource?query=param HTTP/1.1";

    let raw_str = match buffer.find(' ') {
        Some(index) => &buffer[index + 1..],
        None => return,
    };

    let path = match raw_str.trim_path() {
        Some(p) => p,
        None => return,
    };

    let request = Request { path };
    println!("{:?}", request);
}
