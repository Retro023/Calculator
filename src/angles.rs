use std::io;
use std::process::Command;

// Clear the screen
fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("clear").status();
    }
}

// Selecting what kind of angle math
pub fn run() {
    println!("Angles calculator!");
    println!("please select a option");
    loop {
        println!("1. Quadrilateral angles");
        println!("2. Unit conversion");
        println!("3. Area of a sector");
        println!("4. Gradians");
        
        // taking users input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        clear_screen();
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        match choice {
            1 => quadrilateral(),
            2 => convert_degrees_to_radians(),
            3 => sector(),
            _ => {
                println!("Invalid choice, please select a valid option.");
                continue;
            }
        }
    }
}

fn quadrilateral() {
    // The angles
    let angle1 = read_angle("Please enter the first angle:");
    let angle2 = read_angle("Please enter the second angle:");
    let angle3 = read_angle("Please enter the third angle:");

    // Calculate the 4th angle
    let sum = angle1 + angle2 + angle3;
    if sum >= 360 {
        println!("The sum of the angles is greater than or equal to 360. Please enter valid angles.");
        return;
    }
    let angle4 = 360 - sum;
    println!("The missing angle is {}", angle4);
}

fn read_angle(prompt: &str) -> u32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<u32>() {
            Ok(angle) => return angle,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}
// convert units
fn convert_degrees_to_radians() {
    loop {
        println!("1. Convert degrees to radians");
        println!("2. Convert radians to degrees");
        println!("3. convert degrees to gradians");
        println!("4. Convert radians to gradians");
        println!("5. convert gradians to degrees");
        println!("6. convert gradians to radians");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        // match choice for converting units
        match choice {
            1 => {
                // degrees to radians
                clear_screen();
                let degrees = read_value("Enter degrees:");
                let radians = degrees as f64 * std::f64::consts::PI / 180.0;
                println!("{} degrees is {} radians", degrees, radians);
            }
            2 => {
                // radians to degrees
                clear_screen();
                let radians = read_value("Enter radians:");
                let degrees = radians as f64 * 180.0 / std::f64::consts::PI;
                println!("{} radians is {} degrees", radians, degrees);
            }
            // degrees to gradians
            3 => {
                let degrees = read_value("Enter degrees:");
                let gradians = degrees as f64 * std::f64::consts::PI / 180.0;
                println!("{} degrees is {} radians",degrees, gradians);
            }
            // radians to gradians
            4 => {
                let radians = read_value("Enter radians:");
                let gradians = radians * 200.0 / std::f64::consts::PI;
                println!("{} radians is {} gradians", radians, gradians);
            }
            // gradians to degrees
            5 => {
                let gradians = read_value("Enter gradians");
                let degrees = gradians * 9.0 / 10.0;
                println!(" {} gradians is {} degrees", gradians, degrees);
            }
            6 => {
                //gradians to radians
                let gradians = read_value("Enter gradians");
                let radians = gradians *  std::f64::consts::PI / 200.0;
                println!(" {} gradians is {} radians", gradians ,radians);
            }
            _ => {
                println!("Invalid choice, please select a valid option.");
                continue;
            }
        }
    }
}

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

fn sector() {
    clear_screen();
    let radius = read_angle("what is the radius of the circle: ");
    let angle = read_angle("What is the center angle of the Sector: ");
    let area = (angle as f64 / 360.0) * std::f64::consts::PI * (radius as f64).powi(2);
    println!("the area of the sector is {}", area);
}