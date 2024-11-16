fn atbash_char(c: char) -> char {
    match c {
        'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
        'A'..='Z' => (b'Z' - (c as u8 - b'A')) as char,
        _ => c,
    }
}

fn atbash_cipher(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(atbash_char(c.to_ascii_lowercase()))
            } else {
                c
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let encoded = atbash_cipher("test");
    println!("Encoded: {}", encoded);

    let encoded = atbash_cipher("x123 yes");
    println!("Encoded: {}", encoded);

    let decoded = atbash_cipher("gvhg");
    println!("Decoded: {}", decoded);

    let decoded = atbash_cipher("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt");
    println!("Decoded: {}", decoded);
}