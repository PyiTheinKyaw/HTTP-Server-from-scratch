use std::collections::HashMap;
// a=1&b=2&c&d=&e===&d=7&d=adc
pub struct QueryString<'query> {
    data: HashMap<&'query str, Value<'query>>,
}

enum Value<'query> {
    Single(&'query str),
    Multiple(Vec<&'query str>)
}

impl<'query> QueryString<'query> {
    fn get(&self, key: &str) -> Option<&Value> {
        unimplemented!()
    }
}

#[cfg(test)]
mod query_string_test {
    use crate::http::string_parser::StringParser;

    #[test]
    fn test_string_parser() {

    }
}