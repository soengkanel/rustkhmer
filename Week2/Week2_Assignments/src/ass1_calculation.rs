use std::io::{self, Write};
fn input_number() -> (i32, i32){
    print!("\nPlease enter the start number: ");
    io::stdout().flush().unwrap();
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).expect("Failed to read input.");
    let start_num: i32 = start_input.trim().parse().expect("Invalid input! Please enter a valid integer.");

    print!("Please enter the end number: ");
    io::stdout().flush().unwrap();
    let mut end_input = String::new();
    io::stdin().read_line(&mut end_input).expect("Failed to read input.");
    let end_num: i32 = end_input.trim().parse().expect("Invalid input! Please enter a valid integer.");
    
    (start_num, end_num)
}

fn type_of_number()->i32{
    println!("\nFor now, please choose which option you want:");
    println!("1. Just Calculate even numbers");
    println!("2. Calculate odd numbers");
    println!("3. Calculate as a whole from start to end");
    print!("Please enter your choice (1-3): ");
    io::stdout().flush().unwrap();
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read input.");
   
    choice_input.trim().parse().expect("Invalid input! Please input the correct above number.")
}

fn sum_even_numbers(start: i32, end: i32) -> i32 {
    let mut total_sum = 0;
    if start <= end {
        for number in start..=end {
            if number % 2 == 0 {
                total_sum += number;
            }
        }
    } else {
        for number in (end..=start).rev() {
            if number % 2 == 0 {
                total_sum += number;
            }
        }
    }
    total_sum
}

fn sum_odd_numbers(start: i32, end: i32) -> i32 {
    let mut total_sum = 0;
    if start <= end {
        for number in start..=end {
            if number % 2 != 0 {
                total_sum += number;
            }
        }
    } else {
        for number in (end..=start).rev() {
            if number % 2 != 0 {
                total_sum += number;
            }
        }
    }
    total_sum
}

fn sum_all_numbers(start: i32, end: i32) -> i32 {
    let mut total_sum = 0;
    if start <= end {
        for number in start..=end {
            total_sum += number;
        }
    } else {
        for number in (end..=start).rev() {
            total_sum += number;
        }
    }
    total_sum
}

fn subtract_even_numbers(start: i32, end: i32) -> i32 {
    if start <= end {
        let actual_start = if start % 2 == 0 { start } else { start + 1 };
        let mut total_difference = actual_start;
        for number in (actual_start + 1)..=end {
            if number % 2 == 0 {
                total_difference -= number;
            }
        }
        total_difference
    } else {
        let actual_start = if start % 2 == 0 { start } else { start - 1 };
        let mut total_difference = actual_start;
        for number in (end..actual_start).rev() {
            if number % 2 == 0 {
                total_difference -= number;
            }
        }
        total_difference
    }
}

fn subtract_odd_numbers(start: i32, end: i32) -> i32 {
    if start <= end {
        let actual_start = if start % 2 != 0 { start } else { start + 1 };
        let mut total_difference = actual_start;
        for number in (actual_start + 1)..=end {
            if number % 2 != 0 {
                total_difference -= number;
            }
        }
        total_difference
    } else {
        let actual_start = if start % 2 != 0 { start } else { start - 1 };
        let mut total_difference = actual_start;
        for number in (end..actual_start).rev() {
            if number % 2 != 0 {
                total_difference -= number;
            }
        }
        total_difference
    }
}

fn subtract_all_numbers(start: i32, end: i32) -> i32 {
    let mut total_difference = start;
    if start <= end {
        for number in (start + 1)..=end {
            total_difference -= number;
        }
    } else {
        for number in (end..start).rev() {
            total_difference -= number;
        }
    }
    total_difference
}

fn multiply_even_numbers(start: i32, end: i32) -> i32 {
    let mut total_product = 1;
    if start <= end {
        for number in start..=end {
            if number % 2 == 0 {
                total_product *= number;
            }
        }
    } else {
        for number in (end..=start).rev() {
            if number % 2 == 0 {
                total_product *= number;
            }
        }
    }
    total_product
}

fn multiply_odd_numbers(start: i32, end: i32) -> i32 {
    let mut total_product = 1;
    if start <= end {
        for number in start..=end {
            if number % 2 != 0 {
                total_product *= number;
            }
        }
    } else {
        for number in (end..=start).rev() {
            if number % 2 != 0 {
                total_product *= number;
            }
        }
    }
    total_product
}

fn multiply_all_numbers(start: i32, end: i32) -> i32 {
    let mut total_product = 1;
    if start <= end {
        for number in start..=end {
            total_product *= number;
        }
    } else {
        for number in (end..=start).rev() {
            total_product *= number;
        }
    }
    total_product
}

