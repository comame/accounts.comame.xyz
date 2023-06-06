use std::collections::HashMap;

use super::url_encode;

#[allow(clippy::result_unit_err)]
pub fn parse(body: &str) -> Result<HashMap<String, String>, ()> {
    if body.trim().is_empty() {
        return Ok(HashMap::new());
    }

    if body == "&" || body == "=" {
        return Err(());
    }

    let mut map: HashMap<String, String> = HashMap::new();

    let entries: Vec<&str> = body.split('&').map(|str| str.trim()).collect();

    for entry in entries {
        let key_value: Vec<&str> = entry.split('=').collect();
        if key_value.len() > 2 {
            return Err(());
        }

        let key = key_value.first().cloned().unwrap();
        let value = url_encode::decode(key_value.get(1).cloned().unwrap_or(""));

        if key.is_empty() || value.is_err() {
            return Err(());
        }

        map.insert(key.to_string(), value.unwrap().to_string());
    }

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
            assert!(parse(error).is_err());
        }
    }
}
