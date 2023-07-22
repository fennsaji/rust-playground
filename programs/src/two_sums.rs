pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, n) in nums.iter().enumerate() {
        let sum = 0;
        for (j, m) in nums.iter().enumerate() {
            if i == j {
                continue;
            }
            if n+m == target {
                return vec![i as i32, j as i32]
            }
        }
    }
    vec![]
}
