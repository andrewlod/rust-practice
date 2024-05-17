use std::{sync::Arc, thread};

pub fn parallel_sum(values: Arc<[i32]>, start: usize, end: usize, min_divide_size: usize) -> i32 {
  if end - start <= min_divide_size {
    return values[start..end].iter().sum();
  }

  let middle = (start + end) / 2;
  let clone_left = Arc::clone(&values);
  let clone_right = Arc::clone(&values);

  let left_sum = thread::spawn(move || parallel_sum(clone_left, start, middle, min_divide_size));
  let right_sum = thread::spawn(move || parallel_sum(clone_right, middle, end, min_divide_size));

  let left_sum = left_sum.join().unwrap();
  let right_sum = right_sum.join().unwrap();

  left_sum + right_sum
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parallel_sum() {
    let values: Arc<[i32]> = Arc::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(parallel_sum(Arc::clone(&values), 0, values.len(), 5), 55);
  }

  #[test]
  fn test_parallel_sum_large_array() {
    let values: Arc<[i32]> = Arc::new([5; 100]);
    assert_eq!(parallel_sum(Arc::clone(&values), 0, values.len(), 5), 500);
  }
}