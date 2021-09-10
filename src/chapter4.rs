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
  println!("{}", s3);
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

// chapter4.3.

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}

fn main4_3() {
  let mut s = String::from("hello world");

  let word = first_word(&s); // word will get the value 5

  // error occurs
  // s.clear(); // this empties the String, making it equal to ""

  // word still has the value 5 here, but there's no more string that
  // we could meaningfully use the value 5 with. word is now totally invalid!
  println!("the first word is: {}", word);
}

pub fn main4_3_1() {
  let my_string = String::from("hello world");

  // first_word works on slices of `String`s
  let word = first_word(&my_string);

  let my_string_literal = "hello world";

  // first_word works on slices of string literals
  let word = first_word(&my_string_literal[..]);

  // Because string literals *are* string slices already,
  // this works too, without the slice syntax!
  let word = first_word(my_string_literal);
}
