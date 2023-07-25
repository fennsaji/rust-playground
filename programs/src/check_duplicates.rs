pub fn contains_duplicate(nums: Vec<i32>) -> bool {
  for (i, n) in nums.iter().enumerate() {
      if let Some(_) = nums.iter().skip(i+1).find(|&x| x==n) {
          return true;
      }
  }
  false
}