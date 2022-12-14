fn to_hex(half_byte: u8) -> char {
    if half_byte >= 16 {
        panic!();
    }

    let charcode_of_a = b'A';
    let charcode_of_zero = b'0';

    if half_byte < 10 {
        (charcode_of_zero + half_byte) as char
    } else {
        (charcode_of_a + half_byte - 10) as char
    }
}

fn to_byte(c: char) -> u8 {
    let charcode_of_a = b'A';
    let charcode_of_zero = b'0';

    if ('0'..='9').contains(&c) {
        c as u8 - charcode_of_zero
    } else if ('A'..='F').contains(&c) {
        c as u8 - charcode_of_a + 10
    } else {
        panic!();
    }
}

fn hex_to_u8_map() -> Vec<Vec<u8>> {
    let mut v: Vec<Vec<u8>> = vec![];
    for i in 0..=255 {
        if i % 16 == 0 {
            let vv: Vec<u8> = vec![i];
            v.push(vv);
        } else {
            let index_first: u8 = i >> 4;
            let vv = &mut v[index_first as usize];
            vv.push(i);
        }
    }
    v
}

fn u8_to_hex_map() -> [String; 256] {
    let mut vec: Vec<String> = vec![];
    for i in 0..=255 {
        let mut str = String::new();
        str.push(to_hex(i >> 4));
        str.push(to_hex(i & 0x0f));
        vec.push(str);
    }
    vec.try_into().unwrap()
}

#[allow(dead_code)]
pub fn encode(bytes: Vec<u8>) -> String {
    let mut str = String::new();
    for byte in bytes {
        str.push_str(u8_to_hex_map()[byte as usize].as_str())
    }
    str
}

pub fn decode(hex: &str) -> Vec<u8> {
    let hex = hex.to_ascii_uppercase();

    let mut arr: Vec<u8> = vec![];
    let mut chars: [usize; 2] = [0, 0];

    for (i, char) in hex.chars().into_iter().enumerate() {
        if i % 2 == 0 {
            chars[0] = to_byte(char) as usize;
        } else {
            chars[1] = to_byte(char) as usize;
            let byte = hex_to_u8_map()[chars[0]][chars[1]];
            arr.push(byte);
        }
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    type Testcase = (Vec<u8>, &'static str);

    fn testcases() -> Vec<Testcase> {
        vec![
            (vec![], ""),
            (vec![0], "00"),
            (vec![1], "01"),
            (vec![255], "FF"),
            (vec![1, 0], "0100"),
            (vec![255, 255], "FFFF"),
            (
                vec![
                    105, 196, 224, 216, 106, 123, 4, 48, 216, 205, 183, 128, 112, 180, 197, 90,
                ],
                "69C4E0D86A7B0430D8CDB78070B4C55A",
            ),
        ]
    }

    #[test]
    fn test_encode() {
        for testcase in testcases() {
            assert_eq!(encode(testcase.0), testcase.1);
        }
    }

    #[test]
    fn test_encode_case() {
        let testcases = vec![(vec![255], "ff"), (vec![255], "fF")];
        for testcase in testcases {
            assert_eq!(encode(testcase.0), testcase.1.to_uppercase());
        }
    }

    #[test]
    fn test_decode() {
        for testcase in testcases() {
            assert_eq!(decode(testcase.1), testcase.0);
        }
    }
}
