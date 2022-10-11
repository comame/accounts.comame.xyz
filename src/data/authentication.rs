use std::fmt;

use crate::db::authentication::insert_authentication;
use crate::time::now;

pub struct Authentication {
    pub authenticated_at: u64,
    pub audience: String,
    pub subject: String,
    pub method: AuthenticationMethod,
    pub prompt: LoginPrompt,
}

impl Authentication {
    pub fn new(
        audience: &str,
        subject: &str,
        method: AuthenticationMethod,
        prompt: LoginPrompt,
    ) -> Self {
        let instance = Self {
            authenticated_at: now(),
            audience: audience.to_string(),
            subject: subject.to_string(),
            method,
            prompt,
        };

        insert_authentication(&instance);

        instance
    }
}

pub enum AuthenticationMethod {
    None,
    Password,
    Google,
}

impl fmt::Display for AuthenticationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none".to_string(),
                Self::Password => "password".to_string(),
                Self::Google => "google".to_string(),
            }
        )
    }
}

pub enum LoginPrompt {
    None,
    Login,
    Consent,
    SelectAccount,
}

impl fmt::Display for LoginPrompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => String::from("none"),
                Self::Login => String::from("login"),
                Self::Consent => String::from("consent"),
                Self::SelectAccount => String::from("select_account"),
            }
        )
    }
}

impl From<String> for LoginPrompt {
    fn from(str: String) -> Self {
        let str = str.to_lowercase();
        if str == "none" {
            LoginPrompt::None
        } else if str == "login" {
            LoginPrompt::Login
        } else if str == "consent" {
            LoginPrompt::Consent
        } else if str == "select_account" {
            LoginPrompt::SelectAccount
        } else {
            panic!("invalid format `{str}` for LoginPrompt");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::_test_init::init_mysql;

    #[test]
    fn can_insert() {
        init_mysql();
        let _auth = Authentication::new(
            "audience.comame.dev",
            "Bob",
            AuthenticationMethod::Password,
            LoginPrompt::Login,
        );
    }
}
