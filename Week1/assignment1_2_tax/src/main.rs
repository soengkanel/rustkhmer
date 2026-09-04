use std::io::{self, Write};

fn main() {
    println!("--- Welcome to the Tax Calculator ---");

    print!("Please enter the item name: "); 
    io::stdout().flush().unwrap();          
    let mut item_name = String::new();
    io::stdin()
        .read_line(&mut item_name)
        .expect("Failed to read input");
    let item = item_name.trim();

    print!("Enter the price of the item: "); 
    io::stdout().flush().unwrap();          
    let mut price_input = String::new();
    io::stdin()
        .read_line(&mut price_input)
        .expect("Failed to read input");
    let price: f64 = price_input
        .trim()
        .parse()
        .expect("Please type a valid number!");

    print!("Enter the quantity: ");         
    io::stdout().flush().unwrap();         
    let mut qty_input = String::new();
    io::stdin()
        .read_line(&mut qty_input)
        .expect("Failed to read input");
    let quantity: i32 = qty_input
        .trim()
        .parse()
        .expect("Please type a valid integer!");

    let tax_rate: f64 = 0.03;
    let subtotal: f64 = price * (quantity as f64);
    let tax_amount: f64 = subtotal * tax_rate;
    let total_price: f64 = subtotal + tax_amount;

    
    println!("\n--- Receipt ---");
    println!("Item: {}", item);
    println!("Quantity: {}", quantity);
    println!("Subtotal: ${:.2}", subtotal);
    println!("Sales Tax: ${:.2}", tax_amount); 
    println!("Total Price: ${:.2}", total_price);
}