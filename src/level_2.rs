use std::io;

// function to find largest of two numbers using if statement
pub fn largest2(){
    let mut input = String::new();
    println!("Enter the two numbers A & B seperated by spaces:");
    io::stdin().read_line(&mut input).expect("Failed to take input");
    //some new method for parsing two inputs from a single line
    let mut substr_iter = input.split_whitespace();
    let mut next_sum = || -> usize {
        substr_iter.next().expect("Not enough input numbers")
                   .parse().expect("Input is not a number")
    };
    let val1 = next_sum();
    let val2 = next_sum();

    if val1 < val2 {
        println!("{} is greater than {}-",val2,val1);
    } else if val1==val2 {
        println!("both {} & {} are equal",val1,val2);
    } else {
        println!("{} is greater than {}",val2,val1);
    }
}

// function to find minimum of 3 numbers using 2 methods, (match & Comp )
pub fn min3(){
    println!("This function compares 3 numbers and finds it's minimum");
    let mut input = String::new();
    println!("Enter 3 numbers A, B & C in single line seperated by space");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut substr_iter = input.split_whitespace();
    let mut next_sum = || -> usize {
        substr_iter.next().expect("Enough enough input numbers")
                   .parse().expect("Input is not a number")
    };
    let val1 = next_sum();
    let val2 = next_sum();
    let val3 = next_sum();

    match (val1,val2,val3) {
        (x,y,z) if x==y && y==z => {
            println!("The minimum using match is {}",x);
        }
        (x,y,z) if x<=y && x<=z => {
            println!("the minimum using match  is {}",x);
        }
        (x,y,z) if y<=x && y<=z => {
            println!("The minimum using match is {}",y);
        }
        (x,y,z) if z<=x && z<=y => {
            println!("The minimum using match is {}",z);
        }
        _ => {
            println!("Uncatched condition");
        }
    };
    // alternative method
    let min_val = std::cmp::min(std::cmp::min(val1,val2),val3);
    println!("the min val using std::comp is {}",min_val);
}

// function to calculate vote-eligibility (which is 18 in India)
fn voteeligible() -> bool {
    println!("Enter age");
    let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Enter right value!");
     let age:i32 =input.trim().parse().expect("Enter an integer!");
                
    age>=18
}

// 🟩public wrapper function
pub fn calculate_vote_eligibility() {
    println!("This is a vote eligibility testing function!");
    println!("Voting is the base of every democracy!");
    let answer = voteeligible();
    match answer {
        true => println!("The person is eligible to vote!"),
        false => println!("The person is not elibile to vote!"),
    };
}
// function to check if a character is a vowel or not
fn vowelornot(chaar:char){
    let c = chaar.to_ascii_lowercase();
    let d = matches!(c, 'a' | 'e' | 'i'| 'o' | 'u');
    if d {
        println!("'{}' is a vowel.",c);
    } else if c.is_alphabetic() {
        println!("'{}' is a consonant",c);
    } else {
        println!("'{}' is not a letter",c);
    }
}

// 🟩public wrapper function
pub fn vowel_or_consonant(){
    println!("This is used to check whether a given character in the English language\nis  a vowel or a consonant!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let c:char = input.trim().parse().expect("Enter only a character!");
    vowelornot(c);
}

// function to print multiplication table of given number upto 10
fn print_mult(n: i32){
    println!("The multiplication table of {} is  as follows",n);
    let mut count = 0;
    loop {
        count +=1;
        if count == 11 {break};
        println!("{} x {} = {}",n,count, n*count);
    }
}
// 🟩public wrapper function
pub fn multiplication_table() {
    println!("Here we calculate the multiplication table of a given number n");
    println!("Enter the number N");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to receive input");
    let n:i32 = input.trim().parse().expect("Input a number N");

    //passing to the function
    print_mult(n);

}