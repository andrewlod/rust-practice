pub trait State {
  fn publish(self: Box<Self>) -> Box<dyn State>;
  fn reject(self: Box<Self>) -> Box<dyn State>;
}

pub struct Draft {
  pub content: String,
  pub rejected: bool
}

pub struct Published {
  pub content: String
}

impl State for Draft {
  fn publish(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {
      content: self.content
    })
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    Box::new(Draft {
      content: self.content,
      rejected: true
    })
  }
}

impl State for Published {
  fn publish(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    Box::new(Draft {
      content: self.content,
      rejected: true
    })
  }
}

struct Post {
  state: Option<Box<dyn State>>,
  content: String
}

impl Post {
  fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {
        content: String::new(),
        rejected: false
      })),
      content: String::new()
    }
  }

  fn add_content(&mut self, text: &str) {
    self.content.push_str(text);
  }

  fn publish(&mut self) {
    if let Some(state) = self.state.take() {
      self.state = Some(state.publish());
    }
  }

  fn reject(&mut self) {
    if let Some(state) = self.state.take() {
      self.state = Some(state.reject());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_publish() {
    let mut post = Post::new();
    post.add_content("Hello, world!");

    assert_eq!(post.content, "Hello, world!");
    
    post.publish();
    assert!(post.state.is_some());
  }

  #[test]
  fn test_reject() {
    let mut post = Post::new();
    post.add_content("Hello, world!");

    assert_eq!(post.content, "Hello, world!");
    
    post.reject();
    assert!(post.state.is_some());
  }
}