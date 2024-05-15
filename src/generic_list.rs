struct GenericList<T> 
where
  T: PartialEq + Ord
{
  data: Vec<T>
}

impl<T> GenericList<T> 
where
  T: PartialEq + Ord
{
  pub fn new() -> GenericList<T> {
    GenericList {
      data: Vec::new()
    }
  }

  pub fn add(&mut self, item: T) {
    self.data.push(item);
  }

  pub fn remove(&mut self, item: T) {
    if let Some(idx) = self.find(item) {
      self.data.remove(idx);
    }
  }

  pub fn find(&self, item: T) -> Option<usize> {
    self.data.iter().position(|x| *x == item)
  }

  pub fn sort(&mut self) {
    self.data.sort();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    let mut list = GenericList::new();
    list.add(1);
    assert_eq!(list.data, vec![1]);
  }

  #[test]
  fn test_remove() {
    let mut list = GenericList::new();
    list.add(1);
    list.add(2);
    list.remove(1);
    assert_eq!(list.data, vec![2]);
  }

  #[test]
  fn test_find() {
    let mut list = GenericList::new();
    list.add(1);
    list.add(3);
    list.add(438);
    list.add(12);
    assert_eq!(list.find(438), Some(2));
  }

  #[test]
  fn test_not_found() {
    let mut list = GenericList::new();
    list.add(1);
    list.add(3);
    list.add(438);
    list.add(12);
    assert_eq!(list.find(999), None);
  }

  #[test]
  fn test_sort() {
    let mut list = GenericList::new();
    list.add(1);
    list.add(3);
    list.add(438);
    list.add(12);
    list.sort();
    assert_eq!(list.data, vec![1,3,12,438]);
  }
}