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