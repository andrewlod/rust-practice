enum AssetCategory {
  Stock,
  Bond,
  Fund,
  Cash
}

struct Asset {
  name: String,
  value: f64,
  category: AssetCategory,
}

impl Asset {
  pub fn get_value(&self) -> f64 {
    self.value
  }
}

struct Portfolio {
  assets: Vec<Asset>
}

impl Portfolio {
  pub fn new() -> Portfolio {
    Portfolio {
      assets: Vec::new()
    }
  }

  pub fn add_asset(&mut self, asset: Asset) {
    self.assets.push(asset);
  }

  pub fn get_total_value(&self) -> f64 {
    self.assets.iter().fold(0f64, |acc, val| acc + val.get_value())
  }
}

pub fn demo() {
  let mut portfolio = Portfolio::new();
  portfolio.add_asset(Asset {
    name: String::from("AAPL"),
    value: 100.0,
    category: AssetCategory::Stock
  });
  portfolio.add_asset(Asset {
    name: String::from("MSFT"),
    value: 200.0,
    category: AssetCategory::Stock
  });

  println!("{}", &portfolio.get_total_value());
}