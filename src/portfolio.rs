enum Asset {
  Stock(f32),
  Bond(f32),
  Fund(f32),
  Cash(f32)
}

impl Asset {
  pub fn price(&self) -> f32 {
    match self {
      Asset::Stock(price) => *price,
      Asset::Bond(price) => *price,
      Asset::Fund(price) => *price,
      Asset::Cash(price) => *price,
    }
  }
}

pub fn demo() {
  let portfolio = vec![
    Asset::Stock(100.0),
    Asset::Bond(200.0),
    Asset::Fund(300.0),
    Asset::Cash(400.0),
    Asset::Stock(500.0),
    Asset::Bond(600.0)
  ];

  let prices: Vec<f32> = portfolio.iter().map(|asset| asset.price()).collect();
  println!("{:?}", &prices);

  let total: f32 = prices.iter().sum();
  println!("{}", total);
}