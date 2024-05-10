use std::collections::HashMap;

pub fn is_constructible(ransom_note: &str, magazine: &str) -> bool {
  let mut ransom_note_counts: HashMap<char, i32> = HashMap::new();

  for c in ransom_note.chars() {
    let count = ransom_note_counts.entry(c).or_insert(0);
    *count += 1;
  }

  for c in magazine.chars() {
    if let Some(count) = ransom_note_counts.get_mut(&c) {
      *count -= 1;
    }
  }

  for item in ransom_note_counts.values() {
    if *item > 0 {
      return false;
    }
  }

  true
}