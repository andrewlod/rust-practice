pub fn greater_str<'a>(strings: &'a [&str]) -> &'a str {
  strings.iter().max_by(|&a, &b| a.len().cmp(&b.len())).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_greater_str_begin() {
    assert_eq!(greater_str(&["foo", "ba", ""]), "foo");
  }

  #[test]
  fn test_greater_str_end() {
    assert_eq!(greater_str(&["foo", "ba", "hello world"]), "hello world");
  }
}