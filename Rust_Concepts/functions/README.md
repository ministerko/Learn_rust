# Functions
the `main` function, which is the entry point of many programs. You’ve also seen the `fn`keyword, which allows you to declare new functions.

Rust code uses `snake_case` as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words. Here’s a program that contains an example function definition:

![check codes->](./main.rs)

### Parameters

These are special variables that are part of a function’s signature.

When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called `arguments`, but in casual conversation, people tend to use the words `parameter` and `argument` interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function

![check codes->](./parameters.rs)


you can use multiple parameters and requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean

When defining multiple parameters, separate the parameter declarations with `commas`, like this:

```
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

### statements and expression


- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. 
  
##### statements
Let’s look at some examples.

```
fn main() {
    let y = 6; // this is statement
}
```

`Statements` do not return values. Therefore, you can’t assign a `let` statement to another variable, as the following code tries to do; you’ll get an error:

```
fn main() {
    let x = (let y = 6);
}

```
##### expressions
Consider a math operation, such as `5 + 6`, which is an expression that evaluates to the value `11`. Expressions can be part of statements

`6` in the statement `let y = 6;` is an expression that evaluates to the value `6`

Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:

![check codes](./statements_and_expressions.rs)

This is expression:

```
{
    let x = 3;
    x + 1
}
```

### Functions with Return Values

Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (`->`). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

![check codes->](./rerurn_values.rs)






