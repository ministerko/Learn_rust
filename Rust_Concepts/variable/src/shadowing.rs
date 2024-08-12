
fn main(){
  let x = 5;

  let x = "I am shadowed variable";

  println!(" The value of x is: {x}");
}

//shadowing vs mutability


/*
fn main() {
    let mut y = 5; // Mutable variable
    y = 10;        // Can be reassigned but still an integer
    // y = "hello";  // Error: can't change type of 'y'

    let z = 5; // Immutable variable
    let z = "hello"; // Shadowed with a new type (string)
    println!("z is now: {}", z);
}

 */