use std::sync::Mutex;
use mysql::{PooledConn, Pool};
use once_cell::sync::OnceCell;

static POOL: OnceCell<Mutex<Pool>> = OnceCell::new();

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
