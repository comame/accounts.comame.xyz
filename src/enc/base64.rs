use openssl::base64::{decode_block, encode_block};

pub fn encode_base64(src: &[u8]) -> String {
    encode_block(src)
}

pub fn encode_base64_url(src: &[u8]) -> String {
    encode_base64(src)
        .replace('+', "-")
        .replace('/', "_")
        .replace('=', "")
}

pub fn decode_base64(src: &str) -> Result<Vec<u8>, ()> {
    let result = decode_block(src);
    if result.is_err() {
        return Err(());
    }
    Ok(result.unwrap())
}
