use super::hex;

pub fn encode(str: &str) -> String {
    let mut string = String::new();

    for char in str.chars() {
        let next = match char {
            ':' => "%3A".to_string(),
            '/' => "%2F".to_string(),
            '?' => "%3F".to_string(),
            '#' => "%23".to_string(),
            '[' => "%5B".to_string(),
            ']' => "%5D".to_string(),
            '@' => "%40".to_string(),
            '!' => "%21".to_string(),
            '$' => "%24".to_string(),
            '&' => "%26".to_string(),
            '\'' => "%27".to_string(),
            '(' => "%28".to_string(),
            ')' => "%29".to_string(),
            '*' => "%2A".to_string(),
            '+' => "%2B".to_string(),
            ',' => "%2C".to_string(),
            ';' => "%3B".to_string(),
            '=' => "%3D".to_string(),
            '%' => "%25".to_string(),
            ' ' => "%20".to_string(),
            _ => char.to_string(),
        };
        string.push_str(&next);
    }

    string
}

#[allow(clippy::result_unit_err)]
pub fn decode(str: &str) -> Result<String, ()> {
    let mut string = String::new();
    let mut hex_chars: [char; 2] = ['\0', '\0'];
    let mut i: usize = 0;
    for char in str.chars() {
        if char == '%' {
            i += 1;
            continue;
        } else if char == '+' {
            string.push(' ');
        } else if 0 < i && i <= 2 {
            if !char.is_ascii_hexdigit() {
                return Err(());
            }
            hex_chars[i - 1usize] = char;
            i += 1;

            if i == 3 {
                let hex = hex_chars.iter().collect::<String>();
                let char = hex::decode(&hex).first().unwrap().to_owned() as char;
                string.push(char);
                i = 0;
            }
        } else {
            string.push(char);
        }
    }

    if i != 0 {
        return Err(());
    }

    Ok(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        assert_eq!("Hello, world!", super::decode("Hello,%20world!").unwrap());
    }

    #[test]
    fn plus_space() {
        assert_eq!("a b", super::decode("a+b").unwrap());
    }

    #[test]
    fn empty() {
        assert_eq!("", super::decode("").unwrap());
    }

    #[test]
    fn zero_char() {
        assert_eq!("\0", super::decode("%00").unwrap());
    }

    #[test]
    fn long_encode_decode() {
        let str = r#"Special characters needing encoding are: ':', '/', '?', '#', '[', ']', '@', '!', '$', '&', "'", '(', ')', '*', '+', ',', ';', '=', as well as '%' itself. Other characters don't need to be encoded, though they could."#;
        assert_eq!(str, decode(&encode(str)).unwrap());
    }

    #[test]
    fn melformed_1() {
        assert!(super::decode("%").is_err());
    }

    #[test]
    fn melformed_2() {
        assert!(super::decode("%2").is_err());
    }

    #[test]
    fn melformed_3() {
        assert!(super::decode("abcde%").is_err());
    }

    #[test]
    fn melformed_4() {
        assert!(super::decode("abcde%2").is_err());
    }

    #[test]
    fn melformed_5() {
        assert!(super::decode("%z").is_err());
    }

    #[test]
    fn melformed_6() {
        assert!(super::decode("%2z").is_err());
    }
}
