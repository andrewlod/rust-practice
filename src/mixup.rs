#[derive(Debug)]
pub struct Point<X1, Y1> 
where
  X1: Copy + PartialEq,
  Y1: Copy + PartialEq
{
  x: X1,
  y: Y1
}

impl<X1, Y1> Point<X1, Y1> 
where
  X1: Copy + PartialEq,
  Y1: Copy + PartialEq
{
  pub fn mixup(&self, other: &Point<X1, Y1>) -> Point<X1, Y1> {
    Point {
      x: self.x,
      y: other.y
    }
  }
}

impl <X1, Y1> PartialEq for Point<X1, Y1>
where
  X1: Copy + PartialEq,
  Y1: Copy + PartialEq
{
  fn eq(&self, other: &Self) -> bool {
      self.x == other.x && self.y == other.y
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mixup() {
    let point1 = Point {
      x: 5f32,
      y: 8
    };
    let point2 = Point {
      x: 1.0,
      y: 2
    };

    assert_eq!(point1.mixup(&point2), Point {
      x: 5f32,
      y: 2
    });
  }
}