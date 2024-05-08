pub fn multiplication_table(num: u8) {
  for i in 1..11 {
    println!("{} x {} = {}", num, i, num * i);
  }
}