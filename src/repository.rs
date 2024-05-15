use std::ops::Add;

use num_traits::Num;

struct Repository<T> {
  data: Vec<T>
}

impl<T> Repository<T> {
  pub fn new() -> Repository<T> {
    Repository { data: Vec::new() }
  }

  pub fn add(&mut self, data: T) {
    self.data.push(data);
  }
}

impl Repository<i32> {
  pub fn sum(&self) -> i32 {
    self.data.iter().sum()
  }
}

impl Repository<f64> {
  pub fn product(&self) -> f64 {
    self.data.iter().product()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sum() {
    let mut repo = Repository::new();
    repo.add(1);
    repo.add(2);
    repo.add(3);
    repo.add(4);

    assert_eq!(repo.sum(), 10);
  }

  #[test]
  fn test_product() {
    let mut repo = Repository::new();
    repo.add(1.0);
    repo.add(2.0);
    repo.add(3.0);
    repo.add(4.0);

    assert_eq!(repo.product(), 24.0);
  }
}