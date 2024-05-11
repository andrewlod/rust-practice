pub struct Router {
  ip: String
}

pub trait Network {
  fn ping(&self, host: &str) -> bool;
  fn traceroute(&self, host: &str) -> Vec<String>;
  fn nslookup(&self, host: &str) -> String;
}

impl Network for Router {
  fn ping(&self, host: &str) -> bool {
    host == self.ip
  }

  fn traceroute(&self, host: &str) -> Vec<String> {
    vec![self.ip.to_string()]
  }

  fn nslookup(&self, host: &str) -> String {
    self.ip.to_string()
  }
}