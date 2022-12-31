use http::enc::hex;
use openssl::sha::sha256 as openssl_sha256;

pub fn sha256(msg: &str) -> String {
    let buf = msg.as_bytes();
    let hashed = openssl_sha256(buf);
    hex::encode(hashed.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    type Testcase = (&'static str, &'static str);

    fn testcases() -> Vec<Testcase> {
        vec![
            (
                "",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                "abc",
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
        ]
    }

    #[test]
    fn test() {
        for case in testcases() {
            assert_eq!(sha256(case.0), case.1.to_ascii_uppercase());
        }
    }
}
