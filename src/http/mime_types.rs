use std::collections::HashMap;

pub fn extract_extension(path: &str) -> String {
    let dot_index = path.find('.');
    if dot_index.is_none() {
        return path.to_string();
    }

    let mut path = path.to_string();
    for _i in 0..dot_index.unwrap() {
        path.remove(0);
    }

    path.remove(0);

    path
}

pub fn get_mime_types(extension: &str) -> Option<String> {
    match extension {
        "html" => Some("text/html".to_string()),
        "svg" => Some("image/svg+xml".to_string()),
        _ => None
    }
}
