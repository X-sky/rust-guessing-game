use std::io;

fn main() {
    println!("Guess the number:");
    println!("Please input your guess,");
    
    // tip1. keyword 'mut': in rust, variables are immutable by default, to make them mutable, we need to use the mut keyword
    // tip2. :: syntax is used for associated functions, and an associated function is a function that’s implemented on a type, in this case String
    let mut guess = String::new();
    io::stdin()
    // tip1. like in C++, & indicates that this argument is a reference
    // tip2. like variables, references are immutable by default, so you need to write &mut guess rather than &guess to make it mutable
    // tip3. expect is a method on Result types, which are enumerations used for error handling 0.4ms
        .read_line(&mut guess)
        .expect("Failed to read line");
    // "crab pincers" to hold a value
    println!("You guessed: {guess}");
}