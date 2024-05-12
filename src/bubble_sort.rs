use std::{fs::File, io::Read};

pub fn read_file_int_vec(path: &str) -> Vec<i32> {
  let mut file = File::open(path).unwrap();

  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();

  content.split_whitespace().map(|line| line.parse::<i32>().unwrap()).collect()
}

pub fn bubble_sort() -> Vec<i32> {
  let mut values = read_file_int_vec("./assets/bubble_sort.txt");

  for i in 0..values.len() - 1 {
    for j in i+1..values.len() {
      if values[i] > values[j] {
        values.swap(i, j);
      }
    }
  }

  values
}