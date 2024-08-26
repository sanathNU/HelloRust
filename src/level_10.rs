use std::io;
use std::sync::{Arc,Mutex};
use std::thread;
use std::fs::File;

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

// 🟩public wrapper function
pub fn reading_integers(path:&str)->std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // let ints:Vec<i32> = 
    Ok(())
}