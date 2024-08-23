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