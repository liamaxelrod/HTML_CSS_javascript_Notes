// Interactive Rust Example: Using match, if/else, and user input
// Corresponds to HTML template: <!-- Rust: If ... else ... / Switch -> Match / User Input / Result Handling --> in N/Rust/#
// =============================================================
// Title: Interactive Number Selector
// What this example shows: How to use `match`, `if/else`, and handle user input.
// Learning purpose: To visually see how Rust reacts to different inputs and control flow.

use std::io; // Import the input/output library for reading user input

fn main() {
    loop {
        println!("\nMain Menu:");
        println!("1. Test number positivity");
        println!("2. Check even or odd");
        println!("3. Multiply by 10");
        println!("Type 'exit' to quit.");

        // Create a mutable String to store user input
        let mut input = String::new();

        // Read input from the user and store it in `input`
        io::stdin() // io::stdin() does the reading from input
            .read_line(&mut input) // .read_line() reads a line and appends it to `input`
            .expect("Failed to read input"); // .expect() handles any errors that may occur

        // `trim()` removes whitespace (like newlines)
        let input = input.trim();

        // Exit program if user types 'exit' (case insensitive)
        if input.eq_ignore_ascii_case("exit") { // `eq_ignore_ascii_case()` compares strings without worrying about uppercase/lowercase.
            println!("Goodbye!");
            break; // Exit the loop and end the program
        }

        // Use `match` to determine what to do based on the user's choice
        match input {
            "1" => {
                // Ask for a number and check if it's positive, negative, or zero
                println!("Enter a number to test if it's positive or negative:");
                let mut num_input = String::new();
                io::stdin().read_line(&mut num_input).expect("Failed to read input");
                let num_input = num_input.trim();

                match num_input.parse::<i32>() { // the integer num_input is `parse::<i32>()` what tries to convert the string to a 32-bit integer
                    // maych Method returns a Result type, which can be Ok or Err
                    // Ok(num) is successful and num Becomes num_input Until the end of this match
                    Ok(num) => {
                        // If successful, use if/else to check the value
                        if num > 0 {
                            println!("{} is positive.", num);
                        } else if num < 0 {
                            println!("{} is negative.", num);
                        } else {
                            println!("{} is zero.", num);
                        }
                    }
                    // Err(_) handles the case where parsing fails
                    Err(_) => println!("Invalid number. Please try again."),
                }
            }

            "2" => {
                // Check if a number is even or odd
                println!("Enter a number to check if it's even or odd:");
                let mut num_input = String::new();
                io::stdin().read_line(&mut num_input).expect("Failed to read input");
                let num_input = num_input.trim();

                match num_input.parse::<i32>() {
                    Ok(num) => {
                        if num % 2 == 0 {
                            println!("{} is even.", num);
                        } else {
                            println!("{} is odd.", num);
                        }
                    }
                    Err(_) => println!("Invalid number. Please try again."),
                }
            }

            "3" => {
                // Multiply a number by 10
                println!("Enter a number to multiply by 10:");
                let mut num_input = String::new();
                io::stdin().read_line(&mut num_input).expect("Failed to read input");
                let num_input = num_input.trim();

                match num_input.parse::<i32>() {
                    Ok(num) => println!("{} * 10 = {}", num, num * 10),
                    Err(_) => println!("Invalid number. Please try again."),
                }
            }

            _ => println!("Invalid choice. Please enter 1, 2, 3, or 'exit'."),
        }
    }
}


