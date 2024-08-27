use core::fmt;
use std::sync::{Arc,Mutex};
use std::thread;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::io;

// function to use threads for calculating factorial of a number
fn factorial(n:u32) -> u32 {
    (1..=n).product()
}
// 🟩public wrapper function
pub fn multiple_factorial() {
    // let numbers = vec![5,6,7,8,9];
    println!("Enter the numbers whose factorial should be calculated in a whitespace seperated string");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers:Vec<u32>= input.trim().split_whitespace().map(|c| c.parse::<u32>().expect("expected a number")).collect();
    let results = Arc::new(Mutex::new(vec![0; numbers.len()]));

    let mut handles = vec![];

    for (i,&num) in numbers.iter().enumerate() {
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let result = factorial(num);
            let mut results = results.lock().unwrap();
            results[i] = result;
        });

        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    for (i,&num) in numbers.iter().enumerate() {
        println!("Factorial of {} if {}",num, results[i]);
    }

}

#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(String),
}
impl fmt::Display for AppError {
    fn fmt(&self, f:&mut fmt::Formatter<!_>)-> fmt::Result {
        match self {
            AppError::IoError(err) => write!(f,"IO error: {}",err),
            AppError::ParseError(ref msg) => write!(f,"Parse error: {}",msg),
        }
    }
}
impl Error for AppError{}

impl From<io::Error> for AppError {
    fn from(err:io::Error) -> Self {
        AppError::IoError(err)
    }
}
// 🟩public wrapper function
pub fn reading_integers(path:&str)->Result<Vec<i32>, AppError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut integers = Vec::new();
    for line in reader.lines() {
        let line = line?;
        for token in line.split_whitespace(){
            match token.parse::<i32>() {
                Ok(num) => integers.push(num),
                Err(_) => return Err(AppError::ParseError(format!("Invalid Integer: {}",token))),
                }
            }
        ;}
    Ok(integers)
}