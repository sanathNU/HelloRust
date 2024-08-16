use std::io::{self, Write, Read};
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::prelude::*;

use HelloRust::level_1; // Adjust this to match the module path in your crate

// Function to capture stdout output
fn capture_stdout_output<F>(func: F) -> String
where
    F: FnOnce(),
{
    // Create a buffer to capture stdout
    let mut buffer = Cursor::new(Vec::new());
    
    // Backup the current stdout
    let old_stdout = io::stdout();
    let old_stdout_handle = old_stdout.lock();

    // Redirect stdout to our buffer
    let mut new_stdout = buffer.clone();
    let new_stdout_handle = Arc::new(Mutex::new(new_stdout));
    
    // Redirect stdout
    {
        let handle = Arc::clone(&new_stdout_handle);
        std::panic::set_hook(Box::new(move |_| {
            let mut handle = handle.lock().unwrap();
            let _ = handle.write_all(b"panic occurred");
        }));
    }

    io::set_output(Box::new(new_stdout_handle.clone()));

    // Call the function that will write to stdout
    func();

    // Restore the old stdout
    io::set_output(Box::new(old_stdout_handle));

    // Convert the captured output to a string
    String::from_utf8(buffer.into_inner()).unwrap()
}

#[test]
fn test_stud_read_write() {
    // Prepare simulated stdin input
    let input = "2\nAlice\n170\n20\nBob\n160\n22\n";
    let simulated_stdin = Cursor::new(input.as_bytes().to_vec());

    // Redirect stdin
    let old_stdin = io::stdin();
    let _stdin_redirect = io::set_input(Box::new(simulated_stdin));

    // Capture stdout and run the function
    let output = capture_stdout_output(|| {
        level_1::stud_read_write();
    });

    // Assertions
    assert!(output.contains("Student Name: Alice"));
    assert!(output.contains("Student Height: 170"));
    assert!(output.contains("Student age: 20"));
    assert!(output.contains("Student Name: Bob"));
    assert!(output.contains("Student Height: 160"));
    assert!(output.contains("Student age: 22"));

    // Restore old stdin
    io::set_input(Box::new(old_stdin));
}
