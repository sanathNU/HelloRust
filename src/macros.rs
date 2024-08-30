// Basic macro definition
macro_rules! BasicMacro {
    ($expression:expr) => {
         println!("This is the string,{}",$expression);
        };
}

// Macro with Conditional Patterns
macro_rules! ifmacro {
    ($x:expr) => {
        println!("This is the base single case, the value is this: {}",$x)
    };
    ($x:expr,$y:expr) => {
        println!("Secondary case, more exprs needed. The first is {}",$x);
        println!("the second string is {}",$y);
    }
}
// Macro with Repetition
// Macro with Variable Arguments
// Simple Struct & Method Generation
// Struct Field Initilzation
// Macro with Syntax Checks
// Implemnting traits via macros


// function to test macros
// this is just a remeniscent of the file I used to debug this code
// fn main() {

// }