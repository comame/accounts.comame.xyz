use std::fmt::Display;

use super::authentication::AuthenticationMethod;
use crate::db::authentication_failure::{count_recent_log, insert_fail};
use crate::time::now;

pub struct AuthenticationFailure {
    pub tried_at: u64,
    pub subject_input: String,
    pub method: AuthenticationMethod,
    pub reason: AuthenticationFailureReason,
    pub remote_addr: String,
}

impl AuthenticationFailure {
    pub fn new(
        subject_input: &str,
        method: &AuthenticationMethod,
        reason: &AuthenticationFailureReason,
        remote_addr: &str,
    ) -> Self {
        let obj = Self {
            tried_at: now(),
            subject_input: subject_input.to_string(),
            method: method.clone(),
            reason: reason.clone(),
            remote_addr: remote_addr.to_string(),
        };

        insert_fail(&obj);

        obj
    }

    pub fn is_too_many(user_id: &str) -> bool {
        let recent = count_recent_log(user_id);
        recent > 5
    }
}

#[derive(Clone)]
pub enum AuthenticationFailureReason {
    UserNotFound,
    InvalidPassword,
}

impl Display for AuthenticationFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UserNotFound => "user_not_found",
                Self::InvalidPassword => "invalid_password",
            }
        )
    }
}

impl From<String> for AuthenticationFailureReason {
    fn from(str: String) -> Self {
        match str.as_str() {
            "user_not_found" => AuthenticationFailureReason::UserNotFound,
            "invalid_password" => AuthenticationFailureReason::InvalidPassword,
            _ => panic!(),
        }
    }
}
