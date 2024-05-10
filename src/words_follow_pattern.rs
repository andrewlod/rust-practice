use std::collections::HashMap;

pub fn words_follow_pattern(pattern: &str, words: &str) -> bool {
  let mut pattern_to_word: HashMap<char, &str> = HashMap::new();
  let mut word_to_pattern: HashMap<&str, char> = HashMap::new();

  let mut words = words.split(' ');
  let pattern = pattern.chars();

  for c in pattern {
    let word = words.next();
    if word.is_none() {
      return false;
    }

    let word = word.unwrap();

    let map_word = pattern_to_word.entry(c).or_insert(word);
    if *map_word != word {
      return false;
    }

    let map_pattern = word_to_pattern.entry(word).or_insert(c);
    if map_pattern != &c {
      return false;
    }
  }

  true
}