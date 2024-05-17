pub trait OptionExt<T> {
  fn unwrap_or_else<F: FnOnce() -> T>(&self, func: F) -> T;
  fn map<F: FnOnce(Option<&T>) -> T>(&self, func: F) -> Option<T>;
}

impl<T> OptionExt<T> for Option<T>
where
  T: Copy
{
  fn unwrap_or_else<F: FnOnce() -> T>(&self, func: F) -> T {
    match self {
      Some(value) => *value,
      None => func()
    }
  }

  fn map<F: FnOnce(Option<&T>) -> T>(&self, func: F) -> Option<T> {
    match self {
      Some(value) => Some(func(Some(value))),
      None => None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_unwrap_or_else_some() {
    let optional_number = Some(42);
    assert_eq!(optional_number.unwrap_or_else(|| 1), 42);
  }

  #[test]
  fn test_unwrap_or_else_none() {
    let optional_number = None;
    assert_eq!(optional_number.unwrap_or_else(|| 1), 1);
  }

  #[test]
  fn test_map_some() {
    let optional_number = Some(42);
    assert_eq!(optional_number.map(|x| x + 1), Some(43));
  }

  #[test]
  fn test_map_none() {
    let optional_number: Option<i32> = None;
    assert_eq!(optional_number.map(|x| x + 1), None);
  }
}