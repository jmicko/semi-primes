// basic utilities for the program that clutters up other files
use std::io;
use std::io::Write;

const CLEAR: bool = true;

pub fn clear_console() {
    if CLEAR {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }
}
