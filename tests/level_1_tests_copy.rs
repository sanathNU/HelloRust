use std::io::{self, BufRead, BufReader, Cursor, Write};

// Assuming the functions are in a module named `level_1` in the `HelloRust` crate
use HelloRust::level_1;

// use std::io::{BufRead, Write, Cursor};

// Capture the output of a function that uses `BufRead` and `Write`
fn capture_stdout_output<R: BufRead, F>(reader: &mut R, func: F) -> String
where
    F: FnOnce(&mut R, &mut Vec<u8>),
{
    let mut output_buffer = Vec::new();
    func(reader, &mut output_buffer);
    String::from_utf8(output_buffer).unwrap()
}

#[test]
fn test_stud_read_write() {
    // Prepare simulated stdin input
    let input = "2\nChetan\n170\n20\nJanavi\n160\n22\n";
    let simulated_stdin = Cursor::new(input);
    let mut reader = BufReader::new(simulated_stdin);

    // Prepare a buffer to capture stdout
    let output = capture_stdout_output(&mut reader, |r, w| {
        level_1::stud_read_write(r, w);
    });

    // Assertions
    assert!(output.contains("Student Name: Chetan"));
    assert!(output.contains("Student Height: 170"));
    assert!(output.contains("Student age: 20"));
    assert!(output.contains("Student Name: Janavi"));
    assert!(output.contains("Student Height: 160"));
    assert!(output.contains("Student age: 22"));
}
