use openssl::base64::encode_block;

pub fn encode_base64(src: &[u8]) -> String {
    encode_block(src)
}

pub fn encode_base64_url(src: &[u8]) -> String {
    encode_base64(src)
        .replace('+', "-")
        .replace('/', "_")
        .replace('=', "")
}
