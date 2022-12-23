use serde::de::{self, Deserialize, Visitor};
use std::fmt;

use crate::db::authentication::{find_latest_authentication_by_user, insert_authentication};
use crate::time::now;

#[derive(Clone, Debug)]
pub struct Authentication {
    pub authenticated_at: u64,
    pub created_at: u64,
    pub audience: String,
    pub subject: String,
    pub user_agent_id: String,
    pub method: AuthenticationMethod,
}

impl Authentication {
    pub fn create(
        authenticated_at: u64,
        audience: &str,
        subject: &str,
        method: AuthenticationMethod,
        user_agent_id: &str,
    ) -> Self {
        let instance = Self {
            authenticated_at,
            created_at: now(),
            audience: audience.to_string(),
            subject: subject.to_string(),
            user_agent_id: user_agent_id.to_string(),
            method,
        };

        insert_authentication(&instance);

        instance
    }

    pub fn latest(user_id: &str, user_agent_id: &str) -> Option<Self> {
        find_latest_authentication_by_user(user_id, user_agent_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthenticationMethod {
    Password,
    Google,
    Session,
    Consent,
}

impl fmt::Display for AuthenticationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Password => "password".to_string(),
                Self::Google => "google".to_string(),
                Self::Session => "session".to_string(),
                Self::Consent => "consent".to_string(),
            }
        )
    }
}

impl AuthenticationMethod {
    pub fn parse(str: &str) -> Result<Self, ()> {
        match str {
            "password" => Ok(Self::Password),
            "google" => Ok(Self::Google),
            "session" => Ok(Self::Session),
            "consent" => Ok(Self::Consent),
            _ => Err(()),
        }
    }
}

struct AuthenticationMethodVisitor;

impl<'de> Visitor<'de> for AuthenticationMethodVisitor {
    type Value = AuthenticationMethod;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("see AuthenticationMethod::parse")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let result = AuthenticationMethod::parse(v);
        match result {
            Ok(value) => Ok(value),
            Err(_) => Err(E::custom("Invalid format.")),
        }
    }
}

impl<'de> Deserialize<'de> for AuthenticationMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(AuthenticationMethodVisitor)
    }
}

#[derive(Clone, Debug)]
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

impl LoginPrompt {
    pub fn parse(str: &str) -> Result<Self, ()> {
        let str = str.to_lowercase();
        if str == "none" {
            Ok(LoginPrompt::None)
        } else if str == "login" {
            Ok(LoginPrompt::Login)
        } else if str == "consent" {
            Ok(LoginPrompt::Consent)
        } else if str == "select_account" {
            Ok(LoginPrompt::SelectAccount)
        } else {
            Err(())
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
        let _auth = Authentication::create(
            now(),
            "audience.comame.dev",
            "Bob",
            AuthenticationMethod::Password,
            "ua",
        );
    }
}
