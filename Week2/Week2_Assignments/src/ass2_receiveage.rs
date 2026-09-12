use std::io::{self, Write};

pub fn main() {
    println!("\n============= Receive Age =============");
    print!("Please enter your age: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let result: Result<i32, _> = input.trim().parse();

   
    match result {
        Ok(age)=> {
            if age >=0 {
                println!("Success! Your age is: {}", age);
            }
            else{
                println!("Please enter a valid age (positive integer).");
            }
        }
        Err(_) => {
            println!("Error: Please enter a valid number.");
        }
    }
}