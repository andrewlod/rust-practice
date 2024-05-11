#[derive(Debug)]
struct RealEstate {
  address: String,
  price: f64,
  num_rooms: u8,
  num_bathrooms: u8,
  area: f64
}

struct Broker {
  name: String,
  address: String,
  assets: Vec<RealEstate>
}

impl Broker {
  pub fn create_real_estate(&mut self, address: &str, price: f64, num_rooms: u8, num_bathrooms: u8, area: f64) {
    self.assets.push(RealEstate {
      address: String::from(address),
      price,
      num_rooms,
      num_bathrooms,
      area
    });
  }

  pub fn list_assets(&self) {
    println!("{:?}", &self.assets);
  }
}

pub fn demo() {
  let mut broker = Broker {
    name: String::from("MyBroker"),
    address: String::from("FooBar"),
    assets: Vec::new()
  };

  broker.create_real_estate("FooBar, 234", 123456.78, 2, 2, 78f64);
  broker.list_assets();
}