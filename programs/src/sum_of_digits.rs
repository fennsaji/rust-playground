pub fn sum(n: u32) -> u32 {
    n.to_string().chars().map(|d| d.to_digit(10).unwrap()).sum()
}