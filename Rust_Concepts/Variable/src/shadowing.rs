/* 
variable shadowing is a feature that allows
 you to declare a new variable with the same
 
name as a previous variable. The new variable 
"shadows" the previous one, meaning that after
 the new declaration, the old variable is 
 no longer accessible. However, unlike in 
 some other languages, the original variable
 
is not modified; instead, a completely new 
variable is created.*/

fn main(){
  let x = 5;

  let x = "I am shadowed variable";

  println!(" The value of x is: {x}");
}

//shadowing vs mutability

/*
- Mutability: In Rust, a variable declared 
with let mut can be changed, but its type must remain the same.

- Shadowing: Shadowing allows you to redeclare a variable with a 
different type or value, even if the original was immutable. 
It’s useful when you want to reuse a variable name but change 
its type or value in a safe and controlled way. */

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