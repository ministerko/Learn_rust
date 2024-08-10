use std::io;
use rand::Rng;
use std::cmp::Ordering;




fn main(){
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    //Making it multiple guesses with looping 
    loop {
        println!("input your guess");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
           .expect("failed to read the line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;}
        }
   
    }

   

}

