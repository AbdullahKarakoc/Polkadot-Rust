use std::io;

// Define the Operation enum
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Calculate function using pattern matching
fn calculate(operation: Operation, a: f64, b: f64) -> f64 {
    match operation {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => a / b,
    }
}

fn main() {
    // Prompt user for input
    println!("Enter the first number:");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Invalid input for the first number!");

    println!("Enter the operation (+, -, *, /):");
    let mut operation_symbol = String::new();
    io::stdin().read_line(&mut operation_symbol).expect("Failed to read line");
    let operation = match operation_symbol.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            println!("Invalid operation symbol!");
            return;
        }
    };

    println!("Enter the second number:");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Invalid input for the second number!");

    // Call the calculate function with the created Operation enum instance
    let result = calculate(operation, first_number, second_number);

    // Print the result to the console
    println!("Result: {}", result);
}
