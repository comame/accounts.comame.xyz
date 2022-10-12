use openssl::base64::encode_block;

pub fn encode_base64(src: Vec<u8>) -> String {
    encode_block(&src)
}
