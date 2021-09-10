pub fn print_hello() {
  println!("Hello, world!");
}

pub fn main2() {
  let s = String::from("hello");
  takes_ownership(s);
  // println!("{}", s); // ownershipがtakenされたのでアウト

  let x = 5;

  makes_copy(x);

  let ss = String::from("World");
  let l = take_reference(&ss);
  println!("{}", l);


  let mut s2 = String::from("hello");
  let r1 = &mut s2;
  // let r2 = &mut s2; // 2回目はアウト

  println!("{}", r1);

  let mut s3 = String::from("hello");
  change_mut(&mut s3);
  change_mut(&mut s3);
  println!("{}",s3);
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}


fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

fn take_reference(s: &String) -> usize {
  s.len()
}


// fn change(some_string: &String) {
//     some_string.push_str(", world"); // referenceを書き換えできない
// }


pub fn change_mut(some_string: &mut String) {
  some_string.push_str(", world");
}

