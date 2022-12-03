use std::collections::HashMap;

use crate::enc::url::decode;

pub fn parse(body: &str) -> Result<HashMap<String, String>, ()> {
    let mut map: HashMap<String, String> = HashMap::new();
    let mut is_key = true;
    let mut is_value = false;
    let mut tmp_key: Vec<char> = vec![];
    let mut tmp_value: Vec<char> = vec![];

    if body.is_empty() {
        return Ok(map);
    }

    for c in body.chars() {
        if is_value && c == '=' {
            return Err(());
        }

        if is_key && c == '=' {
            is_key = false;
            is_value = true;

            if tmp_key.is_empty() {
                return Err(());
            }
        } else if is_value && c == '&' {
            is_key = true;
            is_value = false;

            if tmp_value.is_empty() {
                return Err(());
            }

            map.insert(
                decode(&tmp_key.iter().collect::<String>()),
                decode(&tmp_value.iter().collect::<String>()),
            );
            tmp_key = vec![];
            tmp_value = vec![];
        } else if is_key && c == '&' {
            is_key = true;
            is_value = false;

            if tmp_key.is_empty() {
                return Err(());
            }

            map.insert(
                decode(&tmp_key.iter().collect::<String>()),
                String::from(""),
            );

            tmp_key = vec![];
            tmp_value = vec![];
        } else if is_key {
            tmp_key.push(c);
        } else if is_value {
            tmp_value.push(c);
        } else {
            panic!();
        }
    }

    map.insert(
        decode(&tmp_key.iter().collect::<String>()),
        decode(&tmp_value.iter().collect::<String>()),
    );

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        let map = parse("field1=value1").unwrap();
        assert!(map.len() == 1);
        assert_eq!(map.get("field1").unwrap(), "value1");

        let map = parse("field1=value1&field2=value2").unwrap();
        assert!(map.len() == 2);
        assert_eq!(map.get("field1").unwrap(), "value1");
        assert_eq!(map.get("field2").unwrap(), "value2");

        let map = parse("field").unwrap();
        assert!(map.len() == 1);
        assert_eq!(map.get("field").unwrap(), "");

        let map = parse("field1&field2=value2").unwrap();
        assert!(map.len() == 2);
        assert_eq!(map.get("field1").unwrap(), "");
        assert_eq!(map.get("field2").unwrap(), "value2");
    }

    #[test]
    fn url_encoded() {
        let map = parse("key=hello%20world").unwrap();
        assert!(map.len() == 1);
        assert_eq!(map.get("key").unwrap(), "hello world");
    }

    #[test]
    fn empty() {
        let map = parse("").unwrap();
        assert!(map.is_empty());

        let map = parse(" ").unwrap();
        assert!(map.is_empty());
    }

    #[test]
    fn errors() {
        let errors = vec!["=", "&", "&foo", "=bar"];
        for error in errors {
            dbg!(error);
            assert!(parse(error).is_err());
        }
    }
}
