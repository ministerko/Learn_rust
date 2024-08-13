# Ownership

**why `ownership??` 🤔**
It enables Rust to make `memory safety` guarantees without needing a `garbage collector`, so it’s important to understand how ownership works

**Why Memory Safety is Important:**

- Security: Memory safety helps prevent security vulnerabilities such as buffer overflow attacks, which can be exploited to execute arbitrary code or compromise systems.

- Stability: By avoiding memory-related bugs, programs are less likely to crash or behave unpredictably.

- Reliability: Ensuring that memory management is handled correctly contributes to the overall reliability and correctness of software.

## What is ownership by the way 🤷
Ownership is a concept in Rust that determines which part of the code is responsible for deallocating the memory
 
more about `ownership`
[see here](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

#### The Stack and the Heap

In Rust, **The Stack** and **The Heap** are two different areas in memory where data is stored. Here's a simple explanation:

### The Stack
- **What it is:** A region of memory that works like a stack of plates. You add (or "push") data on top, and remove (or "pop") it from the top.
- **How it works:** When you create a variable in Rust that has a fixed size (like integers, booleans, or references), it's stored on the stack. The stack is very fast because it knows exactly where to put and find data.
- **Lifespan:** Data on the stack is automatically removed when the function that created it finishes. This is called "last in, first out" (LIFO).
- **Example:**
  ```rust
  fn main() {
      let x = 5; // 'x' is stored on the stack.
  } // 'x' is removed from the stack here.
  ```

### The Heap
- **What it is:** A region of memory used for storing data that might change in size or needs to live longer than the current function call.
- **How it works:** When you need to store data that can grow (like a `String` or a `Vec`) or data that needs to be shared, Rust puts it on the heap. Accessing heap data is slower than stack data because you need to manage it (allocate and deallocate memory).
- **Lifespan:** Data on the heap stays there until you explicitly remove it or Rust's ownership rules automatically handle it for you (via a feature called `drop`).
- **Example:**
  ```rust
  fn main() {
      let s = String::from("hello"); // 's' is stored on the heap.
  } // 's' is automatically removed from the heap here.
  ```

### Key Differences
- **Stack:** Fast, fixed-size, automatically cleaned up.
- **Heap:** Flexible size, slightly slower, managed by Rust’s ownership system.

In Rust, memory management is handled for you, so you don't have to worry about manually allocating or freeing memory, but understanding the difference between stack and heap helps you write more efficient code.

## Ownership Rules
Here are the main rules of ownership in Rust:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Variable Scope

As a first example of `ownership`, we’ll look at the scope of some variables. A scope is the range within a program for which an item is valid. Take the following `variable`:

```rust
let s = "hello";

```
The variable `s` refers to a string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it’s declared until the end of the current scope.

this codes below will show where `s`
will be valid 

```rust 
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid


```
more about `ownership`
[see here](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)


## The String Type 

To illustrate the rules of `ownership`, we need a data type that is more complex than those we covered in the “`Data Types`”

we want to look at data that is stored on the `heap` and explore how Rust knows when to clean up that data, and the `String` type is a great example


You can create a `String` from a string literal using the `from` function, like so:

```rust
let s = String::from("hello");
```

The double colon `::` operator allows us to namespace this particular `from `function under the `String` type rather than using some sort of name like `string_from`.

```rust
let mut s = String::from("hello");

s.push_str(",world!"); // push_str() appends a literal to a String

println!("{}",s); // This will print `hello, world!`
```
So, what’s the difference here? Why can `String` be mutated but literals cannot? The difference is in how these two types deal with memory

## Memory and Allocation

In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program

With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our String

That first part is done by us: when we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming 
languages.

[Read more](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

## Variables and Data Interacting with Move 

In Rust, variables and data interact with each other through a concept called **ownership**, and one key part of this is the idea of a **move**.

### Ownership and Moves
- **Ownership:** In Rust, every piece of data has a single owner—a variable that "owns" that data. When the owner goes out of scope, Rust automatically cleans up the data (frees memory). 

- **Move:** When you assign a value from one variable to another, you might expect that Rust would copy the data, but in some cases, it doesn't. Instead, it **moves** the ownership of the data from the original variable to the new one. After the move, the original variable is no longer valid.

### How Move Works
When a **move** happens, Rust transfers ownership of the data from one variable to another. Here’s an example:

```rust
fn main() {
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1; // s1's ownership is moved to s2

    // println!("{}", s1); // Error: s1 is no longer valid
    println!("{}", s2); // Works fine, as s2 now owns the String
}
```

### What's Happening Here ? 🤔
- **Step 1:** `s1` is created and owns the string `"hello"`.
- **Step 2:** When you assign `s1` to `s2`, Rust **moves** the ownership of `"hello"` from `s1` to `s2`.
- **After the move:** `s1` is no longer valid, so if you try to use `s1` after the move, Rust will throw a compile-time error. Only `s2` owns the data now.

### Why Use Moves? 🤔
Moves help Rust ensure **memory safety** without needing a garbage collector. By knowing exactly who owns what, Rust can automatically clean up data when it's no longer needed, avoiding common bugs like dangling pointers or double-free errors.

### Moves vs. Copy
For simple data types like integers, Rust uses a **copy** instead of a move. This is because copying an integer is cheap and doesn’t require transferring ownership.

```rust
fn main() {
    let x = 5;
    let y = x; // Copy happens because x is a simple type
    println!("{}", x); // Works fine, x is still valid
    println!("{}", y); // y is also valid
}
```

- **Copy types:** Simple, fixed-size types (like integers, booleans, etc.) are automatically copied instead of moved.
- **Move types:** Complex types (like `String`, `Vec`, etc.) are moved to ensure memory safety.

### Summary
- **Move**: Transfers ownership from one variable to another. The original variable becomes invalid.
- **Ownership**: Ensures that each piece of data has only one owner, allowing Rust to automatically manage memory.
- **Copy**: For simple types, Rust makes a copy instead of moving ownership, allowing both variables to be used.

Understanding moves and ownership is key to writing safe and efficient Rust code.

## Variables and Data Interacting with Clone
If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called `clone`

[see codes->](./clone.rs)

## Ownership and Functions
The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does

[see codes->](./Ownership_and_Functions.rs)

## Return Values and Scope

In Rust, understanding how return values and scope work together is essential for managing memory safely and efficiently. Here's a breakdown of these concepts:

### Scope in Rust
**Scope** is the region of code where a variable is valid and accessible. When a variable goes out of scope, Rust automatically cleans up (or "drops") the resources that the variable was using.

```rust
{
    let x = 5; // x comes into scope
    // x can be used here
} // x goes out of scope, and is no longer valid
```

### Ownership and Scope
When a variable goes out of scope, Rust takes ownership of the resources it was managing and automatically frees the memory. This is a key part of Rust's memory safety model.

```rust
{
    let s = String::from("hello");
} // s goes out of scope here, and the memory is freed
```

### Return Values and Ownership
When a function returns a value, it can transfer ownership of that value to the calling function. This is how Rust allows data to be passed around while maintaining strict ownership rules.

### Example: Returning Values and Ownership

[see codes->](./return_values_and_scope.rs)

### How it Works:
1. **gives_ownership:** This function creates a `String` and then returns it. Ownership of the string is moved to the calling function (`main`), where it is assigned to `s1`.

2. **takes_and_gives_back:** This function takes ownership of a `String` (passed as `s2`) and then returns it. The returned value is assigned to `s3`, so `s3` now owns the string.

3. **Ownership Transfer:** When a value is returned from a function, ownership is transferred to the receiving variable in the calling function. The original variable in the called function is no longer valid after the return.

### Scope of Returned Values
When a value is returned from a function, the scope of that value extends to wherever the returned value is used. This allows the data to be used beyond


But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for using a value without transferring ownership, called `references`

[jump here->](../References/README.md)