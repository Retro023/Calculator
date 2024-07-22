use std::io::{self, Write};
use std::process::Command;
use crossterm::{event::{self, Event, KeyCode}, terminal};
// Clear screen function
fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("clear").status();
    }
}

fn wait_for_spacebar() {
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    loop {
        if let Event::Key(key_event) = event::read().expect("Failed to read event") {
            if key_event.code == KeyCode::Char(' ') {
                break;
            } else {
                println!("Press the spacebar to return to the main menu");
            }
        }
    }
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}




// Function definitions for basic operations
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

fn exponentiate(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

fn sine(x: f64) -> f64 {
    x.to_radians().sin()
}

// Main function
pub fn run() {
    loop {
        clear_screen(); // Clear the screen before displaying the menu
        println!("|----------------------------------------------------|");
        println!("My Calculator - It's not cheating if I made it");
        println!("Please select an option:");
        println!("1: Add");
        println!("2: Subtract");
        println!("3: Multiply");
        println!("4: Divide");
        println!("5: Exponentiate");
        println!("6: Sine");
        println!("7: Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1..=5 => {
                // Prompt for the first number
                print!("Enter first number: ");
                io::stdout().flush().unwrap();
                let mut num1 = String::new();
                io::stdin().read_line(&mut num1).expect("Failed to read line");
                let num1: f64 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                // Prompt for the second number
                print!("Enter second number: ");
                io::stdout().flush().unwrap();
                let mut num2 = String::new();
                io::stdin().read_line(&mut num2).expect("Failed to read line");
                let num2: f64 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                // Perform the selected operation based on the user's choice
                let result = match choice {
                    1 => add(num1, num2),
                    2 => subtract(num1, num2),
                    3 => multiply(num1, num2),
                    4 => match divide(num1, num2) {
                        Ok(res) => res,
                        Err(e) => {
                            println!("Error: {}", e);
                            continue;
                        }
                    },
                    5 => exponentiate(num1, num2),
                    _ => {
                        println!("Invalid choice, please try again.");
                        continue;
                    }
                };

                // Print the result of the operation
                println!("Result: {}", result);
                println!("----------------------------------"); // Separator for readability

                // Ask if the user wants to exit after seeing the result
                println!("Press 'q' to quit or any other key to continue: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let choice_to_exit: char = match input.trim().chars().next() {
                    Some(c) => c,
                    None => continue,
                };

                if choice_to_exit == 'q' {
                    println!("Exiting Calculator");
                    clear_screen();
                    break;
                }
            }
            6 => {
                // Sine function only needs one input
                print!("Enter the number (in degrees): ");
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Failed to read line");
                let num: f64 = match num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                // Calculate the sine of the number
                let result = sine(num);

                // Print the result of the operation
                println!("Result: {}", result);
                println!("----------------------------------"); // Separator for readability

                // Ask if the user wants to exit after seeing the result
                println!("Press 'q' to quit or any other key to continue: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let choice_to_exit: char = match input.trim().chars().next() {
                    Some(c) => c,
                    None => continue,
                };

                if choice_to_exit == 'q' {
                    println!("Exiting Calculator");
                    clear_screen();
                    break;
                }
            }
            7 => {
                clear_screen();
                println!("Exiting Calculator");
                clear_screen();
                println!("Press space_bar to continue");
                wait_for_spacebar();
                break;
            }
            _ => {
                println!("Invalid choice. Please select a number between 1 and 7.");
                continue;
            }
        }
    }
}
