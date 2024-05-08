pub fn gcd(num1: u32, num2: u32) -> u32 {
  let mut a: u32;
  let mut b: u32;

  if num1 < num2 {
    a = num2;
    b = num1;
  } else {
    a = num1;
    b = num2;
  }

  while b > 0 {
    let temp = b;
    b = a % b;
    a = temp;
  }

  a
}