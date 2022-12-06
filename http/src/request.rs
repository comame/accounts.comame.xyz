use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone)]
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

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Get => "Get",
            Self::Post => "Post",
        };
        write!(f, "{str}")
    }
}
