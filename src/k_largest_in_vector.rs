pub fn k_largest_in_vector(nums: &mut [i32], k: u32) -> i32 {
  if k as usize >= nums.len() {
    return -1;
  }

  nums.sort();
  nums[nums.len() - k as usize]
}