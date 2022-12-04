pub fn normalize_header_key(header_key: &str) -> String {
    let words: Vec<&str> = header_key.split('-').collect();
    let mut new_words: Vec<String> = vec![];
    for word in words {
        let mut new_word = String::new();
        for (i, char) in word.chars().enumerate() {
            if i == 0 {
                new_word.push_str(&char.to_uppercase().to_string());
            } else {
                new_word.push_str(&char.to_lowercase().to_string());
            }
        }
        new_words.push(new_word);
    }
    new_words.join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Authorization", normalize_header_key("authorization"));
        assert_eq!("Authorization", normalize_header_key("Authorization"));
        assert_eq!("Authorization", normalize_header_key("AUTHORIZATION"));

        assert_eq!("Www-Authenticate", normalize_header_key("WWW-Authenticate"));
        assert_eq!("Www-Authenticate", normalize_header_key("www-authenticate"));
        assert_eq!("Www-Authenticate", normalize_header_key("WWW-AUTHENTICATE"));
    }
}
