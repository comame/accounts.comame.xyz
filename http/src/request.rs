use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Request {
    pub origin: Option<String>,
    pub method: Method,
    pub path: String,
    pub query: Option<String>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(path: &str, body: Option<&str>) -> Self {
        Self {
            origin: None,
            method: Method::Get,
            path: path.to_string(),
            query: None,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            body: body.map(|s| s.to_string()),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
