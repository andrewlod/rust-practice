use std::f32::consts::PI;

pub struct Circle {
  radius: f32
}

pub trait Shape {
  fn area(&self) -> f32;
  fn perimeter(&self) -> f32;
  fn draw(&self);
}

impl Shape for Circle {
  fn area(&self) -> f32 {
    PI * self.radius.powi(2)
  }

  fn perimeter(&self) -> f32 {
    2f32 * PI * self.radius
  }

  fn draw(&self) {
    println!("Circle with radius {}", &self.radius);
  }
}

pub fn demo() {
  let circle = Circle { radius: 10.0 };
  println!("Area: {}", circle.area());
  println!("Perimeter: {}", circle.perimeter());
  circle.draw();
}