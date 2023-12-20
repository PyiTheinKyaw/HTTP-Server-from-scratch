use std::str;

pub trait StringParser {
    fn get_next_word(&self) -> Option<(&str, &str)>;

    fn get_query(&self) -> Option<&str>;
    fn trim_path(&self) -> Option<&str>;
}

impl StringParser for &str {
    fn get_next_word<'a>(&'a self) -> Option<(&'a str, &'a str)>{
        for (i,c) in self.chars().enumerate() {
            if c == ' ' || c == '\r' {
                return Some((&self[..i], &self[i+1..]));
            }
        }
        None
    }

    fn get_query(&self) -> Option<&str> {
        if let Some(index) = self.find('?') {
            return Some(&self[index+1..]);
        }
        None
    }

    fn trim_path(&self) -> Option<&str> {
        if let Some(index) = self.find('?') {
            return Some(&self[..index]);
        }

        Some(self)
    }
}

fn main() {
    let buffer = &[
        0x47, 0x45, 0x54, 0x20, 0x2f, 0x68, 0x6f, 0x6d, 0x65, 0x3f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
        0x6e, 0x3d, 0x73, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x26, 0x6d, 0x6f, 0x64, 0x65, 0x3d, 0x64, 0x65,
        0x62, 0x75, 0x67, 0x26, 0x65, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x3d, 0x32, 0x30, 0x32, 0x31,
        0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31
    ];

    let(path, query) = convert(buffer);
    // convert(buffer);
}

fn convert(buffer: &[u8]) -> (&str, &str) {
    let converted_str = str::from_utf8(buffer).unwrap();

    let (method, path_raw) = converted_str.get_next_word().unwrap();
    let (path_query, raw_protocol) = path_raw.get_next_word().unwrap();

    let query = path_query.get_query().unwrap();
    let path = path_query.trim_path().unwrap();

    println!("Method: {:?}, path_raw: {:?}", method, path_raw);
    println!("path_query: {:?}, raw_protocol: {:?}", path_query, raw_protocol);
    println!("query: {:?}, path_raw: {:?}", query, path);
    println!("Converted Str: {:?}", converted_str);

    (path, query)
}