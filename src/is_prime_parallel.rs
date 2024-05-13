use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn is_prime(num: u32) -> bool {
  if num % 2 == 0 || num < 2 {
    return false;
  }

  let mut i = 3;
  while i * i <= num {
    if num % i == 0 {
      return false;
    }

    i += 2;
  }

  true
}

pub fn is_prime_parallel(num: u32) -> bool {
  if num % 2 == 0 || num < 2 {
    return false;
  }

  let num_sqrt = (num as f32).sqrt().floor() as i32;
  let result = (3..num_sqrt).step_by(2).par_bridge().find_any(|&x| num % x as u32 == 0);

  result.is_none()
}