use std::fmt;

use super::authentication::AuthenticationMethod;
use crate::db::authentication_failure::insert_authentication_failure;
use crate::time::now;

pub struct AuthenticationFailure {
    pub tried_at: u64,
    pub audience: String,
    pub subject: String,
    pub method: AuthenticationMethod,
    pub reason: Reason,
}

impl AuthenticationFailure {
    pub fn create(
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
    InvalidSessionCookie,
    EmptySessionCookie,
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidPassword => "invalid password".to_string(),
                Self::UserNotFound => "user not found".to_string(),
                Self::InvalidSessionCookie => "invalid session cookie".to_string(),
                Self::EmptySessionCookie => "empty sessoin cookie".to_string(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        crate::db::_test_init::init_mysql();
        AuthenticationFailure::create(
            "evil.comame.dev",
            "Alice",
            AuthenticationMethod::Password,
            Reason::InvalidPassword,
        );
    }
}
