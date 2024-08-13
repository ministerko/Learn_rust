# References and Borrowing 
### Moving from Ownership to Borrowing in Rust

In Rust, when we pass data to a function, the function can take ownership of that data. But sometimes, we want to use the data after the function call, which leads us to a problem: if the function takes ownership, we lose the ability to use that data elsewhere.

For example, let’s look at this situation:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len); // Error! s1 is no longer valid here.
}

fn calculate_length(s: String) -> usize {
    s.len()
}
```

Here, `calculate_length` takes ownership of `s1`, and after the function call, `s1` is no longer valid, so we can’t use it again.

### Solution: Using References

Instead of moving ownership, we can **borrow** the data. Borrowing in Rust is done using **references**, which allow us to refer to a value without taking ownership.

Here’s how you can do it:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len); // s1 is still valid here
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Key Concepts:
- **&s1**: This creates a reference to `s1` without taking ownership. We’re saying, “I want to use `s1` in this function, but I don’t need to own it.”
- **&String in the function**: The function signature now says it accepts a reference to a `String`, not the `String` itself. This tells Rust that `calculate_length` will borrow the string instead of owning it.

### Why References Work

When we pass a reference, the original data (`s1`) is not moved; it’s just borrowed for a while. The function can use the data, but when the function ends, the original data is still valid and can be used elsewhere.

Think of it like borrowing a book from a friend. You can read it (use it), but you don’t own it, so when you’re done, your friend still has the book.

### What About Modifying Borrowed Data?

Here’s an important point: by default, references are immutable, meaning you can’t change the data you’re borrowing. If you try, Rust will give you an error.

Example:

```rust
fn main() {
    let s = String::from("hello");

    change(&s); // Error! We can’t change `s` because it’s borrowed as immutable.
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

This code doesn’t work because we’re trying to modify `some_string`, which is a reference to a `String`, and by default, it’s immutable. The compiler will tell you that you need to use a mutable reference if you want to modify the data.

### Fixing the Code with Mutable References

If you want to change the borrowed data, you need to explicitly state that the reference is mutable:

```rust
fn main() {
    let mut s = String::from("hello"); // The string itself must be mutable.

    change(&mut s); // We pass a mutable reference.

    println!("{}", s); // Now we can see the change.
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // Modify the borrowed string.
}
```

### Summary
- **References (`&T`)**: Allow you to borrow data without taking ownership.
- **Mutable References (`&mut T`)**: Allow you to borrow data and modify it.
- **Borrowing Rules**: Only one mutable reference or multiple immutable references can exist at the same time.

In Rust, borrowing is a powerful feature that lets you work with data safely, preventing many common bugs related to memory management.

Mutable references have one big restriction: *`if you have a mutable reference to a value, you can have no other references to that value`*__ <span style="color: yellow;">is like Newton's law aanhha 😆</span>
. This code that attempts to create two mutable references to `s` will fail:

```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

```
As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:


```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

```
Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the `println!,` occurs before the mutable reference is introduced:

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


```
## Dangling References

In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

Let’s try to create a dangling reference to see how Rust prevents them with a compile-time error:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```
Because `s` is created inside `dangle`, when the code of `dangle` is finished, `s` will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid `String`. That’s no good! Rust won’t let us do this.

The solution here is to return the `String` directly:

```rust 
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```
## The Rules of References

Let’s recap what we’ve discussed about references:

  - At any given time, you can have either one mutable reference or any number of immutable references.

  - References must always be valid.

Next, we’ll look at a different kind of reference: slices.

[jump here->](../slices/)

