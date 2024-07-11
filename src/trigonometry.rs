use std::io;
use std::process::Command;

use num_complex::Complex;

// Clear the screen
fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("clear").status();
    }
}

// Func to read a floating-point value from user input
fn read_value(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

// Func to convert degrees to radians
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.0)
}




// Func for complex Sine
fn complex_sin(angle: f64) -> Complex<f64> {
    let angle_radians = degrees_to_radians(angle);
    Complex::new(angle_radians.sin(), 0.0)
}

//Func for complex cosine
fn complex_cos(angle: f64) -> Complex<f64> {
    let angle_radians = degrees_to_radians(angle);
    Complex::new(angle_radians.cos(), 0.0)

}

//Func for complex tangent
fn complex_tan(angle: f64) -> Complex<f64> {
    let angle_radians = degrees_to_radians(angle);
    Complex::new(angle_radians.tan(), 0.0)
}








pub fn run() {
    loop {
        clear_screen();
        println!("Trigonometry Calculator!");
        println!("1. Sine");
        println!("2. Cosine");
        println!("3. Angle in Degrees");
        println!("4. Tangent");
        println!("5. Complex Sine");
        println!("6. Complex Cosine");
        println!("7. Complex Tangent");
        println!("8. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number (e.g., 1)");
                continue;
            }
        };

        match choice {
            1 => {
                let angle_degrees = read_value("Enter the angle in degrees:");
                let angle_radians = degrees_to_radians(angle_degrees);
                let sine = angle_radians.sin();
                println!("Sine: {}", sine);
                println!("Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            2 => {
                let angle_degrees = read_value("Enter the angle in degrees:");
                let angle_radians = degrees_to_radians(angle_degrees);
                let cosine = angle_radians.cos();
                println!("Cosine: {}", cosine);
                println!("Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            3 => {
                let angle_degrees = read_value("Enter the angle in degrees:");
                println!("Angle: {} degrees", angle_degrees);
                println!("Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            4 => {
                let angle_degrees = read_value("Enter the angle in degrees:");
                let angle_radians = degrees_to_radians(angle_degrees);
                let tangent = angle_radians.tan();
                println!("Tangent: {}", tangent);
                println!("Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            
            5 => {
                let angle_degrees = read_value("Enter the angle in degrees:");
                let result = complex_sin(angle_degrees);
                println!("|-------------------------------------------|");
                println!("Complex Sine: {}", result);
                println!("\nPress Enter to continue ...");
                println!("|-------------------------------------------|");
                let _ = io::stdin().read_line(&mut String::new()); 
            }
            
            6 => {
                let angle_degrees = read_value("Enter the angle in degrees:");
                let result = complex_cos(angle_degrees);
                println!("|-------------------------------------------|");
                println!("Complex Cosine: {}", result);
                println!("\nPress Enter to continue ...");
                println!("|-------------------------------------------|");
                let _ = io::stdin().read_line(&mut String::new()); 
            }
            
            7 => {
                let angle_degrees = read_value("Enter the angle in degrees:");
                let result = complex_tan(angle_degrees);
                println!("|-------------------------------------------|");
                println!("Complex Tangent: {}", result);
                println!("\nPress Enter to continue ...");
                println!("|-------------------------------------------|");
                let _ = io::stdin().read_line(&mut String::new()); 
            }

            
            8 => {
                println!("Exiting Trigonometry Calculator");
                break;
            }
            _ => {
                println!("Invalid choice. Please select an option from 1 to 5.");
                println!("Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
            }
        }
    }
}