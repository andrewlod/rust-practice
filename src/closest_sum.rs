pub fn closest_sum(nums: &[i32], target: i32) -> i32 {
  if nums.len() < 3 {
    return nums.iter().sum();
  }

  let mut min_diff = i32::MAX;
  let mut closest = 0;
  let mut current_window = 0;
  let mut i = 0;
  let mut j = 0;

  while j < nums.len() {
    if j - i < 3 {
      current_window += nums[j];
      j += 1;
      continue;
    }

    let diff = (current_window - target).abs();
    if diff < min_diff {
      min_diff = diff;
      closest = current_window;
    }

    current_window -= nums[i];
    current_window += nums[j];
    i += 1;
    j += 1;
  }

  if (current_window - target).abs() < (closest - target).abs() {
    closest = current_window;
  }

  closest
}