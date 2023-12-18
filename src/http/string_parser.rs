pub trait StringParser {
    fn get_next_word(&self) -> Option<(&str, &str)>;
}

impl StringParser for &str {
    fn get_next_word(&self) -> Option<(&str, &str)>{
        for (i,c) in self.chars().enumerate() {
            if c == ' ' || c == '\r' {
                return Some((&self[..i], &self[i+1..]));
            }
        }
        None
    }
}