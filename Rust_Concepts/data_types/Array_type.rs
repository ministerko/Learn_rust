/* 
fn main (){

  let a = [1,2,3,4,5];

  let first = a[0];
  let second = a[1];

  println!("here are first and second array values {first},{second}");
}
*/

//Invalid Array Element Access

/*
Let’s see what happens if you try 
to access an element of an array
that is past the end of the array
 */

use std::io;

fn main(){

  let a =[1,2,3,4,5];

  println!("Please enter an array index.");

  let mut index = String::new();

  io::stdin()
    .read_line(&mut index);
    .expect("Failed to read line");

  let index: usize = index
      .trim()
      .parse()
      .expect()
      .expect("Index entered was not a number ");

  let element = a[index];

  println!("The value of the element at index {index} is: {element}");
}


