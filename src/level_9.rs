use text_io::read;
use std::io;
use std::collections::HashMap;

// **Case Reference Conversion**
pub fn convert_case() {
    // Get user input
    println!("Enter the lowercase string to be case converted:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut input:String = input;

    // Print the original string
    println!("The String before case conversion is: {}", input);

    convert_char_case(&mut input);
    println!("The String after case conversion is: {}", input);
}

fn convert_char_case(input: &mut String) {
    let uppercased: String = input.chars().map(|c| c.to_uppercase()).flatten().collect();
    *input = uppercased;
}

// **Book Details
// The book details are pulled from a input file, hence even completing the function. There will also be a function to calculate file statistics
struct Library {
    books: HashMap<String,Vec<Book>>
}
struct Book {
    name: String,
    author: String,
    pages: i32
}

fn file_statistics(){}
fn read_input(){} //reads input from the file
fn create_library(){} //reads values from the file to create a genre of books
fn write_library(){} //writes the entire library to a file