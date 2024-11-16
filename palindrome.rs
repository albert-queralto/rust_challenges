fn is_palindrome(num: u32) -> bool {
    let s = num.to_string();
    s == s.chars().rev().collect::<String>()
}

fn palindrome_products(min: u32, max: u32) -> (u32, Vec<(u32, u32)>, u32, Vec<(u32, u32)>) {
    let mut smallest_palindrome = None;
    let mut largest_palindrome = None;
    let mut smallest_factors = Vec::new();
    let mut largest_factors = Vec::new();

    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if is_palindrome(product) {
                match smallest_palindrome {
                    None => {
                        smallest_palindrome = Some(product);
                        smallest_factors.push((i, j));
                    }
                    Some(sp) if product < sp => {
                        smallest_palindrome = Some(product);
                        smallest_factors.clear();
                        smallest_factors.push((i, j));
                    }
                    Some(sp) if product == sp => {
                        smallest_factors.push((i, j));
                    }
                    _ => {}
                }

                match largest_palindrome {
                    None => {
                        largest_palindrome = Some(product);
                        largest_factors.push((i, j));
                    }
                    Some(lp) if product > lp => {
                        largest_palindrome = Some(product);
                        largest_factors.clear();
                        largest_factors.push((i, j));
                    }
                    Some(lp) if product == lp => {
                        largest_factors.push((i, j));
                    }
                    _ => {}
                }
            }
        }
    }

    (
        smallest_palindrome.unwrap(),
        smallest_factors,
        largest_palindrome.unwrap(),
        largest_factors,
    )
}

fn main() {
    let (min, max) = (10, 99);
    let (smallest_palindrome, smallest_factors, largest_palindrome, largest_factors) = palindrome_products(min, max);

    println!("Smallest palindrome product: {}", smallest_palindrome);
    for (i, j) in smallest_factors {
        println!("Factors: ({}, {})", i, j);
    }

    println!("Largest palindrome product: {}", largest_palindrome);
    for (i, j) in largest_factors {
        println!("Factors: ({}, {})", i, j);
    }
}