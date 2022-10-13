use openssl::rand::rand_bytes;

use crate::enc::base64;

pub fn random_str(len: usize) -> String {
    let mut buf = vec![];
    let bytes_len = len * 3 / 4 + 1;
    for _i in 0..bytes_len {
        buf.push(0);
    }
    rand_bytes(&mut buf).unwrap();
    let mut answer = base64::encode_base64(buf);
    answer = answer.replace("+", "-");
    answer = answer.replace("/", "_");

    let diff = answer.len() - len;
    if diff > 0 {
        answer = answer[0..answer.len() - diff].into();
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_len() {
        let mut buf = vec![];
        let len = 30;
        for _i in 0..len {
            buf.push(0);
        }
        rand_bytes(&mut buf).unwrap();

        for size in buf.iter() {
            let text = random_str(*size as usize);
            assert_eq!(text.len(), *size as usize);
        }
    }

    #[test]
    fn test_zero_length() {
        let text = random_str(0);
        assert_eq!(text, "");
    }

    #[test]
    fn test_big_length() {
        let size = 2i32.pow(16).try_into().expect("err");
        let text = random_str(size);
        assert_eq!(size, text.len());
    }
}
