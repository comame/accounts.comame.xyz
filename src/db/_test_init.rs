use crate::crypto::rand::random_str;

use super::mysql::get_conn;
use mysql::prelude::*;
use mysql::Params;
use std::env;
use std::sync::Once;

static INIT_MYSQL: Once = Once::new();
static INIT_REDIS: Once = Once::new();

#[allow(dead_code)]
pub fn init_mysql() {
    let tables = vec![
        "users",
        "user_passwords",
        "sessions",
        "authentications",
        "authentication_failures",
    ];

    let mysql_user = env::var("MYSQL_USER").unwrap();
    let mysql_password = env::var("MYSQL_PASSWORD").unwrap();
    let mysql_db = env::var("MYSQL_DATABASE").unwrap();
    let mysql_host = env::var("MYSQL_HOST").unwrap();
    super::mysql::init(&format!(
        "mysql://{}:{}@{}/{}",
        mysql_user, mysql_password, mysql_host, mysql_db
    ));

    if mysql_db != "id_dev" {
        panic!("Not in development environment");
    }

    INIT_MYSQL.call_once(|| {
        for table in tables {
            get_conn()
                .unwrap()
                .exec_drop(format!("DELETE FROM {}", table), Params::Empty)
                .unwrap();
        }
    });
}

#[allow(dead_code)]
pub fn init_redis() {
    let redis_host = env::var("REDIS_HOST").unwrap();
    super::redis::init(&format!("redis://{}", redis_host));

    INIT_REDIS.call_once(|| {
        let redis_prefix = env::var("REDIS_PREFIX").unwrap();
        env::set_var("REDIS_PREFIX", format!("{redis_prefix}-{}", random_str(8)));
    });
}
