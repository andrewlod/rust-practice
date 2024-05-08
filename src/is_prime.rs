pub fn is_prime(num: u32) -> bool {
  if num % 2 == 0 || num < 2 {
    return false;
  }

  let mut i: u32 = 3;
  while i * i <= num {
    if num % i == 0 {
      return false;
    }
    i += 2;
  }

  true
}