//This is my approach
/* 
use std::io;

fn main(){
   println!("if you want to convert to Celsius put 1 if you want convert to fernheit put 2!");
  
   let mut choice = String::new();
  
   println!("put your choice 1 or 2");
  
   io::stdin().read_line(&mut choice)
   .expect("fail to read a line");
  
   let choice: u32 = choice.trim().parse().expect("not a number");
  
   if choice ==1{
      fahrenheit_to_celsius(choice);
   }else if choice==2{
      celsius_to_fahrenheit(choice);
   }else{
      println!("choice dont match any");
   }
  

}

fn celsius_to_fahrenheit(_f:u32){

    let mut c = String::new();

    println!("input temprature in Celsius!");

    io::stdin().read_line(&mut c)
    .expect("failed to read  a line");

    let c: u32 =c.trim().parse().expect("not a number");

    let  f = 9*c/5 +32;

    println!("{f} Fahrenheit");

}

fn fahrenheit_to_celsius(_c:u32){

    let mut f = String::new();
    
    println!("input temprature in Fahrenheit!");
    
    io::stdin().read_line(&mut f)
    .expect("fail to read the line");

    let f: u32 =f.trim().parse().expect("not a number");
    
    let c = 5*(f-32)/9;

    println!("{c} Celsius");

}

*/

//This correct approach

use std::io;

fn main() {
    println!("If you want to convert to Celsius, put 1. If you want to convert to Fahrenheit, put 2.");

    let mut choice = String::new();
    println!("Put your choice (1 or 2):");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = choice.trim().parse().expect("Not a number");

    if choice == 1 {
        fahrenheit_to_celsius();
    } else if choice == 2 {
        celsius_to_fahrenheit();
    } else {
        println!("Choice doesn't match any.");
    }
}

fn celsius_to_fahrenheit() {
    let mut c = String::new();

    println!("Input temperature in Celsius:");

    io::stdin()
        .read_line(&mut c)
        .expect("Failed to read line");

    let c: f64 = c.trim().parse().expect("Not a number");

    let f = (9.0 / 5.0) * c + 32.0;

    println!("{c} Celsius is equal to {f} Fahrenheit.");
}

fn fahrenheit_to_celsius() {
    let mut f = String::new();

    println!("Input temperature in Fahrenheit:");

    io::stdin()
        .read_line(&mut f)
        .expect("Failed to read line");

    let f: f64 = f.trim().parse().expect("Not a number");

    let c = (f - 32.0) * 5.0 / 9.0;

    println!("{f} Fahrenheit is equal to {c} Celsius.");
}

//This is Kings Approach
/*use std::io;

fn main() {
    println!("Convert to Celsius (1) or Fahrenheit (2):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => convert("Fahrenheit", |f| (f - 32.0) * 5.0 / 9.0, "Celsius"),
        "2" => convert("Celsius", |c| c * 9.0 / 5.0 + 32.0, "Fahrenheit"),
        _ => println!("Invalid choice"),
    }
}

fn convert(unit: &str, formula: fn(f64) -> f64, target: &str) {
    println!("Enter temperature in {unit}:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");

    let temp: f64 = temp.trim().parse().expect("Not a valid number");
    println!("{temp} {unit} is equal to {} {target}", formula(temp));
}
*/