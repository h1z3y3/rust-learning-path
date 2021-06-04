// This challenge involves handling error that result in reading file from a computer.

// TODO #1: Expects you to handle both the success and failure scenarios inside `match` expression.
// The `Ok(value)` must provide that inner value, and the `Err(error_value)` must be returned early from the `read_file_contents` function.

// TODO #2: Expects you to handle the `Err` case exactly as you've done in TODO #1 exercise.

// TODO #3: Expects you to return the modified `String` inside an `Ok` variant, which expresses that it's expected successful output of the function

use std::path::PathBuf;
use std::io::{Error as IoError, Read};
use std::fs::File;

fn read_file_contents(path: PathBuf) -> Result<String, IoError> {

    let mut string = String::new();


    // TODO #1: Handle this match expression
    // -------------------------------------
    // Pass the variable to the `file` variable on success, or
    // return from the function early if it is an error

    // let mut file: File = match File::open(path) {
    //     Ok(file_handler) => todo!(),
    //     Err(io_error) => todo!(),
    // };

    // My work:
    let mut file: File = match File::open(path) {
        Ok(file_handler) => file_handler,
        Err(io_error) => return Err(io_error),
    };


    // TODO #2: Handle this error
    // ---------------------------
    // The success path is already filled in for you.
    // Return early from the function if it is an error

    // match file.read_to_string(&mut string) {
    //     Ok(_) => (),
    //     Err(io_error) => todo!(),
    // };

    // My work:
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    // TODO #3: Return the `string` variable as expected by its function signature.
    // todo!()

    // My work:
    Ok(string)
}




fn main() {
    assert!(read_file_contents(PathBuf::from("src/main.rs")).is_ok());
    assert!(read_file_contents(PathBuf::from("non-existent-file.txt")).is_err());
}
