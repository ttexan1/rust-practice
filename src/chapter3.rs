// chapter1 ~ chapter3 のもの

// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};

// fn main() {
//   let stdout = stdout();
//   let message = String::from("Hello fellow Rustaceans!");
//   let width = message.chars().count();

//   let mut writer = BufWriter::new(stdout.lock());
//   say(message.as_bytes(), width, &mut writer).unwrap();
// }

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_game() {
  println!("Guess the number!");
  println!("Please input your guess.");
  let secret_number = rand::thread_rng().gen_range(1..=100);
  // println!("The secret number is: {}", secret_number); // this is the answer;
  loop {
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please type a number!");
        continue;
      }
    };

    println!("You guessed: {}", guess);
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}

pub fn chrismas_carol() {
  let dates = ["first", "second", "third", "fourth", "fifth"];
  let phrases = [
    "A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
  ];
  for (count, date) in dates.iter().enumerate() {
    println!("On the {} day of Christmas, my true love sent to me", date);
    for n in (0..=count).rev() {
      println!("{}", phrases[n]);
    }
    println!("");
  }

  println!("{:?}", dates)
}
