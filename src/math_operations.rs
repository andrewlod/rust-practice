enum Operation {
  Addition(i32, i32),
  Subtraction(i32, i32),
  Multiplication(i32, i32),
  Division(i32, i32)
}

fn calculate(op: &Operation) -> Result<i32, &str> {
  match op {
    Operation::Addition(a, b) => Ok(a + b),
    Operation::Subtraction(a, b) => Ok(a - b),
    Operation::Multiplication(a, b) => Ok(a * b),
    Operation::Division(a, b) =>  {
      if *b == 0 {
        Err("Division by zero is not allowed")
      } else {
        Ok(a / b)
      }
    }
  }
}

fn print_result(result: Result<i32, &str>) {
  match result {
    Ok(result) => println!("{:?}", result),
    Err(msg) => println!("{:?}", msg)
  }
}

pub fn demo() {
  let op1 = Operation::Addition(1, 2);
  let op2 = Operation::Subtraction(3, 4);
  let op3 = Operation::Multiplication(5, 6);
  let op4 = Operation::Division(8, 5);
  let op5 = Operation::Division(9, 0);

  print_result(calculate(&op1));
  print_result(calculate(&op2));
  print_result(calculate(&op3));
  print_result(calculate(&op4));
  print_result(calculate(&op5));
}