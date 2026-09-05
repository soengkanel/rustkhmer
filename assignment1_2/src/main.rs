use std::io;
fn main(){
    println!("===== Temperature Converter =====");
    println!("Enter temperature in Celsius: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");

    let celsius: f64 = input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let fahrenheit: f64 = (celsius * 9.0 / 5.0) + 32.0;

    println!("Celsius: {:.1} °C", celsius);
    println!("Fahrenheit: {:.1} °F", fahrenheit);
    
}