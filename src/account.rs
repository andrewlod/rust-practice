enum Currency {
  USD,
  EUR,
  GBP
}

trait BankAccount {
  fn new(balance: f64, currency: Currency) -> Self;
  fn deposit(&mut self, amount: f64);
  fn withdraw(&mut self, amount: f64) -> bool;
  fn check_balance(&self) -> f64;
}

struct Account {
  balance: f64,
  currency: Currency
}

impl BankAccount for Account {
  fn new(balance: f64, currency: Currency) -> Self {
      Account {
        balance,
        currency
      }
  }

  fn deposit(&mut self, amount: f64) {
      self.balance += amount;
  }

  fn withdraw(&mut self, amount: f64) -> bool {
      if self.balance < amount {
        false
      } else {
        self.balance -= amount;
        true
      }
  }

  fn check_balance(&self) -> f64 {
    self.balance
  }
}

pub fn demo() {
  let mut account = Account::new(1000f64, Currency::USD);
  println!("Balance: {}", &account.check_balance());

  account.deposit(500f64);
  println!("New balance: {}", &account.check_balance());
  
  match account.withdraw(1200f64) {
    true => println!("Withdrawal successful. New balance: {}", &account.check_balance()),
    false => println!("Withdrawal unsuccessful")
  }
}