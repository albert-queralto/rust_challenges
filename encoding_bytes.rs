// https://medium.com/rustaceans/rust-challenge-variable-length-quantity-7dcd052fad3b

fn encode_vlq(mut n: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let mut byte = (n & 0x7F) as u8;
        n >>= 7;
        if n != 0 {
            byte |= 0x80;
        }
        bytes.push(byte);
        if n == 0 {
            break;
        }
    }
    bytes.reverse();
    bytes
}

fn decode_vlq(bytes: &[u8]) -> Result<u32, &'static str> {
    let mut n = 0u32;
    for &byte in bytes {
        n = (n << 7) | (byte & 0x7F) as u32;
        if byte & 0x80 == 0 {
            return Ok(n);
        }
    }
    Err("Incomplete VLQ sequence")
}

fn main() {
    // Test encoding
    let numbers = vec![0, 64, 127, 128, 8192, 16383, 16384, 1048576, 2097151, 2097152, 134217728, 268435455];
    for &number in &numbers {
        let encoded = encode_vlq(number);
        println!("{:08X} -> {:?}", number, encoded);
    }

    // Test decoding
    let encoded_numbers = vec![
        vec![0x00],
        vec![0x40],
        vec![0x7F],
        vec![0x81, 0x00],
        vec![0xC0, 0x00],
        vec![0xFF, 0x7F],
        vec![0x81, 0x80, 0x00],
        vec![0xC0, 0x80, 0x00],
        vec![0xFF, 0xFF, 0x7F],
        vec![0x81, 0x80, 0x80, 0x00],
        vec![0xC0, 0x80, 0x80, 0x00],
        vec![0xFF, 0xFF, 0xFF, 0x7F],
    ];
    for encoded in encoded_numbers {
        match decode_vlq(&encoded) {
            Ok(number) => println!("{:?} -> {:08X}", encoded, number),
            Err(e) => println!("{:?} -> Error: {}", encoded, e),
        }
    }
}