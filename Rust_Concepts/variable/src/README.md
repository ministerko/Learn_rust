# Variables

This Rust program provides an example of variable 
declaration, mutability, and reassignment in Rust.
In Rust, variables are immutable by default. 
This means once a value is assigned to a variable, it cannot be changed. 
However, if you want to allow a variable's value to 
be changed after its initial assignment, you need to
explicitly make it mutable by using the 'mut' keyword.
Declaring a variable as mutable allows the value stored 
in the variable to be modified.

Rust uses the `let` keyword for declaring variables.
The type of the variable is usually inferred by the
compiler based on the value assigned to it.
You can also explicitly specify the type if needed, 
but in this example, the type is inferred as an integer (`i32` by default).

![check codes->](./variables.rs)

# Constants

Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you aren’t allowed to use `mut` with constants. Constants aren’t just immutable by default—they’re always immutable. You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value must be annotated

![check codes->](./constants.rs)


# Shadowing
 
variable shadowing is a feature that allows
you to declare a new variable with the same
name as a previous variable. The new variable 
"`shadows`" the previous one, meaning that after
the new declaration, the old variable is 
no longer accessible. However, unlike in 
some other languages, the original variable
is not modified; instead, a completely new 
variable is created.

### Shadowing vs Mutability

- Mutability: In Rust, a variable declared 
with let mut can be changed, but its type must remain the same.

- Shadowing: Shadowing allows you to redeclare a variable with a 
different type or value, even if the original was immutable. 
It’s useful when you want to reuse a variable name but change 
its type or value in a safe and controlled way. 

![check codes->](./shadowing.rs)