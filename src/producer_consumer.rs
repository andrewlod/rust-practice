use std::{sync::mpsc::channel, thread, time::Duration};

use rand::Rng;

pub fn demo() {
  let (sender, receiver) = channel();

  let producer_handler = thread::spawn(move|| {
    let mut rng = rand::thread_rng();
    loop {
        thread::sleep(Duration::from_secs(1));
        let num = rng.gen_range(1..11);
        sender.send(num).unwrap();

        if num == 10 {
          break;
        }
    }
  });

  let consumer_handler = thread::spawn(move|| {
    loop {
        let num = receiver.recv().unwrap();
        println!("{}", num * num);

        if num == 10 {
          break;
        }
    }
  });

  producer_handler.join().unwrap();
  consumer_handler.join().unwrap();

  println!("Done!");
}