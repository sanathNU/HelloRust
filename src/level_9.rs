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
    }
    fn add_book(&mut self,book:Book,genre:String) {
        let entry = self.books.entry(genre).or_insert_with(Vec::new);
        entry.push(book);
    }

    fn get_genre(&self, genre: &str) -> Option<&Vec<Book>> {
        self.books.get(genre)
    }
    fn get_all_genres(&self)->Vec<&String> {
        self.books.keys().collect()
        // Vec::new()
    }
    
    // fn get_all_books(&self)->Vec<Book> {
    //     self.books.values().flat_map(|books| books.iter()).cloned().collect()
    // }

    fn get_stats(&self)->HashMap<String,usize> {
        self.books.iter().map(|(genre, books)| (genre.clone(), books.len())).collect()
    }

    fn write_library(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;

        let genres = self.get_all_genres();

        for genre in genres {
            writeln!(file, "\n{}", genre)?;

            writeln!(file, "===========")?;

            if let Some(books) = self.get_genre(genre) {
                for book in books {
                    writeln!(
                        file,
                        "Name: {}\tAuthor: {}\tPages: {}",
                        book.name, book.author, book.pages
                    )?;
                }
            }
        }

        let stats = self.get_stats();
        for (genre, count) in stats {
            writeln!(file, "{}: {}", genre, count)?;
        }

        Ok(())
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
pub fn book_read_write(){
    println!("This function reads from a txt file, and creates a library and writes it to a file");
    println!("The default direct it's going to read from is from the repo under books.txt");
    println!("Do you want to display the file statistics? (Y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ans = input.trim().to_uppercase();

    let file_stats = match ans.as_str() {
        "Y" => true,
        "N" => false,
        _ => panic!("Invalid input. Please enter Y or N")
    };
    let in_path = "files/books.txt";
    let out_path = "files/Library.txt";

    let (books,stats) = read_input(&in_path,file_stats);

    if file_stats {
        if let Some(statistics) = stats {
            println!("Word count: {}", statistics.get("word_entry").unwrap_or(&0));
            println!("Line count: {}", statistics.get("line_count").unwrap_or(&0));
        }
    };

    let mut main_library = Library::new(String::from("Hell's Bells"));
    main_library = create_library(main_library,books);

    if let Err(e) = main_library.write_library(out_path) {
        eprintln!("Error writing library to file: {}", e);
    }

}

fn read_input(path: &str, file_stats: bool) -> (Vec<String>,Option<HashMap<String, usize>>) {

    let contents = std::fs::read_to_string(path).expect("Should be able to read");
    let books: Vec<String> = contents.split("\n\n").map(|c| c.to_string()).collect();

    if file_stats {
        let stats = file_statistics(&contents);
        (books,Some(stats))
    } else {
        (books,None)
    }
}

fn file_statistics(content:&String)->HashMap<String, usize> {
    let (mut line_count, mut word_count) = (0, 0);
    // let lines: usize = content.split('\n').collect::<Vec<_>>().len();

    for line in content.split('\n') {
        line_count+=1;
        word_count += line.split_whitespace().count();
    }

    let mut stats = HashMap::new();
    stats.insert("word_entry".to_string(), word_count);
    stats.insert("line_count".to_string(), line_count);
    stats
}

fn create_library(mut library:Library,books: Vec<String>)->Library {

    // books_list:Vec<Book> = Vec::new();

    for book in books {
        let book_deets:Vec<&str> = book
        .split('\n')
        .filter_map (|c| {
            let parts: Vec<&str> = c.trim().split(':').collect();
            if parts.len() >1 {
                Some(parts[1].trim())
            } else {
                None
            }
        }).collect();

        // this isn't allowed for some reason of deref trait and all
        // let (name,author,pages,genre) = book_deets;
        if book_deets.len() == 4 {
            let name = book_deets[0].clone().to_string();
            let author = book_deets[1].clone().to_string();
            let pages = book_deets[2].clone();
            let genre = book_deets[3].clone().to_string();

            let t = Book {
            name: name,
            author: author,
            pages: pages.parse().unwrap(),   
            };

        library.add_book(t,genre);
        }


    }
    library

} //reads values from the file to create a genre of books
