use std::collections::HashMap;

pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: Option<String>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Method {
    Get,
    Post,
}
