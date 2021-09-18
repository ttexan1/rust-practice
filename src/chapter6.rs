// use std::net::IpAddr; // 参考 https://doc.rust-lang.org/std/net/enum.IpAddr.html
#[derive(Debug)]
enum IpAddrKind {
  V4,
  V6,
}

#[derive(Debug)] // String型に限定して何かできる
enum IpAddrKindStr {
  V4(String),
  V6(String),
}

#[derive(Debug)]
struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
// same meaning as avobe
// struct QuitMessage; // unit struct
// struct MoveMessage {
//   x: i32,
//   y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
  fn call(&self) {
    // method body would be defined here
  }
}

pub fn main6_1() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;
  println!("{:?}{:?}", four, six);

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };
  println!("{:?}{:?}", home, loopback);

  let m = Message::Write(String::from("hello"));
  m.call();

  // Optionについて
  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;
}
