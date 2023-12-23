use std::collections::HashMap;
use std::fmt::{Display, format, Formatter, Result as FmtResult};

pub struct QueryString<'query> {
    data: HashMap<&'query str, Value<'query>>,
}

enum Value<'value> {
    Single(&'value str),
    Multiple(Vec<&'value str>)
}

impl<'query> QueryString<'query> {
    fn get(&self, key: &str) -> Option<&'query Value> {
        self.data.get(key)
    }

    fn new(data: HashMap<&'query str, Value<'query>>) -> Self {
        QueryString {
            data
        }
    }
}

impl<'string_to_qs> From<&'string_to_qs str> for QueryString<'string_to_qs> {
    fn from(s: &'string_to_qs str) -> Self {
        let mut data: HashMap<&str, Value> = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut value = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[.. i];
                value = &sub_str[i+1..];
            }

            data.entry(key)
                .and_modify(|existed_value| {
                    match existed_value {
                        Value::Single(old_val) => {
                            let vec: Vec<&str> = vec![old_val, value];
                            *existed_value = Value::Multiple(vec);
                        },
                        Value::Multiple(vec) => {
                            vec.push(value);
                        }
                    }
                })
                .or_insert(Value::Single(value));
        }

        QueryString::new(data)
    }
}

#[cfg(test)]
mod query_string_test {
    use crate::http::query_string::QueryString;
    use crate::http::query_string::Value;

    #[test]
    fn test_parse_simple_query_string() {
        let qs = "key1=value1&key2=value2";
        let query_string = QueryString::from(qs);

        assert_eq!(query_string.get("key1"), Some(Value::Single("value1")));
        assert_eq!(query_string.get("key2"), Some(Value::Single("value2")));
        assert_eq!(query_string.get("key3"), None);
    }

    #[test]
    fn test_parse_multiple_values() {
        let qs = "key=value1&key=value2";
        let query_string = QueryString::from(qs);

        assert_eq!(query_string.get("key"), Some(Value::Multiple(vec!["value1", "value2"])));
    }

    #[test]
    fn test_parse_encoded_values() {
        let qs = "key=%20value%20with%20spaces";
        let query_string = QueryString::from(qs);

        assert_eq!(query_string.get("key"), Some(Value::Single(" value with spaces")));
    }

}