pub trait StringParser<'a> {
    fn get_first_word(&self) -> Option<&'a str>;
}

impl<'a> StringParser<'a> for &'a str {
    fn get_first_word(&self) -> Option<&'a str> {
        Some(&self[..6])
    }
}


fn main() {
    let hello_world = "HELLO WORLD";
    take_ref(&hello_world);
}

fn take_ref<'a>(v: &'a str) -> &'a str {
    v.get_first_word().unwrap()
}