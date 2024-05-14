use std::fmt::Write;

use crate::point::Point;

struct Rectangle {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32
}

impl Rectangle {
  pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
    Rectangle { x, y, width, height }
  }

  pub fn area(&self) -> f32 {
    self.width * self.height
  }

  pub fn perimeter(&self) -> f32 {
    2f32 * (self.width + self.height)
  }

  pub fn contains_point(&self, point: &Point) -> bool {
    point.x >= self.x && point.x <= (self.x + self.width) && point.y >= self.y && point.y <= (self.y + self.height)
  }

  pub fn print<W: Write>(&self, writer: &mut W) {
    writer.write_str(format!("({}, {}, {}, {})\n", self.x, self.y, self.width, self.height).as_str()).unwrap();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_area() {
    let rect = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    assert_eq!(rect.area(), 100.0);
  }

  #[test]
  fn test_perimeter() {
    let rect = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    assert_eq!(rect.perimeter(), 40.0);
  }

  #[test]
  fn test_point_inside_rect() {
    let rect = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    let point = Point::new(5.0, 5.0);

    assert!(rect.contains_point(&point));
  }

  #[test]
  fn test_point_outside_rect() {
    let rect = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    let point = Point::new(15.0, 5.0);

    assert!(!rect.contains_point(&point));
  }

  #[test]
  fn test_print() {
    let mut output = String::new();
    let rect = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    rect.print(&mut output);

    assert_eq!(output, "(0, 0, 10, 10)\n");
  }

}