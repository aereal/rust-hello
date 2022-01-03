use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
  let secret_number = rand::thread_rng().gen_range(1..101);
  loop {
    print!("input guess: ");
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    let guessed_num: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    println!("guessed number: {}", guess);
    match guessed_num.cmp(&secret_number) {
      Ordering::Less => println!("too small"),
      Ordering::Greater => println!("too big"),
      Ordering::Equal => {
        println!("exactly");
        break;
      }
    }
  }
}
