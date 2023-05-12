fn repeatedString(s: &str, n: i64) -> i64 {
    let count_a = s.chars().filter(|&c| c == 'a').count() as i64;
    let num_repeats = n / s.len() as i64;
    let num_extra_chars = n % s.len() as i64;
    let count_a_extra = s.chars().take(num_extra_chars as usize).filter(|&c| c == 'a').count() as i64;
    count_a * num_repeats + count_a_extra
}
