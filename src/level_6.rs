// function to sum the series `1/1! + 4/2! + 27/3! + ...` using recursion.
use crate::level_7::factorial;
use text_io::read;

// **series sum**
// 🟩public wrapper function
pub fn series_sum(){
    println!("Enter the number of term of the series 1/1!+2^2/2!+3^3/3!.... that you want to calculate");
    let n:u32 = read!();
    let answer = sumfrac(n);
    println!("The total sum of the series is: {:.6}",answer);
}
fn sumfrac(n:u32)->f64{
    if n==1 { return 1.0;}
    else {
        return sumfrac(n-1)+ (n.pow(n as u32) as f64/factorial(n) as f64);
    }
}
//alternative factorial function
// fn factorial(n:u32)->u64{
//     (1.=n).product()
// }

// **Circle data calculation** circumferance, Area, Diameter & Volume of the Sphere
struct Circle {
    radius:f64
}

impl Circle {
    // fn new(&self,radius:f64){};
    fn new(radius:f64)->Circle{
        Circle{radius}
    }
    fn diameter(&self)->f64 {
        &self.radius*2.0
    }
    fn circumference(&self)->f64 {
        2.0 * std::f64::consts::PI * &self.radius
    }
    fn area(&self)->f64 {
        std::f64::consts::PI*&self.radius.powi(2)
    }
    fn volume(&self)->f64 {
        (4.0/3.0)*std::f64::consts::PI*&self.radius.powi(3)
    }
}
// 🟩public wrapper function
pub fn circle_values_calculation(){
    println!("Here we calculate some parameters of a circle");
    println!("Enter the radius of the circle:");

    let radius:f64 = read!();
    let circle= Circle::new(radius);

    println!("Diameter: {:.2}", circle.diameter());
    println!("Circumference: {:.2}", circle.circumference());
    println!("Area: {:.2}", circle.area());
    println!("Volume of Sphere: {:.2}", circle.volume());
}

// **Area of Triangle  using Heron's Forumula
struct Triangle {
    a:f64,
    b:f64,
    c:f64
}
impl Triangle {
    fn new(a:f64,b:f64,c:f64)->Triangle{
        Triangle{a,b,c}
    }
    fn semiperimeter(&self)->f64{
        (&self.a+&self.b+&self.c)/2.0
    }
    fn area(&self)->f64 {
        let s = &self.semiperimeter();
        (s*(s-&self.a)*(s-&self.b)*(s-&self.c)).sqrt()
    }
}
// 🟩public wrapper function
pub fn triangle_area_calculation() {
    println!("Enter the sides of the triangle separated by spaces:");

    // Reading input for sides of the triangle
    let a: f64 = read!();
    let b: f64 = read!();
    let c: f64 = read!();

    // Creating a Triangle instance
    let triangle = Triangle::new(a, b, c);

    // Calculating the area
    let area = triangle.area();

    // Printing the result
    println!("The area of the triangle is: {:.2}", area);
}