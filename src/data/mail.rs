use std::env::var;

use serde::{Deserialize, Serialize};

use crate::db::mail::{insert_mail, list_new_mail};
use crate::mail::send_mail;
use crate::time::now;

#[derive(Serialize, Deserialize)]
pub struct Mail {
    pub subject: String,
    pub to: String,
    pub body: String,
    pub created_at: u64,
}

#[allow(dead_code)]
impl Mail {
    pub fn new(to: &str, subject: &str, body: &str) -> Self {
        Self {
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
            created_at: now(),
        }
    }

    pub fn list() -> Vec<Self> {
        list_new_mail()
    }

    pub fn send(&self) -> Result<(), ()> {
        // MAIL 環境変数が存在するか、リリースビルド
        let can_send = var("MAIL").is_ok() || cfg!(not(debug_assertions));

        insert_mail(self);

        if can_send {
            send_mail(&self.subject, &self.to, &self.body)
        } else {
            Ok(())
        }
    }
}
