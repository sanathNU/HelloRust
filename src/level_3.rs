use std::{io, task::ready};
//using the new crate text_io
use text_io;

// student grade calculator functions
fn output_grades(scores:&Vec<f64>,grades:&Vec<String>){
    println!("The average grade of students are:");
    
    for i in 0..grades.len() {
        println!(
            "Student {}: Average score = {:.2}, Final Grade = {}",
            i+1,
            scores[i],
            grades[i]
        );
    }
}
fn calculate_grade(average_scores:&Vec<f64>)->Vec<String> {
    let mut grades:Vec<String> = Vec::new();

    for score in average_scores {
        match score {
            score if *score >= 90.0 => grades.push( String::from("A")),
            score if *score >= 80.0 => grades.push(String::from("B")),
            score if *score >= 70.0 => grades.push(String::from("C")),
            score if *score >= 60.0 => grades.push(String::from("D")),
            score if *score <= 50.0 => grades.push(String::from("F")),
            _ => panic!("Unexpected score:{}]",score)
        }
    }
    grades
}
fn calculate_average_score(scores: &[i32])->f64{
    let sum: i32 = scores.iter().sum();
    let count = scores.len() as f64;
    (sum as f64)/count
}

fn input_scores(students:i32)-> Vec<Vec<i32>>{
    let mut student_scores:Vec<Vec<i32>> = Vec::new();

    for i in 1..students+1{
        println!("\nPlease enter the scores for student {} in the following subjects in whitespace seperated values ",i);
        println!("Physics,Chemistry,Maths, Finance");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let scores: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse input as i32"))
            .collect();

        student_scores.push(scores);
    }
    student_scores
}

// 🟩public wrapper function
pub fn student_grade_calculator() {
    println!("Welcome to the Student Grade Calculator!");
    println!("Enter no of students:");
    let students_n:i32 = text_io::read!();
    //total of 4 assignments  and N students
    let mut student_scores: Vec<Vec<i32>> = input_scores(students_n);

    let mut average_scores:Vec<f64> = Vec::new();
    //avoided giving the control to the borrow checker
    for i in student_scores {
        average_scores.push(calculate_average_score(&i));
    }
    let mut grades:Vec<String> = calculate_grade(&average_scores);
    output_grades(&average_scores, &grades);
}

//functions to calculcate quadratic roots of equation
// 🟩public wrapper function
pub fn quadratic_eqn(){
    println!("Quadratic equations are of mathematical form ax^2+bx+c = 0");
    println!("Enter the values of a,b,c in single line");

    // let (a,b,c):(f64,f64,f64) = text_io::read!("{} {} {}");
    //tried to use multiple values in read! macro, but it isn't allowed: https://docs.rs/text_io/latest/text_io/

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut var_mut = input.split_whitespace();

    // this part is not elegant, but hey, i'm learning
    let a:f64=var_mut.next().expect("Not enough input numbers").parse().expect("Input is not a number");
    let b:f64=var_mut.next().expect("Not enough input numbers").parse().expect("Input is not a number");
    let c:f64=var_mut.next().expect("Not enough input numbers").parse().expect("Input is not a number");

    let (sol1,sol2) = calc_sol(a,b,c);
    println!("The solutions of quadratic eqn {}x^2+ {}x+ {} are: x1 = {}, x2 = {}",a,b,c, sol1, sol2);
}

fn calc_sol(a: f64,b: f64,c: f64)->(f64,f64){
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        println!("The equation has no real solutions.");
        return (f64::NAN, f64::NAN); // Not a Number (NaN) to indicate no real solutions
    }

    let sqrt_discriminant = discriminant.sqrt();
    let p = (-b + sqrt_discriminant) / (2.0 * a);
    let q = (-b - sqrt_discriminant) / (2.0 * a);

    (p,q)
}

// function to calculate factorial of a number using recursion
// - not handling the edge case of negative numbers are given (but it should since any value before 1 should give 1)

// 🟩public wrapper function
pub fn find_factorial() {
    println!("Enter the N to which find the factorial:");
    let n:u32 = text_io::read!();
    let fact:u32 = factorial(n);
    println!("The Factorial of {} is {}",n,fact);
}
//caveats, this doesn't handle a buffer overflow condition if factorial greater than u32 MAX
fn factorial(i:u32)-> u32{
    if i<=1{
        return i;
    } else {
        return i*factorial(i-1)
    }
}
// 🟩public function for 2 problems at once
// function to print the first N natural numbers using a loop
pub fn print_natural_numbers() {
    println!("Simple Function to print N natural numbers using loop");
    println!("Do you want to print the numbers in reverse? (yes/no):");
    let rev:String = text_io::read!();

    println!("Enter N");
    let n:u32 = text_io::read!();

    match rev.as_str() {
        "no" => {
            let mut count:u32 = 0;
            loop {
                print!("{}\t",count);
                if count%50==0 {println!("\n") }
                count+=1;
                if count>n {break};
            }
        },
        "yes" => {
            let mut count:u32 = n;
            while count>0{
                print!("{}\t",count);
                if count%50==0 {println!("\n") };//breaking for brevity 
                count-=1;
            }
        },
        _ => println!("Invalid input. Please enter 'yes' or 'no'.")
    }
}

// 🟩public function for average of N numbers using Rust's iterators
pub fn average_n(){
    println!("This function calculates average upto N numbers using rust's iterators\nEnter the number N");
    let n:i32 = text_io::read!("{}");
    
    let summi:i32 = (1..=n).sum();
    let average = summi as f64/ n as f64;
    println!("The average of N:{} numbers is {}",n,average);
}
