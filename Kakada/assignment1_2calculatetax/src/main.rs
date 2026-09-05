use std::io;

fn main() {
    println!("===== Salary Tax Calculator =====");

    println!("Enter your salary:");

    let mut salary_input = String::new();

    io::stdin()
        .read_line(&mut salary_input)
        .expect("Failed to read input");

    let salary: f64 = salary_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let tax_rate: f64 = 0.10;

    let tax: f64 = salary * tax_rate;

    let net_salary: f64 = salary - tax;

    println!("\n===== Result =====");
    println!("Salary: ${:.2}", salary);
    println!("Tax Rate: {:.0}%", tax_rate * 100.0);
    println!("Tax: ${:.2}", tax);
    println!("Net Salary: ${:.2}", net_salary);
}
