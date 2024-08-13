/*These are key concepts about constants
in rust
- constant are always immutable
- we declare constant with const and not let
- type of value must be annotated */

fn main() {
  // declaring a constant and initializing it with a value of 100
  const MAX_POINTS: u32 = 100;
  // trying to change the value of MAX_POINTS will result in a compile-time error
  // MAX_POINTS = 200;
  println!("The maximum number of points is: {MAX_POINTS}");
}