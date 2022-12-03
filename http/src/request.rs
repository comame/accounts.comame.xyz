use std::collections::HashMap;

pub struct Request {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: Option<String>,
}
