use text_io::read;
use std::collections::HashMap;

//using create array function from previous level
use crate::level_7::create_array;

// **duplicate elements in array**
// 🟩public wrapper function
pub fn duplicate_elements(){
    println!("Finds dupicate elements in an unsorted array");
    let array = create_array();
    let duplicates = count_duplicates(&array);

    println!("The number of duplicates in this array are {}",duplicates);
}
fn count_duplicates(array:&Vec<i32>)->usize {
    let mut counts = HashMap::new();

    for &item in array {
        *counts.entry(item).or_insert(0)+=1;
    }

    counts.values().filter(|&&count | count >1).count()
}

// **String Length**
// 🟩public wrapper function
//this only calculates for single complete string
pub fn string_length() {
    println!("Enter the string whose length should be checked");
    let string:String = read!();

    let mut count = 0;

    for _ in string.trim().chars() {
        count+=1;
    }
    println!("The total number of characters in this string is {}",count);
}

// **String Concatenation:**  
//   Write a function to concatenate two strings without using Rust’s `+` operator.
fn stringconcat(a:&String,b:&String)->String {
    //old school no ownership bakwas code
    // a+&b
    //using format method? Seems nice
    format!("{}{}",*a,*b)
}
// 🟩public wrapper function
pub fn string_concat() {
    println!("Enter two strings to be concatenated seperated by whitespace");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let nos:Vec<String> = input.trim().split_whitespace().map(|c| c.parse::<String>().expect("value not integers")).collect();
    let (a ,b) = (nos[0].clone(),nos[1].clone());
    let c = stringconcat(&a, &b);

    println!("The concatenated string of {} and {} is {}",a,b,c);
}
// **Array Rotation**
// 🟩public wrapper function
pub fn array_rotation() {
    let mut array = create_array();
    println!("Enter the number of positions to shift  in the array:");
    let n:i32 = read!();

    let turned_array = rotate_array(&array,n.try_into().unwrap());
    println!("The original array is \n{:?}\n and the rotated right array is \n{:?}",array.clone(),turned_array);
    // println!("The rotation can also be done inplace using inbuilt functions");
    // println!("The original array is \n{:?}\n and the rotated right array is \n{:?}",array.clone(),array.rotate_left(n.try_into().unwrap()));
}
fn rotate_array(array:&Vec<i32>,n:usize)->Vec<i32>{

    let len = array.len();
    if len==0 {
        return Vec::new();
    }

    let n = n%len;
    let (right,left) = array.split_at(len-n);
    println!("This is for testing {:?} & {:?}",right,left);

    let mut new_array = Vec::new();
    new_array.extend_from_slice(left);
    new_array.extend_from_slice(right);
    new_array

}
