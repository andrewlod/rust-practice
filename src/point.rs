use std::fmt::Write;

pub struct Point {
  pub x: f32,
  pub y: f32
}

impl Point {
  pub fn new(x: f32, y: f32) -> Point {
    Point { x, y }
  }

  pub fn distance(&self, other: &Point) -> f32 {
    ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
  }

  pub fn move_to(&mut self, x: f32, y: f32) {
    self.x = x;
    self.y = y;
  }

  pub fn print<W: Write>(&self, writer: &mut W) {
    writer.write_str(format!("({}, {})\n", self.x, self.y).as_str()).unwrap();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_distance() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);

    assert_eq!(p1.distance(&p2), 5.0);
  }

  #[test]
  fn test_move_to() {
    let mut p1 = Point::new(0.0, 0.0);

    p1.move_to(3.0, 4.0);

    assert_eq!(p1.x, 3.0);
    assert_eq!(p1.y, 4.0);
  }

  #[test]
  fn test_print() {
    let mut output = String::new();
    let p1 = Point::new(1.2, 3.4);
    p1.print(&mut output);

    assert_eq!(output, "(1.2, 3.4)\n");
  }
}