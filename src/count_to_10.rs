pub fn count_for(num: u8) {
  for i in 1..num+1 {
    println!("{}", &i);
  }
}

pub fn count_while(num: u8) {
  let mut i = 1;
  while i <= num {
    println!("{}", &i);
    i += 1;
  }
}