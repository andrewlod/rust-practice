pub fn compress_string(s: &str) -> String {
  let mut compressed = String::new();

  let mut chars = s.chars();
  let mut current_char = chars.next().unwrap();
  let mut count = 1;

  for c in chars {
    if c == current_char {
      count += 1;
      continue;
    }
    
    compressed.push(current_char);

    if count > 1 {
      compressed.push_str(count.to_string().as_str());
      count = 1;
    }
    
    current_char = c;
  }
    
  compressed.push(current_char);

  if count > 1 {
    compressed.push_str(count.to_string().as_str());
  }

  compressed
}