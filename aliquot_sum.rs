fn aliquot_sum(n: u32) -> u32 {
    (1..n).filter(|&x| n % x == 0).sum()
}

fn classify_number(n: u32) -> &'static str {
    let sum = aliquot_sum(n);
    if sum == n {
        "Perfect"
    } else if sum > n {
        "Abundant"
    } else {
        "Deficient"
    }
}

fn main() {
    let numbers = vec![6, 28, 12, 24, 8, 15];
    for &number in &numbers {
        println!("{} is {}", number, classify_number(number));
    }
}