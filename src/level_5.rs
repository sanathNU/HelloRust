use std::{cmp::max, io};
use text_io::read;
use num::complex::Complex;
//using counter for python like counting implementation
use counter::Counter;

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

// case conversion
// 🟩public wrapper function
pub fn case_conversion() {
    println!("Enter the string to be case converted!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let converted: String = input
        .trim()
        .chars()
        .map(|c| {
            match c {
                c if c.is_lowercase() => c.to_uppercase().next().unwrap(),
                c if c.is_uppercase() => c.to_lowercase().next().unwrap(),
                _ => c,
            }
        })
        .collect();

    println!("The converted string of {} is {}",input,converted);
}

// **Anagram Checker** . Use counter::Counter, simple but works?
// 🟩public wrapper function
pub fn anagram_checker() {
    println!("This function checks if two given strings are anagrams or not! An anagram is another word formed by keeping the letters of the former");
    println!("Examples are 'spear' and 'pears' ");
    println!("Enter the two strings in 2 seperate lines");

    let word1:String = read!();
    let word2:String = read!();

    let char_countsA = word1.chars().collect::<Counter<_>>();
    let char_countsB = word2.chars().collect::<Counter<_>>();


    if char_countsA == char_countsB {
         println!("{} is an anagram of {}",word1,word2);
    }
    else {
        println!("The words are not anagrams");
    }
}

// **Read & Swap**
// 🟩public wrapper function
pub fn read_and_swap() {
    println!("Enter two numbers in a single line seperated by whitespace:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nos:Vec<i32> = input.trim().split_whitespace().map(|c| c.parse::<i32>().expect("value not integers")).collect();

    let (a ,b) = (nos[0],nos[1]);
    println!("The data before swapping are {} & {}\n",a,b);
    
    arithmetic_on_numbers(&a, &b);

    let temp = &b;
    let &b = &a;
    let &a = temp;

    println!("The data after swapping are {} & {}",a,b);
}

//performing arithmetic too
fn arithmetic_on_numbers(a:&i32,b:&i32) {
    println!("Simple artimetic functions");
    println!("Addition:{}\n Subtraction:{} \n Multiplication:{} \n Division:{}",*a+*b,*a-*b,*a**b,a/b);
}

// **Longest Common substring**
// // 🟩public wrapper function (Solid leetcode problem lol)
// pub fn longest_common_substring() {
//     println!("Enter two strings A & B to find the longest common subsequence");

//     let mut word1:String = read!();
//     let mut word2:String = read!();

//     //naive approach using recursion
//     let max_len = lcs(&word1,&word2,word1.len() as i32,word2.len() as i32);
    
//     println!("The longest common substring is {}",max_len);
// }
// fn lcs(word1:&String, word2:&String,m:i32,n:i32)->i32 {
    
//     if m==0 || n==0 {
//         return 0
//     }
//     else if word1[m-1]==word2[n-2] {
//         return 1+lcs(word1,word2,m-1,n-1);
//     }
//     else {
//         return max(lcs(word1,word2,m,n-1), lcs(word1,word2,m-1,n));
//     }
// }

// 🟩public wrapper function
pub fn longest_common_substring() {
    println!("Enter two strings A & B to find the longest common substring");

    let mut word1 = String::new();
    let mut word2 = String::new();

    // Read inputs
    std::io::stdin().read_line(&mut word1).expect("Failed to read line");
    std::io::stdin().read_line(&mut word2).expect("Failed to read line");

    let word1 = word1.trim();
    let word2 = word2.trim();

    // Call the function to find the length of the longest common substring
    let max_len = lcs_length(word1, word2);
    
    println!("The length of the longest common substring is {}", max_len);
}

fn lcs_length(word1: &str, word2: &str) -> usize {
    let m = word1.len();
    let n = word2.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];
    let mut max_len = 0;

    for i in 1..=m {
        for j in 1..=n {
            if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                max_len = max_len.max(dp[i][j]);
            }
        }
    }

    max_len
}