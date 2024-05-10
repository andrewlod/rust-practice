extern crate rand;
use rand::Rng;

pub fn generate_random_numbers() {
  let mut rng = rand::thread_rng();
  for _ in 0..10 {
    println!("{}", rng.gen_range(0..101))
  }
}