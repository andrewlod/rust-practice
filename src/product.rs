
#[derive(Debug)]
struct Product {
  name: String,
  price: f64,
  amount: u32
}

impl Product {
  pub fn new(name: &str, price: f64, amount: u32) -> Product {
    Product {
      name: String::from(name), price, amount
    }
  }

  pub fn get_name(&self) -> &str {
    self.name.as_str()
  }

  pub fn get_price(&self) -> f64 {
    self.price
  }

  pub fn get_amount(&self) -> u32 {
    self.amount
  }

  pub fn set_name(&mut self, name: &str) {
    self.name = String::from(name);
  }

  pub fn set_price(&mut self, price: f64) {
    self.price = price;
  }

  pub fn set_amount(&mut self, amount: u32) {
    self.amount = amount;
  }
}

pub fn demo() {
  let mut product = Product::new("Banana", 1.5, 9);

  println!("{:?}", product);

  product.set_name("Apple");
  product.set_price(2.5);
  product.set_amount(10);
  
  println!("{:?}", product);
}