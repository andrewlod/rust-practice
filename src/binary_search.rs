pub fn binary_search(arr: &[i32], target: i32) -> i32 {
  let mut cloned_array: Vec<(usize, &i32)> = arr.clone().iter().enumerate().collect();
  cloned_array.sort_by_key(|(_, &val)| val);

  let mut left = 0;
  let mut right = cloned_array.len() - 1;

  while left < right {
    let midpoint = (left + right) / 2;

    let current = *cloned_array[midpoint].1;

    if current == target {
      return cloned_array[midpoint].0 as i32;
    }

    if current > target {
      right = midpoint;
    } else {
      left = midpoint + 1;
    }
  }

  -1
}