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