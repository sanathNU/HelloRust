use std::io;

//functions to calculcate quadratic roots of equation
fn xinput(){
    println!("Quadratic equations are of mathematical form ax^2+bx+c = 0");
    println!("Enter the values of a,b,c in single line");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut var_mut = input.split_whitespace();

    // this part is not elegant, but hey, i'm learning
    let a:f64=var_mut.next().expect("Not enough input numbers").parse().expect("Input is not a number");
    let b:f64=var_mut.next().expect("Not enough input numbers").parse().expect("Input is not a number");
    let c:f64=var_mut.next().expect("Not enough input numbers").parse().expect("Input is not a number");

    let (sol1,sol2) = calc_sol(a,b,c);
    println!("The solutions of quadratic eqn {}x^2+ {}x+ c are: x1 = {}, x2 = {}",a,b,c, sol1, sol2);
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
