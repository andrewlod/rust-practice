use std::collections::HashSet;

pub fn contains_duplicates(nums: &[i32]) -> bool {
  let mut found: HashSet<i32> = HashSet::new();

  for num in nums {
    if found.contains(num) {
      return true;
    }
    found.insert(*num);
  }

  false
}