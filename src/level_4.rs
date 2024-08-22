// extern crate unic_emoji_char;
use std::io;
use text_io::read;
//using new crate!😄
use std::collections::HashMap;
//another one for them emojis!
use unic_emoji_char::is_emoji;
//learnt that 🟩 panics the code whereas 😄 one works


//functions to read Character until a '*' is encountered, and to show a count of all the characters.
// 🟩public wrapper function
pub fn counting_characters(){
    println!("This function counts the total number of characters that are given as input, and gives a character count!");
    println!("Enter the input string with * as a termination character");

    let input:String = read!();

    let characters:Vec<char> = input
        .trim()
        .chars()
        .collect();

    // println!("This is for test {}",is_emoji('😄'));
    let counts:HashMap<String,i32> = countchar(characters);
    //printing the output
    for (char_type,value) in counts.iter() {
        println!("{}:{}",char_type,value);
    }
}

fn countchar(characters: Vec<char>) -> HashMap<String,i32> {
    let (mut upcount, mut lowcount, mut numeric, mut emojii) = (0, 0, 0,0);
    let mut counts:HashMap<String,i32> = HashMap::new();
    for c in characters.iter(){
        match c {
            c if c.is_uppercase() => {upcount+=1},
            c if c.is_lowercase() => {lowcount+=1},
            c if c.is_digit(10) => {numeric+=1},
            c if *c == '*' => {break},
            c if is_emoji(*c) => {emojii+=1}
            _ => panic!("Invalid Character! Please check again!")
            }
    }
    counts.insert(String::from("Uppercase Characters"),upcount);
    counts.insert(String::from("Lowercase Characters"),lowcount);
    counts.insert(String::from("Numeric Digits"),numeric);
    counts.insert(String::from("Emojis"),emojii);

    counts
}

//function to classify a given number as prime or composite
// 🟩public wrapper function
pub fn prime_or_composite() {

    fn primecomp(number:u32)->bool{
        //normal approach
        // for i in 2..number{
        //     if number%i==0 { return false};
        // }
        // true
    
        // a smarter approach
        if number==1 {return false};
        if number==2 { return true};
        if number%2==0 {return false};
    
        let limit = (number as f64).sqrt() as u32;
    
        for i in 3..limit {
            if number%i==0 { return false};
        }
        true
    }

    println!("Function to calculate whether a number is prime or composite");
    println!("Enter the number");
    let number:u32 = read!();

    let answer:String =  match primecomp(number){
        true => String::from("Prime"),
        _ => String::from("Composite"),
    };

    println!("The number:{} is {}",number,answer);
}

//function to calculate sum of squares of first n even numbers
// 🟩public wrapper function
pub fn sum_of_squares(){
    println!("To calculate the sum of squares of first n even numbers");
    println!("Enter the number N");
    let n:i32 = read!();

    //a simple forumla exists
    let answer = (2*n*(n+1)*(2*n+1)/3);

    println!("The sum of squares of the first N:{} even numbers is {}",n,answer);
}

// function to convert decimal number to a binary number
// 🟩public wrapper function
pub fn decimal_to_binary(){
    println!("This converts a decimal number to a binary string");
    println!("Enter the decimal number to be converted");
    let n:i32 = read!();

    if n < 0 {
        println!("Please enter a non-negative integer.");
        return;
    }
    let mut a = n;
    let mut out = String::new();

    while a > 0 {
        let b: char = match a % 2 {
            0 => '0',
            1 => '1',
            _ => unreachable!(), // This line will never be reached
        };
        out.push(b);
        a /= 2; // Update a to be the quotient of the division by 2
    }
    out = out.chars().rev().collect();

    println!("The binary converted number of {} is {}",n,out);
}

// **Euclidean Algorithm** functions
// 🟩public wrapper function
pub fn gcd_calculate() {
    println!("Enter the characters whose in GCD should be calculated in next line seperated by whitespaces");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nos:Vec<i32> = input.trim().split_whitespace().map(|c| c.parse::<i32>().expect("value not integers")).collect();
    
    let (a ,b) = (nos[0],nos[1]);
    let ans = GCD(a,b);

    println!("The GCD of {} and {} is {}", a, b, ans);
}
// this only passes the borrow checkers wrath, because it's Int and not any other data type
// it would be a nightmare to polymorphize this. ( I know the generics will help me do this, but man, i'm lazy)
fn GCD(a:i32,b:i32)->i32 {
    match b {
        0 => return a,
        _ => return GCD(b,a%b)
    }
}