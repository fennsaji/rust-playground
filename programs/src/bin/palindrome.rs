pub fn palindrome(text: &str) -> bool {
    let rev_text = text.chars().rev().collect::<String>();
    text == rev_text
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false; // Negative numbers are not palindromes
    }

    let mut d = x;
    let mut r = 0;
    while d != 0 {
        let m = d % 10;
        r = r * 10 + m;
        d = d / 10;
    }
    x == r
}