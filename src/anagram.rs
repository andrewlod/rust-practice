use std::collections::HashMap;

pub fn anagram(str1: &str, str2: &str) -> bool {
  let mut counts1: HashMap<char, u32> = HashMap::new();

  for c in str1.chars() {
    let count = counts1.entry(c).or_insert(0);
    *count += 1;
  }

  for c in str2.chars() {
    let count = counts1.get_mut(&c);
    match count {
      Some(val) => *val -= 1,
      None => return false,
    }
  }

  for count in counts1.values() {
    if *count != 0 {
      return false;
    }
  }

  true
}