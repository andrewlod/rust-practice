use std::{sync::mpsc::{channel, Sender}, thread};

use rand::Rng;

pub fn demo() {
  let (sender, receiver) = channel();
  let handlers = (0..5).map(|_| {
    let sender_clone = Sender::clone(&sender);
    thread::spawn(move || {
      let mut rng = rand::thread_rng();
      let value = rng.gen_range(1..101);
      sender_clone.send(value * value).unwrap();
    })
  });

  for handler in handlers {
    handler.join().unwrap();
  }

  let mut sum = 0;
  for _ in 0..5 {
    let num = receiver.recv().unwrap();
    println!("Num: {}", &num);
    sum += num;
  }
  println!("Sum: {}", sum);
}