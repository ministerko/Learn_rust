//
//use std::io;
//
//fn main(){
//    println!("Guess the Number!");
//
//    println!("input your guess");
//
//    let mut guess = String::new();
//
//    io::stdin().read_line(&mut guess)
//       .expect("failed to read the line");
//    println!("You guessed: {}",guess);
//
//}

//using rand 

extern crate rand;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // Note the new syntax
    println!("The secret number is: {}", secret_number);
}
