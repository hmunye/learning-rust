use core::panic;
use std::io::ErrorKind;

// 'Box<dyn Error>' is a 'trait' object which basically means 'any type of error'. This allows any
// 'Err' value to be returned early
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Normally, when a panic occurs, the program 'unwinds', meaning it walks back up the stack
    // to clean up stack frames and local variables
    //
    // To immediatly abort the program instead in release mode, add these lines to the Cargo.toml file
    //
    // [profile.release]
    // panic = 'abort'
    //
    // Memory will instead be cleaned up by the OS
    //

    // panic!("ERROR");

    let _v: Vec<i32> = vec![1; 20];

    // Causes a panic since we are indexing outside the bounds of 'v'
    // let _ = &_v[2054];

    // Use 'Result' enum for recoverable errors
    let _file_error = match std::fs::File::open("not_found.txt") {
        Ok(_) => println!("File found"),
        Err(err) => println!("File not found: {err}"),
    };

    let _file_error = match std::fs::File::open("not_found.txt") {
        Ok(_) => println!("File found"),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match std::fs::File::create("not_found.txt") {
                Ok(file) => println!("File created: {:?}", file.metadata()),
                Err(err) => panic!("Problem creating the file: {:?}", err),
            },
            other => {
                panic!("Problem opening the file: {:?}", other);
            }
        },
    };

    // Without using 'match'. Using closures and the 'unwrap_or_else' method
    let _file_error = std::fs::File::open("not_found.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            std::fs::File::create("not_found.txt").unwrap_or_else(|err| {
                panic!("Problem creating the file: {:?}", err);
            })
        } else {
            panic!("Problem opening the file: {:?}", err);
        }
    });
    
    // Will automatically panic if the result of the operation is the 'Err' variant
    let _file_error = std::fs::File::open("not_found.csv").unwrap();

    // Will automatically panic but with a custom error message
    let _file_error = std::fs::File::open("not_found.csv").expect("Problem opening file");

    // Use '?' operator to propogate errors. '?' is placed after a 'Result' value

    // Will exit the program with a value of '0'
    Ok(())
}
