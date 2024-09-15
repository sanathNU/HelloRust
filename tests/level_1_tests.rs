use HelloRust::level_1::*;
// use super::*;
use std::io;
use std::io::{BufRead, Write};

// test module for testing level_1 function
#[cfg(test)]
mod tests {
        #[test]
        fn test_favapps() {
            level_1::favapps();
        }
        
        #[test]
        fn test_area() {
            let circle = Circle::new(3.0);
            assert_eq!(circle.area(), 28.274333882308138);
        }
    }