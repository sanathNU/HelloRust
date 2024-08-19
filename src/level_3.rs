use std::io;

// function to calculate factorial of a number using recursion
// - not handling the edge case of negative numbers are given (but it should since any value before 1 should give 1)
fn factorial(i:i32)-> i32{
    if i<=1{
        return i;
    } else {
        return i*factorial(i-1)
    }
}

// function to print the first N natural numbers using a loop
fn natnum(number:i32){
    let mut count = 0;
    loop {
        print!("{}\t",count);
        count+=1;
        if count>number {break};
    }
}
// function to print the numbers in reverse
fn rev100(){
    let mut n = 100;
    while n>=0 {
        print!("{} ",n);
        if n==50 { print!("\n"); }
        n-=1;
    }
}
// function for average of N numbers using Rust's iterators
fn avgn(n:i32)->f64 {
    let summi:i32 = (1..=n).sum();
    summi as f64/ n as f64
}
