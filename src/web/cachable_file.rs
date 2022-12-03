use std::collections::HashMap;
use std::sync::Mutex;

use super::static_file::read;
use crate::crypto::sha;

static CACHE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

pub struct ValueWithEtag {
    pub value: String,
    pub etag: String,
}

pub enum CacheResult {
    Value(ValueWithEtag),
    Etag(String),
}

pub fn read_with_etag(path: &str) -> Option<CacheResult> {
    let mut cache_map = CACHE.lock().unwrap();
    let cache_map = cache_map.get_or_insert(HashMap::new());

    let value = read(path);
    if value.is_err() {
        return None;
    }

    match cache_map.get(path).cloned() {
        Some(etag) => Some(CacheResult::Etag(etag)),
        None => {
            let value = String::from_utf8(value.unwrap()).unwrap();
            let etag = sha::sha256(&value);
            cache_map.insert(path.to_string(), etag.clone());
            Some(CacheResult::Value(ValueWithEtag { value, etag }))
        }
    }
}
