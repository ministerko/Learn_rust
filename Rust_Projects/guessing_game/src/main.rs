use std::io;
use rand::Rng;




fn main(){
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
       .expect("failed to read the line");
    println!("You guessed: {guess}");
   

}

