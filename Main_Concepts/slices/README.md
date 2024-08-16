# The Slice Type

**Let state the problem first**

We need to find the first space in given string "hello world"

Here the solution 

```rust
fn main() {
  let mut s = String::from("hello world");

  let word = first_word(&s); // word will get the value 5

  println!("{word}");

  s.clear(); // this empties the String, making it equal to ""

  // word still has the value 5 here, but there's no more string that
  // we could meaningfully use the value 5 with. word is now totally invalid!
}
fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i,&item) in bytes.iter().enumerate() {
    if item == b' '{

      return i;

    }
  }
  s.len()
}
```
That Rust code is an introduction to string slicing, showing how to find the position of the first space in a string. However, it demonstrates a problem with handling string slices in a way that could lead to bugs.

### How it works 🤔 

- String Creation: A mutable String is created with the value `"hello world"`.

- Finding the First Word: The `first_word` function takes a reference to this string and returns the index of the first space (which is `5` in this case).

- Clearing the String: The string is then cleared using `s.clear()`, making it an empty string (`""`).

- Issue: The variable `word` still holds the value `5`, which was valid when the string was "`hello world`". However, since the string has been cleared, this index is now meaningless.
