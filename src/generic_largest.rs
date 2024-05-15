pub fn largest<T>(values: &[T]) -> Option<&T> where T: Ord {
  values.iter().max()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_int() {
    let numbers = vec![34, 50, 25, 100, 65];
    assert_eq!(largest(&numbers), Some(&100));
  }

  #[test]
  fn test_char() {
    let chars = vec!['y','m','a','q'];
    assert_eq!(largest(&chars), Some(&'y'));
  }

  #[test]
  fn test_str() {
    let values = vec!["hello", "world", "foobar", "rusty!!!!"];
    assert_eq!(largest(&values), Some(&"world"));
  }
}