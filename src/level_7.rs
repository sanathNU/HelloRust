use std::{array, io};
use text_io::read;

// 🟩public wrapper function
pub fn find_factorial() {
    println!("Enter the N to which find the factorial:");
    let n:u32 = text_io::read!();
    let fact:u32 = factorial(n);
    println!("The Factorial of {} is {}",n,fact);
}
//caveats, this doesn't handle a buffer overflow condition if factorial greater than u32 MAX (I think?)
pub fn factorial(i:u32)-> u32{
    if i<=1{
        return i;
    } else {
        return i*factorial(i-1)
    }
}

//create array
// **Linear iterative search
pub fn create_array()->Vec<i32>{
    println!("Enter the elements of the array in a line seperate by whitespace:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let array:Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<i32>().expect("Enter an integer"))
        .collect();
    array

}
// 🟩public wrapper function
pub fn linear_search() {
    println!("Simple array search in unsorted array");
    println!("Enter the elements of the array in a line seperate by whitespace:");

    let array = create_array();

    println!("Enter the elemnt to be searched:");
    let ele:i32 = read!();

    let (ans, pos) = lins(&array,ele);

    if ans {
        println!("Element {} found at position {} in a 1-indexed array",ele,pos.unwrap());
    } else {
        println!("Element {} not found in array",ele);
    }
}

fn lins(array:&Vec<i32>,element:i32)->(bool,Option<usize>) {

    for (index,&value) in array.iter().enumerate() {
        if value == element {
            return (true,Some(index+1))
        }
    }
    (false,None)
}   

// 🟩public wrapper function
pub fn binary_search() {
    println!("This function implements binary search in iterative & recursive ways");
    println!("Provide a sorted array for the binary search");

    let array = create_array();

    println!("Enter the element to be searched:");
    let ele:i32 = read!();

    //both implementations of binary search exists
    // let (found, position) = bs_iterative(&array, ele);
    let (found,position) = bs_recursive(&array, 0, array.len()-1, ele);

    if found {
        println!("Element {} found at position {}", ele, position.unwrap());
    } else {
        println!("Element {} not found in the array", ele);
    }

}
fn bs_iterative(array: &Vec<i32>, element: i32) -> (bool, Option<usize>) {
    let mut left: usize = 0;
    let mut right: usize = array.len() - 1;

    while left <= right {
        let middle: usize = left + (right - left) / 2;

        match array[middle] {
            value if value < element => left = middle + 1,
            value if value > element => right = middle - 1,
            _ => return (true, Some(middle)),
        }
    }
    (false, None)
}
fn bs_recursive(array: &Vec<i32>, left:usize,right:usize,element:i32) -> (bool,Option<usize>) {
    if right>=left {
        let mid = left + (right-left)/2;

        if array[mid]==element { return (true,Some(mid))};
        if array[mid] > element { return bs_recursive(&array, left, mid-1, element)};
        return bs_recursive(array, mid+1, right, element)
    }
    return (false,None)
}