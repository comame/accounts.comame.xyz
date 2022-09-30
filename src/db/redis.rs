use std::sync::Mutex;

use once_cell::sync::OnceCell;
use redis::{Client, Commands, Connection};

static CLIENT: OnceCell<Mutex<Client>> = OnceCell::new();
static PREFIX: &'static str = "id.comame.dev:";

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

pub fn set(key: &str, value: &str, time_sec: u16) {
    let mut conn = get_conn().unwrap();
    let _r: Result<(), _> = conn.set_ex(String::from(PREFIX) + key, value, time_sec as usize);
}

pub fn get(key: &str) -> Option<String> {
    let mut conn = get_conn().unwrap();
    conn.get::<String, Option<String>>(String::from(PREFIX) + key)
        .unwrap()
}

pub fn list_keys() -> Vec<String> {
    let mut conn = get_conn().unwrap();
    let keys: Vec<String> = conn.keys(String::from(PREFIX) + "*").unwrap();
    keys.iter()
        .map(|key| String::from(&key[PREFIX.len()..key.len()]))
        .collect()
}

pub fn list_keys_pattern(pattern: &str) -> Vec<String> {
    let mut conn = get_conn().unwrap();
    let keys: Vec<String> = conn.keys(String::from(PREFIX) + pattern).unwrap();
    keys.iter()
        .map(|key| String::from(&key[PREFIX.len()..key.len()]))
        .collect()
}

pub fn del(key: &str) {
    let mut conn = get_conn().unwrap();
    let _r: Result<(), _> = conn.del(String::from(PREFIX) + key);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        super::init("redis://redis.comame.dev");
    }

    const EX_TIME: u16 = 3;

    #[test]
    fn test_set_and_get() {
        init();
        set("set_and_get_1", "foo", EX_TIME);
        assert_eq!(get("set_and_get_1").unwrap(), "foo");
        assert_eq!(get("set_and_get_2"), None);
    }

    #[test]
    fn test_list_all() {
        init();
        set("list_foo", "foo", EX_TIME);
        set("list_bar", "bar", EX_TIME);
        let result = list_keys();
        assert!(
            result.contains(&"list_foo".to_string()) && result.contains(&"list_bar".to_string())
        );
    }

    #[test]
    fn test_list_pattern() {
        init();
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
        init();
        set("del_1", "foo", EX_TIME);
        set("del_2", "foo", EX_TIME);
        del("del_2");
        assert_eq!(list_keys_pattern("del_*"), vec!("del_1"));
    }
}
