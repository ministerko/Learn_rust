/* 
But mutability can be very useful, and can make code
more convenient to write. Although variables are
immutable by default, you can make them mutable by 
adding mut in front of the variable name as you did
in Chapter 2. Adding mut also conveys intent to
future readers of the code by indicating that othe
r parts of the code will be changing this variable’s
value.

*/
fn main() {
    let mut x = 5;

    println!("the value of x is: {x}");

    x = 6;

    println!("the value of x is: {x}");
}
