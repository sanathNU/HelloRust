// function to sum the series `1/1! + 4/2! + 27/3! + ...` using recursion.
mod level_2;

fn sumfrac(n:i32)->f64{
    if n==1 { return 1.0;}
    else {
        return sumfrac(n-1)+ (n.pow(n as u32) as f64/factorial(n) as f64);
    }
}