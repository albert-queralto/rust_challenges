fn main() {
    // Example encoded number
    let encoded_number = 89; // You can change this to any other number for testing

    // Convert the number to binary and count the number of 1 bits
    let mut count = 0;
    let mut number = encoded_number;

    while number > 0 {
        if number & 1 == 1 {
            count += 1;
        }
        number >>= 1;
    }

    println!("Counting binary 1s: {}", count);
}