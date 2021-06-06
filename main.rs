use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
  let secret_number = rand::thread_rng().gen_range(1..101);
  println!("secret numer is: {}", secret_number);
  println!("input guess:");
  let mut guess = String::new();
  io::stdin().read_line(&mut guess).expect("Failed to read line");
  let guessed_num: u32 = guess.trim().parse().expect("please type a number");
  println!("guessed number: {}", guess);
  match guessed_num.cmp(&secret_number) {
    Ordering::Less => println!("too small"),
    Ordering::Greater => println!("too big"),
    Ordering::Equal => println!("exactly")
  }
}
