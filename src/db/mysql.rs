use std::sync::{Mutex, OnceLock};

use chrono::NaiveDate;
use mysql::{Pool, PooledConn};

use crate::time::datetime_to_unixtime;

static POOL: OnceLock<Mutex<Pool>> = OnceLock::new();

pub fn init(mysql_url: &str) {
    POOL.get_or_init(|| {
        let pool = Pool::new(mysql_url).unwrap();
        Mutex::new(pool)
    });
}

pub fn get_conn() -> Result<PooledConn, ()> {
    let mutex = POOL.get();
    if mutex.is_none() {
        return Err(());
    }

    let mutex = mutex.unwrap().lock();
    if mutex.is_err() {
        return Result::Err(());
    }

    let conn = mutex.unwrap().get_conn();
    if conn.is_err() {
        return Err(());
    }

    Ok(conn.unwrap())
}

pub fn mysqldate_to_unixtime(value: mysql::Value) -> u64 {
    match value {
        mysql::Value::Date(y, m, d, h, mins, s, _) => {
            NaiveDate::from_ymd(y as i32, m as u32, d as u32)
                .and_hms(h as u32, mins as u32, s as u32)
                .timestamp()
                .try_into()
                .unwrap()
        }
        mysql::Value::Bytes(bytes) => {
            let string = String::from_utf8(bytes).unwrap();
            datetime_to_unixtime(&string)
        }
        _ => {
            panic!();
        }
    }
}
