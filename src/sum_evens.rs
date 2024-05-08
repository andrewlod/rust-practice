pub fn sum_evens(nums: &[i32]) -> i32 {
  nums.iter().fold(0, |acc, val|  {
    if val % 2 == 0 {
      acc + val
    } else {
      acc
    }
  })
}