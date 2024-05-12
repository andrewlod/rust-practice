pub fn linear_search(arr: &[i32], target: i32) -> i32 {
  for (i, val) in arr.iter().enumerate() {
    if *val == target {
      return i as i32;
    }
  }

  -1
}