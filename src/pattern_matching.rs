use std::any::{Any, TypeId};

pub fn match_input(input: &dyn Any) {
  match input.type_id() {
    id if id == TypeId::of::<i32>() => {
      if let Some(val) = input.downcast_ref::<i32>() {
        println!("{}", val * 2);
      }
    }
    id if id == TypeId::of::<String>() => {
      if let Some(val) = input.downcast_ref::<String>() {
        println!("{}", val.len());
      }
    }
    _ => println!("Type not supported.")
  }
}