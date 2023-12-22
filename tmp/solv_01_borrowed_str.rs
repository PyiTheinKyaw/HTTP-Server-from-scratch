use std::str;

pub trait StringParser<'a> {
    fn get_next_word(&self) -> Option<(&'a str, &'a str)>;
    fn get_path_query(&self) -> Option<(&'a str, &'a str)>;
}

impl<'a> StringParser<'a> for &'a str {
    fn get_next_word(&self) -> Option<(&'a str, &'a str)>{
        for (i,c) in self.chars().enumerate() {
            if c == ' ' || c == '\r' {
                return Some((&self[..i], &self[i+1..]));
            }
        }
        None
    }

    fn get_path_query(&self) -> Option<(&'a str, &'a str)> {
        if let Some(index) = self.find('?') {
            return Some((&self[..index], &self[index+1..]))
        }
        None
    }
}

fn main() {
    let buffer = &[
        0x47, 0x45, 0x54, 0x20, 0x2f, 0x68, 0x6f, 0x6d, 0x65, 0x3f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
        0x6e, 0x3d, 0x73, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x26, 0x6d, 0x6f, 0x64, 0x65, 0x3d, 0x64, 0x65,
        0x62, 0x75, 0x67, 0x26, 0x65, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x3d, 0x32, 0x30, 0x32, 0x31,
        0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31
    ];

    let(method, path, query) = convert(buffer);
    println!("Method: {}, Path:{}, Query: {:?}", method, path, query);
}

fn convert<'a>(buffer: &'a [u8]) -> (&'a str, &'a str, Option<&'a str>) {
    let converted_str = str::from_utf8(buffer).unwrap();

    let (method, raw_str) = converted_str.get_next_word().unwrap();
    let (mut path, protocol_version) = raw_str.get_next_word().unwrap();

    println!("Protocol version: {:?}", protocol_version);

    let mut query = None;
    if let Some((_path, _query)) = path.get_path_query() {
        query = Some(_query);
        path = _path;
    }

    (method, path, query)
}