use std::collections::HashSet;

fn try_position(board: &Vec<Vec<char>>, word: &str, i: usize, j: usize, pos: usize, m: usize, n: usize, visited: &mut HashSet<(usize, usize)>) -> bool {
  let mut word_chars = word.chars();
  if pos >= word.len() - 1 {
    return true;
  }

  visited.insert((i, j));
  let pos = pos + 1;
  let current_char = word_chars.nth(pos).unwrap();

  if i > 0 && board[i-1][j] == current_char && !visited.contains(&(i - 1, j)) && try_position(board, word, i - 1, j, pos, m, n, visited) {
    return true;
  }

  if i < n-1 && board[i+1][j] == current_char && !visited.contains(&(i + 1, j)) && try_position(board, word, i + 1, j, pos, m, n, visited) {
    return true;
  }

  if j > 0 && board[i][j-1] == current_char && !visited.contains(&(i, j - 1)) && try_position(board, word, i, j - 1, pos, m, n, visited) {
    return true;
  }

  if j < m-1 && board[i][j+1] == current_char && !visited.contains(&(i, j + 1)) && try_position(board, word, i, j + 1, pos, m, n, visited) {
    return true;
  }

  false
}

pub fn word_exists_in_board(board: &Vec<Vec<char>>, word: &str) -> bool {
  if word.is_empty() {
    return true;
  }

  let n = board.len();
  let m = board[0].len();

  let mut visited: HashSet<(usize, usize)> = HashSet::new();
  let first_char = word.chars().next().unwrap();

  for i in 0..n {
    for j in 0..m {
      if board[i][j] == first_char && try_position(board, word, i, j, 0, m, n, &mut visited) {
        return true;
      }
    }
  }

  false
}