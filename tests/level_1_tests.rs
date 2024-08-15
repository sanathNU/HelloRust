// // tests/level_1_tests.rs

// use HelloRust::level_1;
// use std::io::{self, Write};
// use std::str::FromStr;
// use std::io::Cursor;

// #[test]
// fn test_favorite_mobile_apps() {
//     //fav apps test
//     level_1::favapps();
// }

// #[test]
// fn test_stud_read_write() {
//     let input = "2\nChetan\n170\n20\nJanavi\n160\n22\n";
//     let mut stdin = io::stdin();
//     let old_stdin = stdin.lock().set_read_content(input);

//     let mut buffer = Cursor::new(Vec::new());
//     {
//         let old_stdout = io::stdout();
//         io::set_panic_hook(Box::new(move |info| {
//             let _ = old_stdout.write_all(&buffer.into_inner());
//         }));

//         level_1::stud_read_write();
//     }

//     let output = String::from_utf8(buffer.into_inner()).unwrap();
//     assert!(output.contains("Student Name: Chetan"));
//     assert!(output.contains("Student Height: 170"));
//     assert!(output.contains("Student age: 20"));
//     assert!(output.contains("Student Name: Janavi"));
//     assert!(output.contains("Student Height: 160"));
//     assert!(output.contains("Student age: 22"));
// }

// fn test_area_of_circle() {
//     let input = "5\n";
//     let mut stdin = io::stdin();
//     let old_stdin = stdin.lock().set_read_content(input);

//     let mut buffer = Cursor::new(Vec::new());
//     {
//         let old_stdout = io::stdout();
//         io::set_panic_hook(Box::new(move |info| {
//             let _ = old_stdout.write_all(&buffer.into_inner());
//         }));

//         level_1::area_of_circle();
//     }

//     //this doesn't check the output computation, but just checks whether the output is computed or not
//     let output = String::from_utf8(buffer.into_inner()).unwrap();
//     assert!(output.contains("The area of the circle with radius 5 is"));
// }

// #[test]
// fn test_print_ascii_value() {
//     let input = "A\n";
//     let mut stdin = io::stdin();
//     let old_stdin = stdin.lock().set_read_content(input);

//     let mut buffer = Cursor::new(Vec::new());
//     {
//         let old_stdout = io::stdout();
//         io::set_panic_hook(Box::new(move |info| {
//             let _ = old_stdout.write_all(&buffer.into_inner());
//         }));

//         level_1::print_ascii_value();
//     }

//     let output = String::from_utf8(buffer.into_inner()).unwrap();
//     assert!(output.contains("The ASCII value of 'A' is 65"));
// }

// #[test]
// fn test_far_to_cel() {
//     // not testing any edge cases here, just some basic functionality
//     let input = "32\n";
//     let mut stdin = io::stdin();
//     let old_stdin = stdin.lock().set_read_content(input);

//     let mut buffer = Cursor::new(Vec::new());
//     {
//         let old_stdout = io::stdout();
//         io::set_panic_hook(Box::new(move |info| {
//             let _ = old_stdout.write_all(&buffer.into_inner());
//         }));

//         level_1::far_to_cel();
//     }

//     let output = String::from_utf8(buffer.into_inner()).unwrap();
//     assert!(output.contains("The temperature in Celsius is 0.00"));
// }