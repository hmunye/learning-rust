// By default, Rust has a set of items it brings into scope for every program called prelude
//
// Any type not included in this prelude should be brought into scope explicitly
//
// #![feature(prelude_import)]
// #[prelude_import]
// use std::prelude::rust_2021::*;
//

use rand::{self, Rng};
// Brings io (input/output) library into scope which comes from the std (standard) library
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");

    // The range specified is inclusive on the lower and upper bounds (number between 1 - 100)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess:");

        // By default, all variables in Rust are immutable
        //
        // This creates a mutable variable named guess and binds it to an empty String
        let mut guess = String::new();

        io::stdin()
            // The '&' indicates that the argument is a reference
            // References are also immutable by default
            .read_line(&mut guess)
            // expect() will execute only if the Result variant returned from read_line() is Err
            .expect("Failed to read line");

        // This is shadowing the previous value of guess with a new one
        //
        // Shadowing lets you reuse the guess variable name instead of creating another unique variable
        //
        // This trims any whitespace on either side of the String and trys to parse it into the type
        // specified in the type annotation
        let guess: u32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small. Try again"),
            Ordering::Greater => println!("Too big. Try again"),
            Ordering::Equal => {
                println!("You win");
                println!("You guessed: {guess}");
                println!("Secret number: {secret_number}");
                break;
            }
        }
    }
}
