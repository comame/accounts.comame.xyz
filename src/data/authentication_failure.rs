use std::fmt;

use crate::{db::authentication_failure::insert_authentication_failure, time::now};

use super::authentication::AuthenticationMethod;

pub struct AuthenticationFailure {
    pub tried_at: u64,
    pub audience: String,
    pub subject: String,
    pub method: AuthenticationMethod,
    pub reason: Reason,
}

impl AuthenticationFailure {
    pub fn new(
        audience: &str,
        subject: &str,
        method: AuthenticationMethod,
        reason: Reason,
    ) -> Self {
        let instance = Self {
            tried_at: now(),
            audience: audience.to_string(),
            subject: subject.to_string(),
            method,
            reason,
        };

        insert_authentication_failure(&instance);

        instance
    }
}

pub enum Reason {
    InvalidPassword,
    UserNotFound,
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidPassword => "invalid password".to_string(),
                Self::UserNotFound => "user not found".to_string(),
            }
        )
    }
}
