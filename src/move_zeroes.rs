pub fn move_zeroes(values: &mut[i32]) {
  let n = values.len();
  let mut new_values: Vec<i32> = vec![];

  for value in values.iter() {
    if *value != 0 {
      new_values.push(*value);
    }
  }

  values[0..new_values.len()].copy_from_slice(&new_values);

  let rest = vec![0; n - new_values.len()];
  values[new_values.len()..].copy_from_slice(&rest);
}