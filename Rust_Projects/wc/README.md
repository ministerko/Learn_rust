# Rust Text Processor

The Rust Text Processor is an interactive command-line application that allows users to perform various operations on a given text input. It is implemented in Rust and uses the standard library to handle input, output, and string manipulations.

## Features
1. Count the number of words in the input text.
2. Count the number of non-whitespace characters in the input text.
3. Extract the first word from the input text.
4. Find the starting position of a substring within the input text.
5. Reverse the input text.
6. Exit the program.

## Code Explanation

### Import Statements
```rust
use std::io::{self, Write};
```
- **`std::io`**: Provides functionalities for input/output operations.
- **`self`**: Imports the `io` module itself.
- **`Write`**: Ensures the use of `io::stdout().flush()` for interactive output.

### Main Function
```rust
fn main() {
    println!("Welcome to the Rust Text Processor!");
```
- **`println!`**: Prints a message to the console.

#### Input Handling
```rust
    println!("Please enter some text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
```
- **`String::new()`**: Creates an empty, mutable `String` to store the input.
- **`io::stdin().read_line(&mut input)`**: Reads a line of input from the user and appends it to `input`.
- **`trim()`**: Removes leading and trailing whitespace from the input.

#### Main Loop
```rust
    loop {
        println!("\nChoose an option:");
        println!("1. Count words");
        println!("2. Count characters");
        println!("3. Extract the first word");
        println!("4. Find a substring");
        println!("5. Reverse the text");
        println!("6. Quit");
```
- Displays the menu repeatedly until the user chooses to quit.

#### Interactive Output
```rust
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
```
- **`flush()`**: Ensures the `print!` statement is immediately displayed to the user.

### Match Statement for Menu Options
```rust
        match choice {
            "1" => {
                let word_count = count_words(input);
                println!("The text contains {} words.", word_count);
            }
            "2" => {
                let char_count = count_characters(input);
                println!("The text contains {} characters (excluding spaces).", char_count);
            }
            "3" => {
                match extract_first_word(input) {
                    Some(word) => println!("The first word is: '{}'.", word),
                    None => println!("No words found in the input."),
                }
            }
            "4" => {
                println!("Enter the substring to find:");
                let mut substring = String::new();
                io::stdin().read_line(&mut substring).expect("Failed to read substring");
                let substring = substring.trim();

                match find_substring(input, substring) {
                    Some(pos) => println!("The substring \"{}\" starts at position {}.", substring, pos),
                    None => println!("The substring \"{}\" was not found.", substring),
                }
            }
            "5" => {
                let reversed = reverse_text(input);
                println!("Reversed text: {}", reversed);
            }
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
```
- **`match`**: Handles the user’s choice and calls the appropriate function.

### Utility Functions

#### `count_words`
```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
```
- Splits the input into words and counts them.
- Uses `split_whitespace` to handle spaces, tabs, and newlines.

#### `count_characters`
```rust
fn count_characters(text: &str) -> usize {
    text.chars().filter(|&c| !c.is_whitespace()).count()
}
```
- Filters out whitespace characters and counts the remaining ones.

#### `extract_first_word`
```rust
fn extract_first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}
```
- Returns the first word or `None` if the input is empty.

#### `find_substring`
```rust
fn find_substring(text: &str, substring: &str) -> Option<usize> {
    text.find(substring)
}
```
- Locates the starting index of a substring in the input text.
- Returns `None` if the substring is not found.

#### `reverse_text`
```rust
fn reverse_text(text: &str) -> String {
    text.chars().rev().collect()
}
```
- Reverses the input text using `chars()` and collects the result into a `String`.

## How to Run

1. **Install Rust**:
   Ensure Rust is installed on your system. If not, install it using:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone or Create the Project**:
   - Clone this repository or create a new Rust project using:
     ```bash
     cargo new rust_text_processor
     cd rust_text_processor
     ```

3. **Paste the Code**:
   Replace the contents of `src/main.rs` with the provided code.

4. **Run the Project**:
   - Build and run the application:
     ```bash
     cargo run
     ```

5. **Interact with the Program**:
   Follow the prompts to test the features.

## Example Interaction
```
Welcome to the Rust Text Processor!
Please enter some text: Rust is a systems programming language.

Choose an option:
1. Count words
2. Count characters
3. Extract the first word
4. Find a substring
5. Reverse the text
6. Quit

Enter your choice: 1
The text contains 6 words.
```

## Requirements
- Rust programming language
- Command-line terminal

## License
This project is open source and available under the MIT License.

