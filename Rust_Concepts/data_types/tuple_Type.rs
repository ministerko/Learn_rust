//Get indvidual values out of a tuple

/*

fn main(){
  let tup: (i32,f64,u8) = (500,6.4,1);

  let (x,y,z) = tup;

  println!("here the tuple {y}");
}


*/

//we can acsess tuple directly using period

fn main (){

  let x: (i32, f64, u8)= (500,6.4,1);

  let five_hundred = x.0;

  let six_point_four = x.1;

  let one = x.2;

  println!("({five_hundred},{six_point_four},{one})");
}
