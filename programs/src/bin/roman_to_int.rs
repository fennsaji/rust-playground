pub fn roman_to_int(s: String) -> i32 {
  let mut value = 0;
  let s_chars: Vec<char> = s.chars().collect();

  for (i, c) in s_chars.iter().enumerate() {
      match c {
          'I' => {
              if i+1 < s_chars.len() && (s_chars[i+1] == 'V' || s_chars[i+1] == 'X') {
                  value-=1;
              } else {
                  value+=1;
              } 
          },
          'V' => value+=5,
          'X' => {
              if  i+1 < s_chars.len() && (s_chars[i+1] == 'L' || s_chars[i+1] == 'C') {
                  value-=10;
              } else {
                  value+=10;
              } 
          },
          'L' => value+=50,
          'C' => {
              if  i+1 < s_chars.len() && (s_chars[i+1] == 'D' || s_chars[i+1] == 'M') {
                  value-=100;
              } else {
                  value+=100;
              } 
          },
          'D' => value+=500,
          'M' => value+=1000,
          _ => (),
      }
  }
  value
}