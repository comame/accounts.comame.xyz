use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::crypto::sha;

pub struct ValueWithEtag {
    pub value: String,
    pub etag: String,
}

fn read_file(path: &str) -> std::io::Result<String> {
    let f = File::open(path)?;

    let mut reader = BufReader::new(f);
    let mut buf: Vec<u8> = vec![];

    reader.read_to_end(&mut buf)?;

    let result = String::from_utf8(buf);

    if let Err(err) = result {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            err.to_string(),
        ))
    } else {
        Ok(result.unwrap())
    }
}

pub fn read(path: &str) -> std::io::Result<String> {
    let relative_path = format!("static{}", &path);

    let buf = read_file(relative_path.as_str());

    if buf.is_ok() {
        buf
    } else {
        // index.html でリトライ
        let path_with_index_html = format!("{}/index.html", relative_path);
        read_file(path_with_index_html.as_str())
    }
}

pub fn read_with_etag(path: &str) -> std::io::Result<ValueWithEtag> {
    let value = read(path)?;
    Ok(ValueWithEtag {
        value: value.clone(),
        etag: sha::sha256(&value),
    })
}
