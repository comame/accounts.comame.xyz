use super::mysql::{get_conn, init as init_mysql};
use mysql::prelude::*;
use mysql::Params;
use std::env;

#[allow(dead_code)]
pub fn init() {
    let tables = vec!["users", "user_passwords"];

    let mysql_user = env::var("MYSQL_USER").unwrap();
    let mysql_password = env::var("MYSQL_PASSWORD").unwrap();
    let mysql_db = env::var("MYSQL_DATABASE").unwrap();
    init_mysql(
        format!(
            "mysql://{}:{}@mysql.comame.dev/{}",
            mysql_user, mysql_password, mysql_db
        )
        .as_str(),
    );

    if mysql_db != "id_dev" {
        panic!("Not in development environment");
    }

    for table in tables {
        let result = get_conn()
            .unwrap()
            .exec_drop(format!("DELETE FROM {}", table), Params::Empty);
        dbg!(table);
        dbg!(&result);
        if let Err(err) = result {
            panic!("{}", err);
        }
    }
}
