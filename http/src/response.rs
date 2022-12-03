use std::collections::HashMap;

pub struct Response {
    pub headers: HashMap<String, String>,
    /// Set-Header values
    pub cookies: Vec<String>,
    pub body: Option<String>,
}
