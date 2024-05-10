pub fn rotate_array(arr: &mut [i32], k: usize) {
  if arr.is_empty() {
    return;
  }

  let k = k % arr.len();

  arr.reverse();
  arr[..k].reverse();
  arr[k..].reverse();
}