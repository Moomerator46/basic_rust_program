use std::io;

fn main() {
  println!("Please enter your name:");
  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("There has been a failure reading the stdio input. Please close the program and try again.");
  println!("Hello, {}!", name);
}
