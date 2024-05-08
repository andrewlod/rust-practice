use std::collections::HashSet;

pub fn has_unique_characters(s: &str) -> bool {
  let mut existing_characters: HashSet<char> = HashSet::new();

  for c in s.chars() {
    if existing_characters.contains(&c) {
      return false;
    } else {
      existing_characters.insert(c);
    }
  }

  true
}