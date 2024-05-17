trait Animal {
  fn make_sound(&self) -> String;
}

struct Dog {}
struct Cat {}

impl Animal for Dog {
  fn make_sound(&self) -> String {
    String::from("Woof!")
  }
}

impl Animal for Cat {
  fn make_sound(&self) -> String {
    String::from("Meow!")
  }
}

#[cfg(test)]

mod tests {
  use super::*;

  #[test]
  fn test_dog() {
    let dog = Dog {};
    assert_eq!(dog.make_sound(), "Woof!");
  }

  #[test]
  fn test_cat() {
    let cat = Cat {};
    assert_eq!(cat.make_sound(), "Meow!");
  }
}