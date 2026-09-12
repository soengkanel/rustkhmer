// Create a function named calculate_sum.

// limit: i32 means the function recieves an integer.
pub fn calculate_sum(limit: i32) -> i32 { 
    // Create a mutable variable called sum. We start the total at 0
    let mut sum = 0;

    // Create a for loop. 1..=limit means start from 1 and continue until the limit.
    for number in 1..=limit {
        // Check wheter the number is even. If number % 2 == 0 -> the number is divided by 2 without remainder.
        if number % 2 == 0 {
            sum += number; // Add the even number to sum
        } else {
            sum += number; // Add the odd number to sum. 
        }
    }
    // Return the result
    sum
}

fn main() {
    let result = calculate_sum(8); // Call the calculate_sum() function and give it the value 8 as the limit.

    println!("Total: {}", result);
}