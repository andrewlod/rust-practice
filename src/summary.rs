pub trait Summary {
  fn summarize(&self) -> String;
  fn create_news(&self, title: &str, content: &str) -> News {
    News {
      title: String::from(title),
      content: String::from(content)
    }
  }
}

impl Summary for &str {
  fn summarize(&self) -> String {
      format!("Breaking news: {}", &self)
  }
}

impl Summary for String {
  fn summarize(&self) -> String {
      format!("Breaking news: {}", &self)
  }
}

pub struct News {
  pub title: String,
  pub content: String
}

impl Summary for News {
  fn summarize(&self) -> String {
      format!("{}: {}", &self.title, &self.content)
  }
}

pub fn notify(item: impl Summary) {
  println!("{}", item.summarize());
}