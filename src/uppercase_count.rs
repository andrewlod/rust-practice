pub fn uppercase_count(s: &str) -> usize {
  s.chars().fold(0, |acc, c| {
    if c.is_uppercase() {
      acc + 1
    } else {
      acc
    }
  })
}