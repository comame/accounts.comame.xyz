use http::request::Request;
use http::response::Response;

use crate::web::mime_types::{extract_extension, get_mime_types};
use crate::web::static_file;

pub fn handler(req: &Request) -> Response {
    let mut res = Response::new();

    let file = static_file::read_with_etag(&req.path);

    if file.is_err() {
        res.status = 404;
        res.body = Some("Not Found".to_string());
        return res;
    }
    let file = file.unwrap();

    let extension = extract_extension(&req.path);
    let content_type = get_mime_types(&extension);
    dbg!(&extension, &content_type);

    if let Some(content_type) = content_type {
        res.headers.insert("Content-Type".to_string(), content_type);
    }

    let previous_etag = req.headers.get("If-None-Match").cloned();

    // if-let を && で繋げるのはまだ unstable らしい
    if let Some(previous_etag) = previous_etag {
        if previous_etag == file.etag {
            res.status = 304; // NOT MODIFIED
            return res;
        }
    }

    res.headers.insert("Etag".to_string(), file.etag);
    res.body = Some(file.value);

    res
}
