// Memory optimised
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
  for (i, n) in nums.iter().enumerate() {
      if let Some(_) = nums.iter().skip(i+1).find(|&x| x==n) {
          return true;
      }
  }
  false
}

// Time Optimised
pub fn contains_duplicate_to(nums: Vec<i32>) -> bool {
  let mut seen_numbers = HashSet::new();

  for num in nums {
      if !seen_numbers.insert(num) {
          return true;
      }
  }

  false
}