use std::fmt;

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn main5_1() {
  let email = String::from("test@example.comm");
  let username = String::from("test@example.comm");
  let user1 = build_user(email, username);
  println!("{}, {}", user1.email, user1.username);

  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };
  println!("{}, {}", user2.email, user2.username);

  let black = Color(255, 255, 0);
  let origin = Point(10, -20, 0);
  println!("{}, {}", black, origin);
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Color: ({}, {}, {})", self.0, self.1, self.2)
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Point: ({}, {}, {})", self.0, self.1, self.2)
  }
}
