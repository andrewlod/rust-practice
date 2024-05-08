pub fn avg(nums: &[f64]) -> f64 {
  nums.iter().fold(0f64, |acc, val| acc + val) / (nums.len() as f64)
}