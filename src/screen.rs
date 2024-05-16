pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {}

impl Draw for Button {
  fn draw(&self) {
      println!("Drawing a button!");
  }
}

pub struct SelectBox {}

impl Draw for SelectBox {
  fn draw(&self) {
      println!("Drawing a select box!");
  }
}

pub struct TextBox {}

impl Draw for TextBox {
  fn draw(&self) {
      println!("Drawing a text box!");
  }
}