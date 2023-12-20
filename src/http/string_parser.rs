pub trait StringParser {
    fn get_next_word(&self) -> Option<(&str, &str)>;

    fn get_query(&self) -> Option<&str>;
    fn trim_path(&self) -> Option<&str>;
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