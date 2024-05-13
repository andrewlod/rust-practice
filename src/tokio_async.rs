use std::time::Duration;
use tokio::time::sleep;

pub async fn do_something(time: u64) {
  println!("Starting some task!");
  sleep(Duration::from_secs(time)).await;
  println!("Finished some task!");
}