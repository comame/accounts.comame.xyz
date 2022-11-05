use mysql::params;
use mysql::prelude::*;

use super::mysql::{get_conn, mysqldate_to_unixtime};
use crate::data::mail::Mail;

pub fn insert_mail(mail: &Mail) {
    get_conn()
        .unwrap()
        .exec_drop(
            "INSERT INTO mails VALUES (:sub, :to, :body, :at)",
            params! {
                "sub" => mail.subject.to_string(),
                "to" => mail.to.to_string(),
                "body" => mail.body.to_string(),
                "at" => mail.created_at
            },
        )
        .unwrap();
}

pub fn list_new_mail() -> Vec<Mail> {
    get_conn()
        .unwrap()
        .query_map(
            "SELECT * FROM mails ORDER BY created_at LIMIT 100",
            |(subject, to, body, created_at)| Mail {
                subject,
                to,
                body,
                created_at: mysqldate_to_unixtime(created_at),
            },
        )
        .unwrap()
}
