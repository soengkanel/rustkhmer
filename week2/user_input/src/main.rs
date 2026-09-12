// Import the standard input/output library.
// We need this to receive input from the user.
use std::io;

fn main() {
    // Display the title of the program.
    println!("==== Age Checker ====");

    // Ask the user to enter their age.
    println!("Enter your age: ");

    // Create a mutable String to store the user's input.
    // The input is initially stored as text (String).
    let mut input = String::new();

    // Read a line of input from the keyboard.
    // &mut input allows read_line() to modify the input variable.
    //
    // expect() handles an error if Rust cannot read the input.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Remove unnecessary spaces/newline using trim().
    // Then try to convert the input from String into i32.
    //
    // Result can contain:
    // Ok(value)  -> conversion was successful
    // Err(_)     -> conversion failed, for example "abc"
    let age: Result<i32, _> = input.trim().parse();

    // Check the Result.
    match age {

        // If the input is successfully converted to a number,
        // the number is stored in the variable "value".
        Ok(value) => {

            // Check whether the age is negative.
            // Negative ages are not valid.
            if value < 0 {

                // Display an error message for negative numbers.
                println!("Invalid age! Please enter a positive number.");

            } else {

                // Display the valid age.
                println!("Your age is: {}", value);

                // Check whether the person is 18 or older.
                if value >= 18 {

                    // If age is 18 or more, the person is an adult.
                    println!("You are an adult.");

                } else {

                    // If age is less than 18, the person is a minor.
                    println!("You are a minor.");
                }
            }
        }

        // If the input cannot be converted into a number,
        // for example "hello" or "abc", Result becomes Err.
        Err(_) => {

            // Display an error message instead of crashing the program.
            println!("Invalid input! please enter a number only.");
        }
    }
}