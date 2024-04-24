pub fn longest_common_prefix(strs: Vec<String>) -> String {
  let mut prefix: Vec<char> = strs[0].chars().collect();
  for s in strs.iter().enumerate().skip(1) {
      let mut updated_prefix: Vec<char> = vec![];
      for (i, c) in s.1.char_indices() {
          if i < prefix.len() && c == prefix[i] {
              updated_prefix.push(c)
          } else {
              break;
          }
      }
      prefix = updated_prefix;
  }
  prefix.iter().collect()
}