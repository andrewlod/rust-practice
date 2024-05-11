use rand::Rng;

fn place_random_ship(board: &mut [[char; 10]; 10]) -> usize {
  let mut rng = rand::thread_rng();

  loop {
    let x = rng.gen_range(0..10);
    let y = rng.gen_range(0..10);
    let direction = rng.gen_range(0..4);
    let size = rng.gen_range(1..4);
    let mut positions: Vec<(usize, usize)> = Vec::new();

    match direction {
      // Right
      0 => {
        if x + size >= 10 {
          continue;
        }
        for i in 0..size {
          positions.push((x + i, y));
        }
      },
      // Left
      1 => {
        if (x as i32 - size as i32) < 0 {
          continue;
        }
        for i in 0..size {
          positions.push((x - i, y));
        }
      },
      // Down
      2 => {
        if y + size >= 10 {
          continue;
        }
        for i in 0..size {
          positions.push((x, y + i));
        }
      },
      // Up
      3 => {
        if (y as i32 - size as i32) < 0 {
          continue;
        }
        for i in 0..size {
          positions.push((x, y - i));
        }
      },
      _ => continue
    }
    
    for coord in &positions {
      if board[coord.0][coord.1] == 'S' {
        continue;
      }
    }
    
    for coord in &positions {
      board[coord.0][coord.1] = 'S';
    }

    return size;
  }
}

fn erase_board(board: &mut [[char; 10]; 10]) {
  for row in board {
    for val in row {
      *val = '_';
    }
  }
}

fn read_input() -> (usize, usize) {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("Failed to read line");

  let coords: Vec<&str> = input.trim().split(' ').collect();
  let x = coords[0].parse::<usize>().unwrap();
  let y = coords[1].parse::<usize>().unwrap();

  (x, y)
}

pub fn main() {
  let mut board: [[char; 10]; 10] = [['_'; 10]; 10];

  let mut hits_remaining = 0;
  for _ in 0..5 {
    hits_remaining += place_random_ship(&mut board);
  }

  for row in &board {
    println!("{:?}", row);
  }

  let mut shots = 25;

  while shots > 0 && hits_remaining > 0 {
    println!("Shots left: {}", &shots);
    println!("Hits remaining: {}", &hits_remaining);

    let coords = read_input();

    if board[coords.0][coords.1] == 'S' {
      hits_remaining -= 1;
      println!("A ship was hit!");
    } else {
      println!("Miss!");
    }

    shots -= 1;
  }

  if hits_remaining == 0 {
    println!("You win!");
  } else {
    println!("Not this time!");
  }
}