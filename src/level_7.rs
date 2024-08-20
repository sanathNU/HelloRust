// function to calculate GCD of two numbers using recursion
// this only passes the borrow checkers wrath, because it's Int and not any other data type
// it would be a nightmare to polymorphize this.
fn gcd(a:i32,b:i32)->i32 {
    if b==0 { return a;}
    return gcd(b,a%b);
}