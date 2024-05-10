use std::{fs::File, io::{Read, Write}};

pub fn file_manip() {
  let mut file = File::options()
    .read(true)
    .write(true)
    .open("./assets/data.txt").unwrap();

  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  println!("{}", &content);

  file.write_all(b"more content...").unwrap();
}

pub fn count_lines(path: &str) -> usize {
  let mut file = File::open(path).unwrap();

  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();

  content.lines().count()
}