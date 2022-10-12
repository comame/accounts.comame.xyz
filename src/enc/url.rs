use super::hex;

pub fn decode(str: &str) -> String {
    let mut string = String::new();
    let mut hex_chars: [char; 2] = ['\0', '\0'];
    let mut i: usize = 0;
    for char in str.chars() {
        if char == '%' {
            i += 1;
            continue;
        } else if 0 < i && i <= 2 {
            if !char.is_ascii_hexdigit() {
                panic!();
            }
            hex_chars[i - 1usize] = char;
            i += 1;

            if i == 3 {
                let hex = hex_chars.iter().collect::<String>();
                let char = hex::decode_hex(&hex).first().unwrap().to_owned() as char;
                string.push(char);
                i = 0;
            }
        } else {
            string.push(char);
        }
    }

    if i != 0 {
        panic!();
    }

    string
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_world() {
        assert_eq!("Hello, world!", super::decode("Hello,%20world!"));
    }

    #[test]
    fn empty() {
        super::decode("");
    }

    #[test]
    fn zero_char() {
        super::decode("%00");
    }

    #[test]
    #[should_panic]
    fn melformed_1() {
        super::decode("%");
    }

    #[test]
    #[should_panic]
    fn melformed_2() {
        super::decode("%2");
    }

    #[test]
    #[should_panic]
    fn melformed_3() {
        super::decode("abcde%");
    }

    #[test]
    #[should_panic]
    fn melformed_4() {
        super::decode("abcde%2");
    }

    #[test]
    #[should_panic]
    fn melformed_5() {
        super::decode("%z");
    }

    #[test]
    #[should_panic]
    fn melformed_6() {
        super::decode("%2z");
    }
}
