use std::f64::consts;
use std::io;
use std::io::{BufRead, Write};

//Function to print favorite mobile apps
pub fn favapps() {
    println!("My fav apps are:");
    println!("Spotify\tCommunistTV\tDoordarshan😃");
}

struct Student {
    name: String,
    height: i32,
    age: i32,
}

//Function to calculate student scores
fn stud_read(studs: i32) -> Vec<Student>{

    let mut stud_list = Vec::new();
    let mut input = String::new();
    for i in 0..studs {

        println!("Enter name of student {}",i);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let name1 = input.trim().to_string();
        input.clear();

        println!("Enter height of student {} in cm",i);
        io::stdin().read_line(&mut input).expect("failed to read input");
        let height1:i32 = input.trim().parse().expect("Expected an integer");
        input.clear();

        println!("Enter age of the student {}",i);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let age1:i32 = input.trim().parse().expect("Expected an integer");
        input.clear();

        println!();

        let s = Student{
            name: name1,
            height: height1,
            age: age1,
        };

        stud_list.push(s);

    }
    stud_list
}

fn stud_print(stud_list: Vec<Student>){
    for student in stud_list {
        println!("Student Name: {}", student.name);
        println!("Student Height: {}", student.height);
        println!("Student age: {}\n", student.age);
    }
}

pub fn stud_read_write(){
    println!("Enter the number of students in the group!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let studs: i32 = input.trim().parse().expect("Expected an integer");
    let students = stud_read(studs);
    stud_print(students);

}

// fn stud_read<R: BufRead>(reader: &mut R, studs: i32) -> Vec<Student> {
//     let mut stud_list = Vec::new();
//     let mut input = String::new();
//     for i in 0..studs {
//         writeln!(std::io::stdout(),"Enter the name of student {}",i).unwrap();
//         reader.read_line(&mut input).expect("Failed to read input");
//         let name1 = input.trim().to_string();
//         input.clear();

//         writeln!(std::io::stdout(), "Enter height of student {} in cm", i).unwrap();
//         reader.read_line(&mut input).expect("Failed to read input");
//         let height1: i32 = input.trim().parse().expect("Expected an integer");
//         input.clear();

//         writeln!(std::io::stdout(), "Enter age of the student {}", i).unwrap();
//         reader.read_line(&mut input).expect("Failed to read input");
//         let age1: i32 = input.trim().parse().expect("Expected an integer");
//         input.clear();

//         writeln!(std::io::stdout()).unwrap();
//         let s = Student {
//             name: name1,
//             height: height1,
//             age: age1,
//         };

//         stud_list.push(s);

//     }
//     stud_list
// }

// fn stud_print<W: Write>(writer: &mut W, stud_list: Vec<Student>) {
//     for student in stud_list {
//         writeln!(writer, "Student Name: {}", student.name).unwrap();
//         writeln!(writer, "Student Height: {}", student.height).unwrap();
//         writeln!(writer, "Student age: {}\n", student.age).unwrap();
//     }
// }
// pub fn stud_read_write<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) {
//     writeln!(writer, "Enter the number of students in the group!").unwrap();
//     let mut input = String::new();
//     reader.read_line(&mut input).expect("Failed to read input");
//     let studs: i32 = input.trim().parse().expect("Expected an integer");
//     let students = stud_read(reader, studs);
//     stud_print(writer, students);
// }

// Function to calculate and print the area of a circle
pub fn area_of_circle(){
    let mut input = String::new();
    println!("Enter the radius of the circle for the area to be calculated:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let radius:f64 = input.trim().parse().expect("Expected a floating int");

    let area: f64 = std::f64::consts::PI * radius.powi(2);

    println!("The area of the circle with radius {} is {}",radius,area);
}

// Function to print the ASCII value of a character
pub fn print_ascii_value() {
    let mut input = String::new();

    println!("Enter a single character:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch: char = match input.trim().chars().next() {
        Some(c) => c,
        None => {
            println!("Please enter a valid character.");
            return;
        }
    };

    println!("The ASCII value of '{}' is {}", ch, ch as u8);
}
// Function to convert Fahrenheit to Celsius
pub fn far_to_cel() {
    let mut input = String::new();

    println!("Enter temperature in Fahrenheit:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let fahrenheit: f64 = match input.trim().parse() {
        Ok(f) => f,
        Err(_) => {
            println!("Please enter a valid number for temperature.");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("The temperature in Celsius is {:.2}", celsius);
}

// Add three numbers
pub fn add_three_numbers() {
    println!("Enter the three numbers to be added in a single line");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let sum:i32 = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Enter a number"))
        .sum();
    
    println!("The sum of the numbers is: {}", sum);
}