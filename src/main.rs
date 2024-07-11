// libarys
use std::io;
use std::process::Command;
use crossterm::{event::{self, Event, KeyCode}, terminal};

// my codes
mod basiccalc;
mod geometry;
mod angles;
mod trigonometry;


fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("clear").status();
    }
}

fn wait_for_spacebar(){
    terminal::enable_raw_mode().expect("failed to enable raw mode");
    loop {
        if let Event::Key(key_event) = event::read().expect("Failed to read event"){
            if key_event.code == KeyCode::Char(' '){
                break;
            }else {
                println!("Press the spacebar to return to the main menu")
            }
        }
    }
terminal::disable_raw_mode().expect("failed to disable raw mode");
}

fn main() {
    println!("The rust calculator!");
    loop {
        println!("A rust based calculator!");
        println!("Select an option");
        println!("1.  Basic calculations");
        println!("2.  Geometric calculations");
        println!("3.  Geometric angles");
        println!("4.  Trigonometry");
        println!("5.  Exit");
        println!("0.  help");
        // User input for choice
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number (e.g., 1)");
                continue;
            }
        };

        match choice {
            1 => {
                clear_screen();
                basiccalc::run();      
            }
            2 => {
                clear_screen(); 
                geometry::run();       
            }
            3 => {
                clear_screen();
                angles::run();
            }
            
            4 => {
                clear_screen();
                trigonometry::run();
            }
            
            
            5 => {
                println!("Thank you for using the rust calculator.");
                break;
            }
            0 => {
                clear_screen();
                println!("HELP:");
                println!("This Calculator is written in Rust and it is here to help you in your maths work with features like");
                println!("- basic arithmetic");
                println!("- Geometry");
                println!("To use the Calculator just simply select the option you wish to use by typing the number and hitting enter");
                println!("To exit the program type 4");
                println!("\npress the spacebar to return to the main menu");
                wait_for_spacebar();
                clear_screen();
                continue;
            }
            _ => println!("Invalid choice. Please select a valid option (1-3)"),
        }
    }
}