use std::{borrow::BorrowMut, collections::HashMap};

use crate::enc::url_encode;

pub struct QueryBuilder {
    value: HashMap<String, String>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
        }
    }

    pub fn append_key(&mut self, key: &str) -> &mut Self {
        self.value
            .borrow_mut()
            .insert(key.to_string(), "".to_string());
        self
    }

    pub fn append(&mut self, key: &str, value: &str) -> &mut Self {
        self.value
            .borrow_mut()
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(&self) -> String {
        let mut query: Vec<String> = vec![];
        let mut keys: Vec<String> = self.value.keys().cloned().collect();

        keys.sort();
        for key in keys {
            let value = self.value.get(&key).unwrap();
            if value.is_empty() {
                query.push(format!("{key}"));
            } else {
                query.push(format!("{key}={}", url_encode::encode(value)));
            }
        }
        query.join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_key() {
        let result = QueryBuilder::new().append_key("key").build();
        assert_eq!(result, "key");
    }

    #[test]
    fn multiple_key() {
        let result = QueryBuilder::new().append_key("a").append_key("b").build();
        assert_eq!(result, "a&b");
    }

    #[test]
    fn single_value() {
        let result = QueryBuilder::new().append("key", "value").build();
        assert_eq!(result, "key=value");
    }

    #[test]
    fn multiple_value() {
        let result = QueryBuilder::new()
            .append("key", "value")
            .append("key2", "value2")
            .build();
        assert_eq!(result, "key=value&key2=value2");
    }

    #[test]
    fn mixed() {
        let result = QueryBuilder::new()
            .append_key("key1")
            .append("key2", "value")
            .build();
        assert_eq!(result, "key1&key2=value");
    }

    #[test]
    fn url_encode() {
        let result = QueryBuilder::new().append("key", "hello world").build();
        assert_eq!(result, "key=hello%20world");
    }
}
