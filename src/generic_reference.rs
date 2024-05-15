pub struct Pair<'a, T> {
  first: &'a T,
  second: &'a T
}

impl<'a, T> Pair<'a, T> {
  pub fn new(first: &'a T, second: &'a T) -> Self {
    Pair {
      first,
      second
    }
  }

  pub fn get_first(&self) -> &'a T {
    self.first
  }

  pub fn get_second(&self) -> &'a T {
    self.second
  }
}