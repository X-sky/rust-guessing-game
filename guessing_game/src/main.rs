use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number:");

    // tip1. unless specified, numbers in Rust defaults to i32, also can be i8, i64, u32, etc.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
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

        // tip1. Rust allows to shadow the previous value of `guess` with a new one instead of creating two unique variables.
        // tip2. By using `: u32`, we tell Rust exactly what type we want, and Rust will convert String to another type with `parse()`
        // tip3. add keyword `match` and arms to handle the Result type returned by parse
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // "crab pincers{}" to hold a value
        println!("You guessed: {guess}");

        // tip1. match expression is made up of "arms" which consists of a pattern to match against
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
