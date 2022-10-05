use super::mysql::get_conn;
use mysql::prelude::*;
use mysql::Params;
use std::env;
use std::sync::Once;

static INIT_MYSQL: Once = Once::new();
static INIT_REDIS: Once = Once::new();

#[allow(dead_code)]
pub fn init_mysql() {
    let tables = vec!["users", "user_passwords"];

    let mysql_user = env::var("MYSQL_USER").unwrap();
    let mysql_password = env::var("MYSQL_PASSWORD").unwrap();
    let mysql_db = env::var("MYSQL_DATABASE").unwrap();
    super::mysql::init(&format!(
        "mysql://{}:{}@mysql.comame.dev/{}",
        mysql_user, mysql_password, mysql_db
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
    super::redis::init("redis://redis.comame.dev");

    INIT_REDIS.call_once(|| {
        let keys = super::redis::list_keys();
        for key in keys {
            super::redis::del(&key);
        }
    })
}
