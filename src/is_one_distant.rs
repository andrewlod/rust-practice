pub fn is_one_distant(str1: &str, str2: &str) -> bool {
  let n1 = str1.len();
  let n2 = str2.len();

  if (n1 as i32 - n2 as i32).abs() > 1 {
    return false;
  }

  let mut str1_ptr: usize = 0;
  let mut str2_ptr: usize = 0;
  let str1_chars: Vec<char> = str1.chars().collect();
  let str2_chars: Vec<char> = str2.chars().collect();
  let mut edits = 0;

  while str1_ptr < n1 && str2_ptr < n2 {
    if str1_chars[str1_ptr] == str2_chars[str2_ptr] {
      str1_ptr += 1;
      str2_ptr += 1;
    } else {
      edits += 1;

      if edits > 1 {
        return false;
      }

      match (n1 as i32) - (n2 as i32) {
        1 => {
          str1_ptr += 1;
        },
        -1 => {
          str2_ptr += 1;
        },
        _ => {
          str1_ptr += 1;
          str2_ptr += 1;
        }
      }
    }
  }

  true
}