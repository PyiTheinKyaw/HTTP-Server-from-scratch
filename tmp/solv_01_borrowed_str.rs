use std::str;
use std::str::Utf8Error;

pub trait StringParser<'a> {
    fn get_next_word(&self) -> Option<(&str, &str)>;
    fn get_query(&self) -> Option<&str>;
    fn trim_path(&self) -> Option<&str>;
}

impl<'a> StringParser<'a> for &'a str {
    fn get_next_word(&self) -> Option<(&'a str, &'a str)>{
        for (i,c) in self.chars().enumerate() {
            if c == ' ' || c == '\r' {
                return Some(
                    (&self[..i], &self[i+1..])
                );
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

    let ret = convert(buffer);

    println!("Return String: {:?}", ret);
}

/*
** The Resilience of Language **

Isn't it fascinating how even with all these typos, you can still understand me?
It's a testament to the remarkable human ability to interpret and
fill in the gaps, highlighting the inherent flexibility and resilience of language.

** Addressing Borrowed Ownership **
Now, onto a different kind of gap – the one posed by borrowed ownership in Rust.
I'm currently grappling with finding solutions to circumvent
the need for explicit unwrapping with unwrap(), ok_or, and similar methods.

The goal is to find cleaner, more elegant ways to handle potential ownership errors and
ensure smooth program execution.
 */
fn convert<'a>(buffer: &'a [u8]) -> Option<(&'a str, &'a str)> {
    // let mut rets = None;
    // if let Ok(ret) = str::from_utf8(buffer) {
    //      return ret.get_next_word();
    // }
    //
    // Some((&"HELLO", &"WORLD"))

     let ret = str::from_utf8(buffer).ok()?;
    get_get(ret)
}

fn get_get<'a>(v: &'a str) -> Option<(&'a str, &'a str)>{
    for (i,c) in v.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&v[..i], &v[i+1..]));
        }
    }
    None
}