use hyper::body::to_bytes;
use hyper::Body;

pub async fn parse_body(body: Body) -> Result<String, ()> {
    let bytes = to_bytes(body).await;
    if let Err(err) = bytes {
        eprintln!("{}", err);
        return Err(());
    }

    let vec = bytes.unwrap().iter().cloned().collect::<Vec<u8>>();

    let str = String::from_utf8(vec);
    if let Err(err) = str {
        eprintln!("{}", err);
        return Err(());
    }

    Ok(str.unwrap())
}
