struct MyResult<T, E>(Result<T, E>);

impl<T, E> MyResult<T, E>
where
  T: Copy
{
  fn unwrap_or_else<F: FnOnce() -> T>(&self, func: F) -> T {
    match self {
      MyResult(Ok(value)) => *value,
      MyResult(Err(_)) => func()
    }
  }
}

#[cfg(test)]

mod tests {
  use std::error::Error;

use super::*;

  #[test]
  fn test_unwrap_or_else_some() {
    let result: MyResult<i32, Box<dyn Error>> = MyResult(Ok(1));
    assert_eq!(result.unwrap_or_else(|| 2), 1);
  }

  #[test]
  fn test_unwrap_or_else_none() {
    let result = MyResult(Err(()));
    assert_eq!(result.unwrap_or_else(|| 2), 2);
  }
}