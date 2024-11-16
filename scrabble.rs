use std::collections::HashMap;

fn scrabble_score(word: &str) -> usize {
    let mut letter_values = HashMap::new();
    letter_values.insert('A', 1);
    letter_values.insert('B', 3);
    letter_values.insert('C', 3);
    letter_values.insert('D', 2);
    letter_values.insert('E', 1);
    letter_values.insert('F', 4);
    letter_values.insert('G', 2);
    letter_values.insert('H', 4);
    letter_values.insert('I', 1);
    letter_values.insert('J', 8);
    letter_values.insert('K', 5);
    letter_values.insert('L', 1);
    letter_values.insert('M', 3);
    letter_values.insert('N', 1);
    letter_values.insert('O', 1);
    letter_values.insert('P', 3);
    letter_values.insert('Q', 10);
    letter_values.insert('R', 1);
    letter_values.insert('S', 1);
    letter_values.insert('T', 1);
    letter_values.insert('U', 1);
    letter_values.insert('V', 4);
    letter_values.insert('W', 4);
    letter_values.insert('X', 8);
    letter_values.insert('Y', 4);
    letter_values.insert('Z', 10);

    word.to_uppercase().chars().map(|c| letter_values.get(&c).unwrap_or(&0)).sum()
}

fn main() {
    let word = "cabbage";
    let score = scrabble_score(word);
    println!("The Scrabble score for '{}' is: {}", word, score);
}