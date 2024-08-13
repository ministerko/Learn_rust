fn main(){
  let s1 = String::from("Ruler");

  let len = calculate_length(s1); //if we add clone it can work but the method is expensive in terms of memory 

  println!("The length of {} is {}.", s1, len); // Error! s1 is no longer valid here.
}

fn calculate_length(s: String) -> usize {

  s.len()
}