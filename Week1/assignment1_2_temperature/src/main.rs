use std::io::{self, Write};

fn main() {
    println!("--- Welcome to the Temperature Converter ---");

    print!("Please enter the name of your city: "); 
    io::stdout().flush().unwrap();          
    let mut city_name = String::new();
    io::stdin()
        .read_line(&mut city_name)
        .expect("Failed to read input");
    let city = city_name.trim();

    println!("\nConversion Options:\n1. C to F\n2. F to C\n(Press any other key to exit)");
    print!("Please enter your choice: ");
    io::stdout().flush().unwrap();
    let mut opt = String::new();
    io::stdin()
        .read_line(&mut opt)
        .expect("Failed to read input");
    let option: i32 = opt.trim().parse().unwrap_or(0);

    if option == 1 {
        println!("\nYou have selected C to F conversion for {city}.");
        print!("Enter the temperature in Celsius: ");
        io::stdout().flush().unwrap();          
        let mut temp_input = String::new();
        io::stdin()
            .read_line(&mut temp_input)
            .expect("Failed to read input");
        let temp_celsius: f64 = temp_input
            .trim()
            .parse()
            .expect("Please type a valid number!");

        let temp_fahrenheit: f64 = (temp_celsius * 9.0 / 5.0) + 32.0;
        println!("The temperature in Fahrenheit is: {:.2}°F", temp_fahrenheit);           
    } else if option == 2 {
        println!("\nYou have selected F to C conversion for {city}.");
        print!("Enter the temperature in Fahrenheit: "); 
        io::stdout().flush().unwrap();         
        let mut temp_input = String::new();
        io::stdin()
            .read_line(&mut temp_input)
            .expect("Failed to read input");
        let temp_fahrenheit: f64 = temp_input
            .trim()
            .parse()
            .expect("Please type a valid number!");

        let temp_celsius: f64 = (temp_fahrenheit - 32.0) * 5.0 / 9.0;
        println!("The temperature in Celsius is: {:.2}°C", temp_celsius);
    } else {
        println!("Invalid option. Please enter 1 or 2.");
        return;
    }
}