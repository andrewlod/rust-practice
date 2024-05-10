extern crate rand;
use std::io::stdin;

use rand::Rng;

pub fn guessing_game() {
  let mut rng = rand::thread_rng();

  let mut buf = String::new();

  println!("Your choice (1-100): ");
  let input = stdin();
  input.read_line(&mut buf).unwrap();

  let mut num: i32 = match buf.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Invalid input");
      return;
    }
  };

  while num != rng.gen_range(1..101) {
    buf.clear();
    println!("Wrong! Try again: (1-100): ");
    let input = stdin();
    input.read_line(&mut buf).unwrap();

    num = match buf.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid input");
        return;
      }
    };
  }

  println!("Correct!")
  
}

const WORDS: &[&str] = &["headline", "opponent", "pneumonia", "define", "mist", "majority", "camera", "awful", "city", "dressing"];

pub fn word_guessing_game() {
  let mut rng = rand::thread_rng();
  let word = WORDS[rng.gen_range(0..WORDS.len())];
  let mut word_build: String = word.chars().map(|_| '_' ).collect();

  while word_build != word {
    let mut buf = String::new();
    println!("Type a letter: ");
    stdin().read_line(&mut buf).expect("Invalid input!");

    let letter = buf.trim().chars().next();
    if letter.is_none() {
      println!("Invalid input!");
      continue;
    }

    let letter = letter.unwrap();
    word_build = word_build.char_indices().map(|(idx, c)| {
      if word.chars().nth(idx).unwrap() == letter {
        letter
      } else {
        c
      }
    }).collect();

    println!("{}", &word_build);
  }
  
  println!("You won!");
}