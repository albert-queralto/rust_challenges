fn encode(input: &str) -> String {
    let mut encoded = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let mut count = 1;
        while let Some(&next) = chars.peek() {
            if next == c {
                count += 1;
                chars.next();
            } else {
                break;
            }
        }
        if count > 1 {
            encoded.push_str(&count.to_string());
        }
        encoded.push(c);
    }

    encoded
}

fn decode(input: &str) -> String {
    let mut decoded = String::new();
    let mut count = String::new();

    for c in input.chars() {
        if c.is_digit(10) {
            count.push(c);
        } else {
            let repeat = count.parse::<usize>().unwrap_or(1);
            for _ in 0..repeat {
                decoded.push(c);
            }
            count.clear();
        }
    }

    decoded
}

fn main() {
    let encoded = encode("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB");
    println!("Encoded: {}", encoded);

    let decoded = decode("12WB12W3B24WB");
    println!("Decoded: {}", decoded);
}