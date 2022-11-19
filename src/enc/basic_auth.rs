use super::base64;

pub struct BasicAuthorization {
    pub user: String,
    pub password: String,
}

impl BasicAuthorization {
    pub fn decode(header_value: &str) -> Result<Self, ()> {
        if header_value.len() < 7 {
            return Err(());
        }
        let basic_slice = &header_value[0..6];
        let encoded = &header_value[6..];

        if basic_slice != "Basic " {
            return Err(());
        }

        let decoded = base64::decode_base64(encoded)?;
        let decoded = String::from_utf8(decoded);
        if decoded.is_err() {
            return Err(());
        }
        let decoded = decoded.unwrap();

        let mut user = String::new();
        let mut password = String::new();
        let mut after_colon = false;

        for char in decoded.chars() {
            if char == ':' {
                after_colon = true;
                continue;
            }
            if after_colon {
                password.push(char);
            } else {
                user.push(char);
            }
        }

        Ok(Self { user, password })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let result = super::BasicAuthorization::decode("Basic cm9vdDpwYXNzd29yZA==").unwrap();
        assert_eq!(result.user, "root");
        assert_eq!(result.password, "password");
    }
}
