fn get_combinations(arr: &[i32], combination: &Vec<i32>, combinations: &mut Vec<Vec<i32>>, i: usize) {
  if i >= arr.len() {
    return;
  }

  for j in i..arr.len() {
    let mut new_combination = combination.clone();
    new_combination.push(arr[j]);
    get_combinations(arr, &new_combination, combinations, j + 1);
    combinations.push(new_combination);
  }
}

pub fn array_combinations(arr: &[i32]) -> Vec<Vec<i32>> {
  let mut combinations: Vec<Vec<i32>> = Vec::new();
  combinations.push(vec![]);
  let combination: Vec<i32> = Vec::new();
  get_combinations(arr, &combination, &mut combinations, 0);

  combinations
}