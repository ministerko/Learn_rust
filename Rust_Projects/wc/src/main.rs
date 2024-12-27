use std::io::{self,Write};

fn main() {
    println!("Welcome to text processor ");
    //Get some input text
    println!("please enter some text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input ");
    let input = input.trim();

    loop {
        //Display Menu
        println!("\nChoose an option:");
        println!("1. Count Words");
        println!("2. Countig characters");
        println!("3. Extrract tyhe first word");
        println!("4. Find a string ");
        println!("5. Reverse the text");
        println!("6. Quit");

        println!("Enter your choice ");            
        io::stdout().flush().unwrap(); 
      
        //Get user input \
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).        
        expect("Failed to read the choice ");
        let choice = choice.trim();
        
        match choice {
            "1" => {
                let word_count = count_words(input);
                println!("The text contains {} words.",word_count)
            }
            "2" => {
                 let char_count  = count_characters(input);
                 println!("The text contains {} characters (excluding spaces).",char_count);
            }
            "3" => {
                match extract_first_word(input) {
                    Some(word) => println!("The first word is '{}'.",word),
                    None =>println!("No words found in the input. "),

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
    }
}
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_characters(text: &str) -> usize {
    text.chars().filter(|&c| !c.is_whitespace()).count()
}

fn extract_first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}

fn find_substring(text: &str, substring: &str) -> Option<usize> {
    text.find(substring)
}

fn reverse_text(text: &str) -> String {
    text.chars().rev().collect()
}

