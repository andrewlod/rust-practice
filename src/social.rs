use std::{collections::{HashMap, HashSet}};

#[derive(Debug)]
enum SocialError {
  NotFound(String),
  AlreadyFriends(String, String),
  AlreadyExists(String),
  CannotAddItself
}

struct User {
  name: String,
  age: u16,
  friends: HashSet<String>
}

impl User {
  pub fn add_friend(&mut self, name: String) -> Result<(), SocialError> {
    if name == self.name {
      return Err(SocialError::CannotAddItself);
    }

    if self.friends.contains(&name) {
      return Err(SocialError::AlreadyFriends(self.name.clone(), name));
    }

    self.friends.insert(name);
    Ok(())
  }

  pub fn list_friends(&self) -> HashSet<String> {
    self.friends.clone()
  }
}

struct SocialNetwork {
  users: HashMap<String, User>
}

impl SocialNetwork {
  pub fn new() -> SocialNetwork {
    SocialNetwork {
      users: HashMap::new()
    }
  }

  pub fn add_user(&mut self, user: User) -> Result<(), SocialError> {
    if self.users.contains_key(&user.name) {
      return Err(SocialError::AlreadyExists(user.name));
    }

    self.users.insert(user.name.clone(), user);
    Ok(())
  }

  pub fn get_user(&self, name: &str) -> Result<&User, SocialError> {
    if !self.users.contains_key(&name.to_string()) {
      return Err(SocialError::NotFound(name.to_string()));
    }

    let user = self.users.get(&name.to_string()).unwrap();
    Ok(user)
  }

  pub fn get_user_mut(&mut self, name: &str) -> Result<&User, SocialError> {
    if !self.users.contains_key(&name.to_string()) {
      return Err(SocialError::NotFound(name.to_string()));
    }

    let user = self.users.get_mut(&name.to_string()).unwrap();
    Ok(user)
  }

  pub fn add_friend(&mut self, user1: &str, user2: &str) -> Result<(), SocialError> {
    if !self.users.contains_key(&user1.to_string()) {
      return Err(SocialError::NotFound(user1.to_string()));
    }
    
    if !self.users.contains_key(&user2.to_string()) {
      return Err(SocialError::NotFound(user2.to_string()));
    }

    let user_a = self.users.get_mut(&user1.to_string()).unwrap();
    user_a.add_friend(user2.to_string())?;

    let user_b = self.users.get_mut(&user2.to_string()).unwrap();
    user_b.add_friend(user1.to_string())?;

    Ok(())
  }
}

fn print_result(result: Result<(), SocialError>) {
  match result {
    Ok(()) => println!("Success"),
    Err(err) => println!("Error: {:?}", err)
  }
}

pub fn demo() {
  let mut social_network = SocialNetwork::new();

  let result = social_network.add_user(User {
    name: String::from("John"),
    age: 20,
    friends: HashSet::new()
  });
  print_result(result);

  let result = social_network.add_user(User {
    name: String::from("Ana"),
    age: 34,
    friends: HashSet::new()
  });
  print_result(result);

  let result = social_network.add_friend("John", "Ana");
  print_result(result);

  let john = social_network.get_user("John").unwrap();
  let john_friends = john.list_friends();
  println!("John's friends: {:?}", john_friends);
}