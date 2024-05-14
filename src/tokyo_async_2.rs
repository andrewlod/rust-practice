use std::time::Duration;

use tokio::{time::sleep, try_join};

async fn do_task(id: u32, duration: u64) -> Result<(), &'static str> {
  println!("Starting task {}", &id);
  if duration % 2 == 0 {
    return Err("Duration must be odd");
  }
  sleep(Duration::from_secs(duration)).await;
  println!("Finished task {} after {} seconds", &id, &duration);

  Ok(())
}

pub async fn demo() {
  let task1 = do_task(1, 1);
  let task2 = do_task(2, 2);
  let task3 = do_task(3, 3);

  match try_join!(task1) {
    Ok(results) => println!("{:?}", results),
    Err(msg) => println!("{:?}", msg)
  }

  match try_join!(task2) {
    Ok(results) => println!("{:?}", results),
    Err(msg) => println!("{:?}", msg)
  }

  match try_join!(task3) {
    Ok(results) => println!("{:?}", results),
    Err(msg) => println!("{:?}", msg)
  }
}