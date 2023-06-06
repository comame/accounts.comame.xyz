use std::env;
use std::sync::{Mutex, OnceLock};

use redis::{Client, Commands, Connection};

static CLIENT: OnceLock<Mutex<Client>> = OnceLock::new();

fn get_prefix() -> String {
    let redis_prefix = env::var("REDIS_PREFIX").unwrap();
    format!("{redis_prefix}:")
}

pub fn init(redis_url: &str) {
    CLIENT.get_or_init(|| {
        let client = Client::open(redis_url).unwrap();
        Mutex::new(client)
    });
}

fn get_conn() -> Result<Connection, ()> {
    let mutex = CLIENT.get();
    if mutex.is_none() {
        return Err(());
    }

    let guard = mutex.unwrap().lock();
    if guard.is_err() {
        return Err(());
    }

    let client = guard.unwrap().get_connection();
    if client.is_err() {
        return Err(());
    }

    Ok(client.unwrap())
}

pub fn set(key: &str, value: &str, time_sec: u64) {
    let mut conn = get_conn().unwrap();
    let _r: Result<(), _> = conn.set_ex(get_prefix() + key, value, time_sec as usize);
}

pub fn get(key: &str) -> Option<String> {
    let mut conn = get_conn().unwrap();
    conn.get::<String, Option<String>>(get_prefix() + key)
        .unwrap()
}

pub fn list_keys_pattern(pattern: &str) -> Vec<String> {
    let mut conn = get_conn().unwrap();
    let keys: Vec<String> = conn.keys(get_prefix() + pattern).unwrap();
    keys.iter()
        .map(|key| String::from(&key[get_prefix().len()..key.len()]))
        .collect()
}

pub fn del(key: &str) {
    let mut conn = get_conn().unwrap();
    let _r: Result<(), _> = conn.del(get_prefix() + key);
}

#[cfg(test)]
mod tests {
    use super::super::_test_init::init_redis;
    use super::*;

    const EX_TIME: u64 = 10;

    #[test]
    fn test_set_and_get() {
        init_redis();
        set("set_and_get_1", "foo", EX_TIME);
        assert_eq!(get("set_and_get_1").unwrap(), "foo");
        assert_eq!(get("set_and_get_2"), None);
    }

    #[test]
    fn test_list_pattern() {
        init_redis();
        set("list_pattern_abc", "foo", EX_TIME);
        assert_eq!(
            list_keys_pattern("list_pattern_a*"),
            vec!("list_pattern_abc")
        );
        assert_eq!(
            list_keys_pattern("list_pattern_abc"),
            vec!("list_pattern_abc")
        );
        assert_eq!(list_keys_pattern("list_pattern_xyz"), vec!() as Vec<&str>);
    }

    #[test]
    fn test_del() {
        init_redis();
        set("del_1", "foo", EX_TIME);
        set("del_2", "foo", EX_TIME);
        del("del_2");
        assert_eq!(list_keys_pattern("del_*"), vec!("del_1"));
    }
}
