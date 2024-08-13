# Control Flow
The ability to run some code depending on whether a condition is `true` and to run some code repeatedly while a condition is `true` are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are `if` expressions and loops.

### if Expressions
You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code."

![check codes->](./if_expression.rs)

*`if` expressions are sometimes called `arms`*

**note:**
 condition in this code must be a `bool`. If the condition isn’t a `bool`, we’ll get an error
  for example
  ```
  fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
  ```

  error

  ```
  $ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error

  ```
###### Handling Multiple Conditions with `else if`

You can use multiple conditions by combining `if` and `else` in an `else if `expression. For example:

```
fn main(){
let number = 6;
if number % 4 == 0 {
  println!("number is divisble by 4");
} else if number % 3 == 0 {
  println!("number is divisible by 3");
}else if number % 2 == 0 {
  println!("number is divisible by 2");
} else {
  println!("number is not divisible by 4, 3, or 2");
    }
}
```
##### Using `if` in a `let` Statement
Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable

```
fn main() {
  let condition = true;
  let number = if condition {5} else {6};
  println!("the value of number is: {number});
}
```
*note: each `arm` of the if must be the same type*

### Repetition with Loops
Rust has three kinds of loops: `loop`, `while`, and `for`.

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop


```
fn main() {
    loop {
        println!("again!");
    }
}
```

###### Returning Values from Loops

```
fn main(){
  let mut counter = 0;

  let result = loop {
    counter += 2;

    if counter == 10{
      break counter *2;
    }
  };

  println!("The result is {result}");
}
```
Before the loop, we declare a variable named `counter` and initialize it to 0. Then we declare a variable named `result` to hold the value returned from the loop. On every iteration of the loop, we add `2` to the `counter` variable, and then check whether the `counter` is equal to `10`. When it is, we use the break keyword with the value `counter * 2`. After the loop, we use a semicolon to end the statement that assigns the value to `result`. Finally, we print the value in `result`, which in this case is `20`.

##### Loop Labels to Disambiguate Between Multiple Loops

If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. You can optionally specify a loop label on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. Here’s an example with two nested loops

![check codes->](./loops.rs)

The outer loop has the label `'counting_up`, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first `break` that doesn’t specify a label will exit the inner loop only. The `break 'counting_up;` statement will exit the outer loop

##### Conditional Loops with `while`

A program will often need to evaluate a condition within a loop. While the condition is `true`, the loop runs. When the condition ceases to be `true`, the program calls `break`, stopping the loop

![check codes->](./while_loop.rs)

You can choose to use the `while` construct to loop over the elements of a collection, such as an array.

![check codes->](./array_loop.rs)

That is good  but this approach below will help us to handle situation correctly 

##### Looping Through a Collection with `for`
As a more concise alternative, you can use a `for` loop and execute some code for each item in a collection

![check codes->](./for_loops.rs)

**count down with `for` loop**

```
fn main(){
  for number in (1..4){
    println!("{number}");

  }
  println!("LIFTOFF!!!")
}
```

here are cools stuff to complete this chapter

- Convert temperatures between Fahrenheit and Celsius. ![see codes here](/Rust_Projects/temp_convert/src/main.rs)
- Generate the nth Fibonacci number.![see codes here](/Rust_Projects/fibonacci/src/main.rs)
- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song ![see codes here](/Rust_Projects/twelve_days_of_chrismas/src/main.rs)


if all done jump to ![Ownership](/Main_Concepts/Ownership/)


