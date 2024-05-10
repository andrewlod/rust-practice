use regex::Regex;

pub fn is_palindrome(s: &str) -> bool {
  let re = Regex::new(r"[[ !?.,]]").unwrap();
  let s = re.replace_all(s, "").to_lowercase();
  s == s.chars().rev().collect::<String>()
}