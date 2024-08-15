// use std::io::{self, Write};
// use std::io::Cursor;
// use std::str;
// use std::fs::File;
// use std::io::prelude::*;

// use HelloRust::level_1; // Adjust this to match the module path in your crate

// fn simulate_stdin_input(input: &str) {
//     let mut stdin = io::stdin();
//     let old_stdin = stdin.lock();

//     let mut simulated_input = Cursor::new(input);
//     io::set_panic_hook(Box::new(move |info| {
//         let _ = old_stdin.read_to_string(&mut simulated_input);
//     }));
// }

// fn capture_stdout_output<F>(func: F) -> String
// where
//     F: FnOnce(),
// {
//     let mut buffer = Cursor::new(Vec::new());
//     {
//         let old_stdout = io::stdout();
//         io::set_panic_hook(Box::new(move |info| {
//             let _ = old_stdout.write_all(&buffer.into_inner());
//         }));

//         func();
//     }

//     String::from_utf8(buffer.into_inner()).unwrap()
// }


// #[test]
// fn test_stud_read_write() {
//     let input = "2\nAlice\n170\n20\nBob\n160\n22\n";
//     simulate_stdin_input(input);

//     let output = capture_stdout_output(|| {
//         level_1::stud_read_write();
//     });

//     assert!(output.contains("Student Name: Alice"));
//     assert!(output.contains("Student Height: 170"));
//     assert!(output.contains("Student age: 20"));
//     assert!(output.contains("Student Name: Bob"));
//     assert!(output.contains("Student Height: 160"));
//     assert!(output.contains("Student age: 22"));
// }