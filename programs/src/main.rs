// use std::io::{self, Write};


mod count_chars;
mod palindrome;
mod fibonacci;
mod sum_of_digits;
mod binary_seach;
mod sock_merchant;

use sock_merchant::find_pairs;

fn main() {
    // let text = String::new();
    // print!("Enter some text: ");
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut text).unwrap();
    // let text = text.trim();
    // println!("You entered: {}", text);

    let text = "TEET".to_owned();

    let freq = count_chars::count(&text);
    print!("Character Count: {:?} \n", freq);

    let is_palindrome = palindrome::palindrome(&text);
    println!("Is {} Palindrome? {}", text, is_palindrome);

    let number = 20;
    let fib_series = fibonacci::fibonacci(number);
    println!("{:?}", fib_series);

    let number = 2023;
    let sum = sum_of_digits::sum(number);
    println!("{}", sum);

    let arr = [0, 2, 4, 6, 8, 10, 13, 15, 18, 20];
    let target = 20;
    let index = binary_seach::search(&arr, target);
    println!("Element Found at {:?}", index);

    let arr = [1, 2, 1, 0, 2, 3, 4, 2, 1, 2];
    let pairs = find_pairs(&arr);
    println!("Sock Pairs Found: {}", pairs);
}
