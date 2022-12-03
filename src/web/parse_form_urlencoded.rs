use std::collections::HashMap;

#[deprecated]
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
                http::enc::url_encode::decode(&tmp_key.iter().collect::<String>()).unwrap(),
                http::enc::url_encode::decode(&tmp_value.iter().collect::<String>()).unwrap(),
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
                http::enc::url_encode::decode(&tmp_key.iter().collect::<String>()).unwrap(),
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
        http::enc::url_encode::decode(&tmp_key.iter().collect::<String>()).unwrap(),
        http::enc::url_encode::decode(&tmp_value.iter().collect::<String>()).unwrap(),
    );

    Ok(map)
}
