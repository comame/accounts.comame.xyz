use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::Display;

use for_test::CookieOptions;

/// Cookie ヘッダの値をパースする
#[allow(clippy::result_unit_err)]
pub fn parse(header_value: &str) -> Result<HashMap<String, String>, ()> {
    let mut map = HashMap::new();

    let values: Vec<&str> = header_value.split(';').collect();
    let values: Vec<&str> = values.iter().map(|str| str.trim()).collect();

    for cookie_value in values {
        let mut key = String::new();
        let mut value = String::new();
        let mut is_key = true;

        for c in cookie_value.chars() {
            if is_key && c == '=' {
                is_key = false;
                continue;
            }
            if is_key {
                key.push(c);
            } else {
                value.push(c);
            }
        }

        if key.is_empty() || value.is_empty() {
            return Err(());
        }

        map.insert(key, value);
    }

    Ok(map)
}

/// Set-Cookie ヘッダの値を生成する
pub fn build(key: &str, value: &str) -> CookieOptionBuilder {
    assert!(!key.is_empty());
    assert!(!value.is_empty());
    CookieOptionBuilder::initialize(key, value)
}

pub struct CookieOptionBuilder {
    key: String,
    value: String,
    opt: CookieOptions,
}

impl CookieOptionBuilder {
    fn initialize(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            opt: CookieOptions {
                max_age: None,
                domain: None,
                secure: true,
                http_only: true,
                same_site: SameSite::Lax,
            },
        }
    }

    pub fn build(&self) -> String {
        let mut cookie = String::new();

        cookie.push_str(&format!("{}={}", self.key, self.value));

        if let Some(max_age) = self.opt.max_age {
            cookie.push_str(&format!("; MaxAge={max_age}"));
        }
        if let Some(domain) = self.opt.domain.clone() {
            cookie.push_str(&format!("; Domain={domain}"));
        }
        if self.opt.secure {
            cookie.push_str("; Secure");
        }
        if self.opt.http_only {
            cookie.push_str("; HttpOnly");
        }
        cookie.push_str(&format!("; SameSite={}", self.opt.same_site));

        cookie.push_str("; Path=/");

        cookie
    }

    pub fn max_age(&mut self, v: u32) -> &mut Self {
        self.opt.borrow_mut().max_age = Some(v);
        self
    }

    pub fn domain(&mut self, v: &str) -> &mut Self {
        self.opt.borrow_mut().domain = Some(v.to_string());
        self
    }

    pub fn secure(&mut self, v: bool) -> &mut Self {
        self.opt.borrow_mut().secure = v;
        self
    }

    pub fn http_only(&mut self, v: bool) -> &mut Self {
        self.opt.borrow_mut().http_only = v;
        self
    }

    pub fn same_site(&mut self, v: SameSite) -> &mut Self {
        self.opt.borrow_mut().same_site = v;
        self
    }
}

#[derive(Debug, PartialEq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Display for SameSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Strict => "Strict",
            Self::Lax => "Lax",
            Self::None => "None",
        };
        write!(f, "{str}")
    }
}

impl From<&str> for SameSite {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "strict" => Self::Strict,
            "lax" => Self::Lax,
            "none" => Self::None,
            _ => Self::Lax,
        }
    }
}

pub mod for_test {
    use super::{parse, SameSite};

    pub struct CookieOptions {
        pub max_age: Option<u32>,
        pub domain: Option<String>,
        pub secure: bool,
        pub http_only: bool,
        pub same_site: SameSite,
    }

