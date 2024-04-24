use std::collections::HashMap;

pub fn count(text: &str) -> HashMap<char, i32> {
    let mut freq = HashMap::new();
    for c in text.chars() {
        // *freq.entry(c).or_insert(0) += 1;
        match freq.get(&c) {
            Some(x) => {
                freq.insert(c, x+1);
            },
            None => {
                freq.insert(c, 1);
            }
        }
    }
    freq
}