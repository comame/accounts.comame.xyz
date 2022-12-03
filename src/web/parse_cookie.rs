use std::collections::HashMap;

#[deprecated]
pub fn parse_cookie(value: &str) -> Result<HashMap<String, String>, ()> {
    let mut map: HashMap<String, String> = HashMap::new();

    if value.is_empty() {
        return Ok(map);
    }

    let mut key_chars: Vec<char> = vec![];
    let mut value_chars: Vec<char> = vec![];

    let mut is_name = true;
    let mut is_value = false;

    for char in value.chars() {
        if char == '=' {
            if !is_name || is_value {
                return Err(());
            }

            if key_chars.is_empty() {
                return Err(());
            }

            is_value = true;
            is_name = false;
        } else if char == ';' {
            if is_name || !is_value {
                return Err(());
            }

            if value_chars.is_empty() {
                return Err(());
            }

            is_value = false;
            is_name = false;

            let key: String = key_chars.iter().collect();
            let value: String = value_chars.iter().collect();

            map.insert(key, value);

            key_chars = vec![];
            value_chars = vec![];
        } else if char == ' ' {
            if is_name || is_value {
                return Err(());
            }
        } else {
            if !is_name && !is_value {
                is_name = true;
            }

            if is_value {
                value_chars.push(char);
            }
            if is_name {
                key_chars.push(char);
            }
        }
    }

    let key: String = key_chars.iter().collect();
    let value: String = value_chars.iter().collect();

    if !key.is_empty() && !value.is_empty() {
        map.insert(key, value);
    }

    if map.is_empty() {
        Err(())
    } else {
        Ok(map)
    }
}
