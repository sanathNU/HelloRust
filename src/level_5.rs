use std::io;
use text_io::read;
use num::complex::Complex;

// function to check whether a given string is a palindrome or not
// 🟩public wrapper function
pub fn palindrome_checker() {
    //this function is not case independant
    fn check_palindrome(words:&String)->bool {
        words.chars().eq(words.chars().rev())
    };

    println!("This functions checks whether the given string is a planindrome or not");
    println!("Enter the string to be tested:");
    let input:String = read!();
    let answer:String = match check_palindrome(&input) {
        true => String::from("is"),
        _ => String::from("is not"),
    };
    println!("The string {} {} a palindrome",input,answer);
}

// complex number magnitude calculation function
// 🟩public wrapper function
pub fn complex_number_magnitude() {
    println!("Complex numbers are of the form a+ib, their magnitude is given by \u{221A}a*2+b*2");
    println!("Enter the numbers A and B seperated by a whitespace");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nos:Vec<i32> = input.trim().split_whitespace().map(|c| c.parse::<i32>().expect("value not integers")).collect();
    
    let (a ,b) = (nos[0],nos[1]);
    let no = Complex::new(a as f64, b as f64);
    let magnitude = calc_magnitude(no);

    println!("The magnitude of complex number {}+i{} is {}",a,b,magnitude);
}
//just wrote a function, because I wanted to use complex crate
fn calc_magnitude(no:Complex<f64>)->f64 {
    no.norm()
}