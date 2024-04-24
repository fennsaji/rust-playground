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
    if let Some(closing_bracket) = possible_brackets.get(&c) {
      stack.push(*closing_bracket);
    } else {
      let popped_item = stack.pop();
      if popped_item.is_none() {
        return false;
      }
      if popped_item.unwrap() != c {
        return false;
      }
    }
  }
  stack.len() == 0
}
