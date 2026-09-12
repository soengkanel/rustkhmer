use std::io::{self, Write};
mod ass1_calculation;
mod ass2_receiveage;

fn main() {
    loop {
        println!("\n============= ASSIGNMENT MENU =============");
        println!("1. Calculation");
        println!("2. Receive Age");
        println!("3. Exit");
        print!("Please enter your choice (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice_input = String::new();
        io::stdin()
            .read_line(&mut choice_input)
            .expect("Failed to read input");

        let choice: i32 = choice_input.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                ass1_calculation::main();
            }
            2 => {
                ass2_receiveage::main();
            }
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice! Please select a number from 1 to 3.");
                continue; 
            }
        }

        print!("\nDo you want to continue? (y/n): ");
        io::stdout().flush().unwrap();

        let mut continue_input = String::new();
        io::stdin()
            .read_line(&mut continue_input)
            .expect("Failed to read input");

        let answer = continue_input.trim().to_lowercase();
        if answer != "y" && answer != "yes"{
            println!("Goodbye!");
            break; 
        }
    }
}
