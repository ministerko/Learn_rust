use std::io;

fn main(){
    println!("enter the position of the fibonacci number you want:");

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let n:u32 =input
    .trim()
    .parse()
    .expect("not a number");


    let fibonacci_number = fibonacci(n);
    println!("The {n}th fibonacci number is: {fibonacci_number}");


}

fn fibonacci(n: u32) -> u32{

    if n ==0{
        return 0;
    }else if n==1{
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut c;

    for _ in 2..=n{

        c = a+b;
        a =b;
        b=c;
    }
    b
}
