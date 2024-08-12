

fn main() {
    // Here, we're declaring a mutable variable 'x' and assigning it an initial value of 5.
    let mut x = 5;

    // The `println!` macro is used to print output to the console. The current value of 'x' is interpolated into the string using `{x}`.
    println!("The value of x is: {x}");

    // Since 'x' is mutable, we can change its value. Here, we reassign 'x' with a new value of 6.
    x = 6;

    // Again, we use `println!` to output the new value of 'x' to the console.
    println!("The value of x is: {x}");
}

// Key points:
// - Variables are immutable by default in Rust. To make them mutable, you need to use the 'mut' keyword.
// - The `let` keyword is used for variable declaration.
// - The `println!` macro is used for printing output, and you can embed variables within the output string using `{}` for formatting.
// - The Rust compiler infers the type of variables based on their initial value, but explicit type annotations can be used when necessary.