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
  // println!("{:?}, {:?}", black, origin); // must implement `std::fmt::Debug`
  // println!("{:#?}, {:#?}", black, origin); // must implement `std::fmt::Debug`
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

// 以下 chapter5-2以下
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

pub fn main5_2() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  println!("rect1 is {:?}", rect1);
  println!("rect1 is {:#?}", rect1);
  println!(
    "The area of the rectangle is {} square pixels.",
    area(&rect1)
  );
}

fn area(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}

// 以下chapter5-3

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
  // associated function
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

pub fn main5_3() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  println!("Area is {} pixels", rect1.area());
  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };
  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };
  let rect4_square = Rectangle::square(32);
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect4_square));
}
