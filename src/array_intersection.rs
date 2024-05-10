use std::collections::HashMap;

pub fn array_intersection(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
  let mut result: Vec<i32> = Vec::new();
  let mut arr1_values: HashMap<i32, bool> = HashMap::new();

  for &val in arr1 {
    arr1_values.entry(val).or_insert(false);
  }

  for &val in arr2 {
    let do_not_add = arr1_values.entry(val).or_insert(true);
    if !*do_not_add {
      *do_not_add = true;
      result.push(val);
    }
  }

  result
}