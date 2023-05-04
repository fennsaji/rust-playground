use std::io::{self, Write};

mod count_chars;
mod palindrome;

fn main() {
    let mut text = String::new();

    print!("Enter some text: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text).unwrap();
    let text = text.trim();
    println!("You entered: {}", text);

    let freq = count_chars::count(&text);
    print!("Character Count: {:?} \n", freq);

    let is_palindrome = palindrome::palindrome(&text);
    println!("Is {} Palindrome? {}", text, is_palindrome);
}