    /// Set-Cookie の値をパースして (key, value) を返す
    #[allow(clippy::result_unit_err)]
    pub fn parse_set_cookie(header_value: &str) -> Result<(String, String, CookieOptions), ()> {
        let directives: Vec<&str> = header_value.split(';').map(|str| str.trim()).collect();
        if directives.is_empty() {
            return Err(());
        }

        let cookie = parse(directives.first().unwrap());
        if cookie.is_err() {
            return Err(());
        }
        let cookie = cookie.unwrap();
        if cookie.len() != 1 {
            return Err(());
        }

        let header = cookie.keys().next().unwrap().to_string();
        let value = cookie.get(&header).unwrap().to_string();

        let directives: Vec<&str> = directives[1..].into();

        let mut opt = CookieOptions {
            max_age: None,
            domain: None,
            secure: false,
            http_only: false,
            same_site: SameSite::Lax,
        };

        for directive in directives {
            let key_value: Vec<&str> = directive.split('=').collect();
            if key_value.len() > 2 {
                return Err(());
            }
            let key = key_value.first().cloned().unwrap();
            let value = key_value.get(1).cloned();

            match key.to_lowercase().as_str() {
                "maxage" => {
                    if value.is_none() {
                        return Err(());
                    }
                    let value = value.unwrap().parse::<u32>();
                    if value.is_err() {
                        return Err(());
                    }
                    opt.max_age = Some(value.unwrap());
                }
                "domain" => {
                    if value.is_none() {
                        return Err(());
                    }
                    opt.domain = Some(value.unwrap().to_string());
                }
                "secure" => {
                    opt.secure = true;
                }
                "httponly" => {
                    opt.http_only = true;
                }
                "samesite" => {
                    if value.is_none() {
                        return Err(());
                    }
                    let value: SameSite = value.unwrap().into();
                    opt.same_site = value;
                }
                _ => {}
            }
        }

        Ok((header, value, opt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single() {
        let result = parse("key=value").unwrap();
        assert_eq!(result.get("key").unwrap(), "value");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn parse_multiple() {
        let result = parse("key1=value1; key2=value2").unwrap();
        assert_eq!(result.get("key1").unwrap(), "value1");
        assert_eq!(result.get("key2").unwrap(), "value2");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn basic_build() {
        let cookie = build("key", "value").build();
        assert_eq!(cookie, "key=value; Secure; HttpOnly; SameSite=Lax; Path=/");
    }

    #[test]
    fn basic_full() {
        let cookie = build("key", "value")
            .domain("example.com")
            .max_age(100)
            .same_site(SameSite::Lax)
            .build();
        assert_eq!(
            cookie,
            "key=value; MaxAge=100; Domain=example.com; Secure; HttpOnly; SameSite=Lax; Path=/"
        );
    }

    #[test]
    fn basic_minimum() {
        let cookie = build("key", "value")
            .secure(false)
            .http_only(false)
            .same_site(SameSite::None)
            .build();
        assert_eq!(cookie, "key=value; SameSite=None; Path=/");
    }

    #[test]
    fn basic_set_cookie() {
        let result =
            for_test::parse_set_cookie("key=value; Secure; HttpOnly; SameSite=Lax; Path=/")
                .unwrap();
        assert_eq!(result.0, "key");
        assert_eq!(result.1, "value");
        assert_eq!(result.2.domain, None);
        assert_eq!(result.2.max_age, None);
        assert_eq!(result.2.http_only, true);
        assert_eq!(result.2.secure, true);
        assert_eq!(result.2.same_site, SameSite::Lax);
    }

    #[test]
    fn minimum_set_cookie() {
        let result = for_test::parse_set_cookie("key=value").unwrap();
        assert_eq!(result.0, "key");
        assert_eq!(result.1, "value");
        assert_eq!(result.2.domain, None);
        assert_eq!(result.2.max_age, None);
        assert_eq!(result.2.http_only, false);
        assert_eq!(result.2.secure, false);
        assert_eq!(result.2.same_site, SameSite::Lax);
    }

    #[test]
    fn full_set_cookie() {
        let result = for_test::parse_set_cookie(
            "key=value; Secure; HttpOnly; SameSite=Strict; Domain=example.com; MaxAge=100; Path=/",
        )
        .unwrap();
        assert_eq!(result.0, "key");
        assert_eq!(result.1, "value");
        assert_eq!(result.2.domain, Some("example.com".to_string()));
        assert_eq!(result.2.max_age, Some(100));
        assert_eq!(result.2.http_only, true);
        assert_eq!(result.2.secure, true);
        assert_eq!(result.2.same_site, SameSite::Strict);
    }
}
