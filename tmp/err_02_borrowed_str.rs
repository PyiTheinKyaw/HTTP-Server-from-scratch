pub trait StringParser {
    fn get_first_word(&self) -> Option<&str>;
}

impl StringParser for &str {
    fn get_first_word(&self) -> Option<&str> {
        Some(&self[..6])
    }
}


fn main() {
    let hello_world = "HELLO WORLD";
    take_ref(&hello_world);
}

fn take_ref(v: &str) -> &str {
    v.get_first_word().unwrap()
}

fn get_get(v: &str) -> Option<&str> {
    Some(&v[..])
}

