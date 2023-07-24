use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let possible_brackets: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('{', '}'),
        ('[', ']')
    ]);
    let mut stack: Vec<char> = vec![];
    let s_chars = s.chars();
    if s.len() % 2 != 0 {
        return false;
    }
    for c in s_chars {
        if possible_brackets.contains_key(&c) {
            stack.push(c);
        } else {
            let popped_item = stack.pop();
            if popped_item.is_none() {
                return false;
            }
            let opposite_bracket = possible_brackets.get(&popped_item.unwrap());
            if *opposite_bracket.unwrap() != c {
                return false;
            }
        }
    }
    stack.len() == 0
}
