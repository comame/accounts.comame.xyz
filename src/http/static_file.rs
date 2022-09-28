use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

fn internal_read(path: &str) -> Result<Vec<u8>> {
    dbg!(&path);

    let f = File::open(path);

    if let Err(err) = f {
        dbg!(&err);
        return Err(err);
    }

    let mut reader = BufReader::new(f.unwrap());
    let mut buf: Vec<u8> = vec![];

    if let Err(err) = reader.read_to_end(&mut buf) {
        dbg!(&err);
        return Err(err);
    }

    Ok(buf)
}

pub fn read(path: &str) -> Result<Vec<u8>> {
    let relative_path = format!("static{}", &path);

    let buf = internal_read(relative_path.as_str());

    if buf.is_ok() {
        buf
    } else {
        // index.html でリトライ
        let path_with_index_html = format!("{}/index.html", relative_path);
        internal_read(path_with_index_html.as_str())
    }
}
