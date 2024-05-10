use std::time::{SystemTime, UNIX_EPOCH};

struct Block {
  index: u64,
  timestamp: u64,
  data: String,
  hash: String,
  prev_hash: String
}

impl Block {
  pub fn new(index: u64, timestamp: u64, data: &str, hash: &str, prev_hash: &str) -> Block {
    Block {
      index,
      timestamp,
      data: data.to_string(),
      hash: hash.to_string(),
      prev_hash: prev_hash.to_string()
    }
  }

  pub fn data_size(&self) -> usize {
    self.data.len()
  }

  pub fn creation_time(&self) -> u64 {
    self.timestamp / 1000
  }
}

pub fn demo() {
  let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
  let block = Block::new(0, time.as_millis() as u64, "Hello, world!", "12345", "");

  println!("Data size: {}", block.data_size());
  println!("Creation time: {}", block.creation_time());
}