fn divide_even_numbers(start: i32, end: i32) -> f64 {
    if start <= end {
        let actual_start = if start % 2 == 0 { start } else { start + 1 };
        
        if actual_start == 0 && actual_start == end {
            return f64::NAN;
        }

        let mut total_quotient = actual_start as f64;
        for number in (actual_start + 1)..=end {
            if number % 2 == 0 && number != 0 {
                total_quotient /= number as f64;
            }
        }
        total_quotient
    } else {
        let actual_start = if start % 2 == 0 { start } else { start - 1 };
        
        if actual_start == 0 && actual_start == end {
            return f64::NAN;
        }

        let mut total_quotient = actual_start as f64;
        for number in (end..actual_start).rev() {
            if number % 2 == 0 && number != 0 {
                total_quotient /= number as f64;
            }
        }
        total_quotient
    }
}

fn divide_odd_numbers(start: i32, end: i32) -> f64 {
    if start <= end {
        let actual_start= if start % 2 != 0 { start } else { start + 1 };

        let mut total_quotient = actual_start as f64;
        for number in (actual_start + 1)..=end {
            if number % 2 != 0 && number != 0 {
                total_quotient /= number as f64;
            }
        }
        total_quotient
    } else {
        let actual_start = if start % 2 != 0 { start } else { start - 1 };

        let mut total_quotient = actual_start as f64;
        for number in (end..actual_start).rev() {
            if number % 2 != 0 && number != 0 {
                total_quotient /= number as f64;
            }
        }
        total_quotient
    }
}

fn divide_all_numbers(start: i32, end: i32) -> f64 {
    if start == 0 && end == 0{
            return f64::NAN;
        }
    
    let mut total_quotient = start as f64;
    if start <= end {
        for number in (start + 1)..=end {
            if number != 0 {
                total_quotient /= number as f64;
            }
        }
    } else {
        for number in (end..start).rev() {
            if number != 0 {
                total_quotient /= number as f64;
            }
        }
    }
    total_quotient
}

fn type_of_operation(){
    println!("There are four operations for you to choose as follows:");
    println!("1. Sum");
    println!("2. Substract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("Exit");
    print!("Please enter your choice (1-4): ");
    io::stdout().flush().unwrap();
    let mut operation_choice_input = String::new();
    std::io::stdin().read_line(&mut operation_choice_input).expect("Failed to read input.");
    let operation_choice: i32 = operation_choice_input.trim().parse().expect("Invalid input! Please input the correct above number.");
    
    if operation_choice >= 1 && operation_choice <= 4 {
        let (start_num, end_num) = input_number();
        let choice = type_of_number();
        if operation_choice == 1 {
            if choice == 1{
                let result = sum_even_numbers(start_num, end_num);
                println!("The sum of even numbers from {} to {} is: {}", start_num, end_num, result);
            } else if choice == 2{
                let result = sum_odd_numbers(start_num, end_num);
                println!("The sum of odd numbers from {} to {} is: {}", start_num, end_num, result);
            } else if choice ==3{
                let result = sum_all_numbers(start_num, end_num);
                println!("The sum of all numbers from {} to {} is: {}", start_num, end_num, result);
            } else {
                println!("Invalid choice! Please select a valid option (1-3).");      
            }
        } else if operation_choice == 2 {
            if choice == 1{
                let result = subtract_even_numbers(start_num, end_num);
                println!("The subtraction of even numbers from {} to {} is: {}", start_num, end_num, result);
            } else if choice == 2{
                let result = subtract_odd_numbers(start_num, end_num);
                println!("The subtraction of odd numbers from {} to {} is: {}", start_num, end_num, result);
            } else if choice == 3{
                let result = subtract_all_numbers(start_num, end_num);
                println!("The subtraction of all numbers from {} to {} is: {}", start_num, end_num, result);
            } else {
                println!("Invalid choice! Please select a valid option (1-3).");      
            }
        } else if operation_choice == 3 {
            if choice == 1{
                let result = multiply_even_numbers(start_num, end_num);
                println!("The multiplication of even numbers from {} to {} is: {}", start_num, end_num, result);
            } else if choice == 2{
                let result = multiply_odd_numbers(start_num, end_num);
                println!("The multiplication of odd numbers from {} to {} is: {}", start_num, end_num, result);
            } else if choice == 3{
                let result = multiply_all_numbers(start_num, end_num);
                println!("The multiplication of all numbers from {} to {} is: {}", start_num, end_num, result);
            } else {
                println!("Invalid choice! Please select a valid option (1-3).");      
            }
        } else if operation_choice == 4 {
            if choice == 1{
                let result = divide_even_numbers(start_num, end_num);
                println!("The division of even numbers from {} to {} is: {:.4}", start_num, end_num, result);
            } else if choice == 2{
                let result = divide_odd_numbers(start_num, end_num);
                println!("The division of odd numbers from {} to {} is: {:.4}", start_num, end_num, result);
            } else if choice == 3{
                let result = divide_all_numbers(start_num, end_num);
                println!("The division of all numbers from {} to {} is: {:.4}", start_num, end_num, result);
            } else {
                println!("Invalid choice! Please select a valid option (1-3).");      
            }
        }
    } 
    else {
        println!("Invalid choice! Please select a valid operation option (1-4).");      
    }

}


pub fn main() {
    println!("\n=============Calculation=============");
    type_of_operation();
}
