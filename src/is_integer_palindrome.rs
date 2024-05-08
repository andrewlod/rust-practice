pub fn is_integer_palindrome(value: i32) -> bool {
  if value < 0 {
    return false;
  }
  let mut remainder = value;

  while remainder > 0 {
    let last_digit = remainder % 10;
    let power =  (10i32).pow((remainder as f32).log10().floor() as u32);
    let first_digit = remainder / power;

    if last_digit != first_digit {
      return false;
    }

    remainder %= power;
    remainder /= 10;
  }

  true
}