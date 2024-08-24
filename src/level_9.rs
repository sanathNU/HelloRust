use text_io::read;
use std::io::{Write, BufReader, BufRead, Error};
use std::io;
use std::collections::HashMap;
use std::fs::File;

// **Case Reference Conversion**
// 🟩public wrapper function
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
    name: String,
    books: HashMap<String,Vec<Book>>
}

impl Library {
    fn new(name:String)-> Self{
        Library { 
            name: name, 
            books:HashMap::new(),}
    

    // fn get_genre(&self,Genre:&str)->&Vec<Book>{
    //     &self.books.get(Genre)
    //     .cloned()
    //     .unwrap_or_else(Vec::new)
    }
    fn get_all_genres(&self)->Vec<&String> {
        // &self.books.keys().cloned().collect()
        Vec::new()
    }
    
    fn get_all_books(&self)->Vec<Book> {
        Vec::new()
    }

    fn get_stats(&self)->HashMap<String,usize> {
        HashMap::new()
    }
}
struct Book {
    name: String,
    author: String,
    pages: u32
}
impl Book {
    fn new(name:String,author:String,pages:u32)->Book{
        Book{ name:name, author:author,pages:pages}
    }
    
    fn display(self:&Self) {
        println!("Name:{}",self.name);
        println!("Author:{}",self.author);
        println!("Pages:{}",self.pages);
    }
}
// 🟩public wrapper function
pub fn book_read_write()-> io::Result<()>{
    println!("This function reads from a txt file, and creates a library and writes it to a file");
    println!("The default direct it's going to read from is from the repo under books.txt");
    println!("Do you want to display the file statistics? (Y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ans = input.trim().to_uppercase();

    // let ans = input
    //     .trim()
    //     .to_uppercase()
    //     // .as_str()
    //     .map(|s| {
    //         if s=='Y' || s=='N' {
    //             Ok(s.to_string())
    //         }
    //         else {
    //             Err("Invalid input. Please enter Y or N")
    //         }
    //     });

    let file_stats = match ans.as_str() {
        "Y" => true,
        "N" => false,
        _ => panic!("Invalid input. Please enter Y or N")
    };
    let in_path = "books.txt";
    let out_path = "Library.txt";

    // let pustakam = read_input(in_path,file_stats);
    // // let library = create_library(pustakam);
    
    // let out_path = "library.txt";
    // write_library(out_path,library);



    match read_input(in_path, file_stats) {
        Ok(stats) => {
            if file_stats {
                println!("File Statistics:");
                println!("Word count: {}", stats.get("word_entry").unwrap_or(&0));
                println!("Line count: {}", stats.get("line_count").unwrap_or(&0));
            } else {
                println!("File read successfully.");
                // You can add more processing for the file content here if needed
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())

}
// fn read_input(path:&str,file_stats:bool)-> io::Result<HashMap<String, usize>>{
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//     // let contents = fs::read_to_string(path);

    // let books: Vec<&str> = contents.unwrap().split("\n\n").collect();
    // let statistcs = file_statistics(&contents.unwrap());

//     if file_stats {
//         let contents = file_statistics(&reader);
//         Ok(contents)
//     } else {
//         Ok(HashMap::new()) // Return an empty HashMap if file_stats is false
//     }
    
//     // file_statistics(contents);
//     // let buffered = BufReader::new(file);
// } //reads input from the file

//calculates statistics for the file
// fn file_statistics(contents:&File)->HashMap<&str,usize>{
//     let (mut line_count,mut word_count) = (0,0);
//     let reader = BufReader::new(contents);

//     for line in reader.lines() {
//         let line = line.unwrap();
//         line_count+=1;
//         word_count += line.split_whitespace().count();
//     }
//     let mut stats = HashMap::new();
//     stats.insert("word_entry",word_count);
//     stats.insert("line_count",line_count);
//     stats
// }

fn read_input(path: &str, file_stats: bool) -> io::Result<HashMap<String, usize>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    if file_stats {
        let stats = file_statistics(&reader);
        Ok(stats)
    } else {
        Ok(HashMap::new()) // Return an empty HashMap if file_stats is false
    }
}

fn file_statistics(reader: &BufReader<File>) -> HashMap<String, usize> {
    let (mut line_count, mut word_count) = (0, 0);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                line_count += 1;
                word_count += line.split_whitespace().count();
            }
            Err(_) => continue, // Handle read errors gracefully
        }
    }

    let mut stats = HashMap::new();
    stats.insert("word_entry".to_string(), word_count);
    stats.insert("line_count".to_string(), line_count);
    stats
}

fn create_library(){} //reads values from the file to create a genre of books
fn write_library(){} //writes the entire library to a file