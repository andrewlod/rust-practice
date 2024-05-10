pub fn buy_sell_stock(values: &[i32]) -> i32 {
  let mut max_profit = 0;
  let mut min_value = values[0];
  let mut max_value: i32;

  for i in 1..values.len() {
    if values[i] > values[i-1] {
      max_value = values[i];
      max_profit = max_profit.max(max_value - min_value);
    } else {
      min_value = min_value.min(values[i]);
    }
  }

  max_profit
}