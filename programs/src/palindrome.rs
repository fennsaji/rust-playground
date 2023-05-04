pub fn palindrome(text: &str) -> bool {
    let rev_text = text.chars().rev().collect::<String>();
    text == rev_text
}
