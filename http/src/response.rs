use std::collections::HashMap;

pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    /// Set-Cookie values
    pub cookies: Vec<String>,
    pub body: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            cookies: vec![],
            body: None,
        }
    }
}
