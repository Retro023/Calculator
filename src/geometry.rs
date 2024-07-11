// geometry calc


use std::io;
use std::process::Command;

fn clear_screen(){
    if cfg!(target_os = "windows"){
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    }else {
        let _ = Command::new("clear").status();
    }
}
pub fn run() {
    
    println!("Geometry Calcuator!");

    loop {
        println!("Please select a shape to calculate:");
        println!("1. Circle");
        println!("2. Square");
        println!("3. Rectangle");
        println!("4. Triangle");
        println!("5. Pentagon");
        println!("6. Hexagon");
        println!("7. Octagon");
        println!("8. Rhombus");
        println!("9. Parallelogram");
        println!("10. Ellipse");
        println!("11. Quit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            clear_screen();
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => handle_circle(),
            2 => handle_square(),
            3 => handle_rectangle(),
            4 => handle_triangle(),
            5 => handle_pentagon(),
            6 => handle_hexagon(),
            7 => handle_octagon(),
            8 => handle_rhombus(),
            9 => handle_parallelogram(),
            10 => handle_ellipse(),
            11 => {
                println!("Thank you for using the Geometric Shapes Calculator. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select a number between 1 and 11."),
        }
    }
}

fn get_user_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
    }
}

fn handle_circle() {
    let radius = get_user_input("Please enter the radius of the circle:");
    let area = std::f64::consts::PI * radius.powi(2);
    let circumference = 2.0 * std::f64::consts::PI * radius;

    println!("The dimensions of the circle:");
    println!("|------------------------------------------------------------|");
    println!(" Radius: {:.2}", radius);
    println!(" The area of the circle is: {:.2} square units", area);
    println!(" The circumference of the circle is: {:.2} units", circumference);
    println!(
        "
                                                             *****
                                                         **         **
                                                       *               *
                                                      *                 *
                                                      *                 *
                                                      *                 *
                                                       *               *
                                                         **         **
                                                             *****
        "
    );
    println!("|------------------------------------------------------------|");
}

fn handle_square() {
    let side_length = get_user_input("Please enter the side length of the square:");
    let area = side_length.powi(2);
    let perimeter = 4.0 * side_length;

    println!("The dimensions of the square:");
    println!("|------------------------------------------------------------|");
    println!(" Side length: {:.2}", side_length);
    println!(" The area of the square is: {:.2} square units", area);
    println!(" The perimeter of the square is: {:.2} units", perimeter);
    println!("
                                                            +--------+
                                                            |        |
                                                            |        |
                                                            |        |
                                                            |        |
                                                            +--------+
    ");
    println!("|------------------------------------------------------------|");
}

fn handle_rectangle() {
    let width = get_user_input("Please enter the width of the rectangle:");
    let length = get_user_input("Please enter the length of the rectangle:");
    let area = width * length;
    let perimeter = 2.0 * (width + length);

    println!("The dimensions of the rectangle:");
    println!("|------------------------------------------------------------|");
    println!(" Width: {:.2}", width);
    println!(" Length: {:.2}", length);
    println!(" The area of the rectangle is: {:.2} square units", area);
    println!(" The perimeter of the rectangle is: {:.2} units", perimeter);
    println!(
        "
                                            +------------------+
                                            |                  |
                                            |                  |
                                            |                  |
                                            |                  |
                                            +------------------+
        "
    );
    println!("|------------------------------------------------------------|");
}

fn handle_triangle() {
    println!("Select the type of triangle:");
    println!("1. Equilateral Triangle");
    println!("2. Isosceles Triangle");
    println!("3. Scalene Triangle");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let choice: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    match choice {
        1 => handle_equilateral_triangle(),
        2 => handle_isosceles_triangle(),
        3 => handle_scalene_triangle(),
        _ => println!("Invalid choice. Please select a number between 1 and 3."),
    }
}

fn handle_pentagon() {
    let side_length = get_user_input("Please enter the side length of the pentagon:");
    let perimeter = 5.0 * side_length;
    let apothem = 0.5 * side_length / std::f64::consts::PI;
    let area = 0.5 * perimeter * apothem;

    println!("The dimensions of the pentagon:");
    println!("|------------------------------------------------------------|");
    println!(" Side length: {:.2}", side_length);
    println!(" The perimeter of the pentagon is: {:.2} units", perimeter);
    println!(" The area of the pentagon is: {:.2} square units", area);
    println!("
                                                                 /\\
                                                               /    \\
                                                             /        \\
                                                           /            \\
                                                         /                \\
                                                       /____________________\\
    ");
    println!("|------------------------------------------------------------|");
}

fn handle_hexagon() {
    let side_length = get_user_input("Please enter the side length of the hexagon:");
    let perimeter = 6.0 * side_length;
    let area = (3.0 * f64::sqrt(3.0) / 2.0) * side_length.powi(2);

    println!("The dimensions of the hexagon:");
    println!("|------------------------------------------------------------|");
    println!(" Side length: {:.2}", side_length);
    println!(" The perimeter of the hexagon is: {:.2} units", perimeter);
    println!(" The area of the hexagon is: {:.2} square units", area);
    println!(
        "
                                                           _______
                                                          /       \\
                                                         /         \\
                                                        /           \\
                                                        \\           /
                                                         \\         /
                                                          \\_______/
        "
    );
    println!("|------------------------------------------------------------|");
}

fn handle_octagon() {
    let side_length = get_user_input("Please enter the side length of the octagon:");
    let perimeter = 8.0 * side_length;
    let area = 2.0 * (1.0 + f64::sqrt(2.0)) * side_length.powi(2);

    println!("The dimensions of the octagon:");
    println!("|------------------------------------------------------------|");
    println!(" Side length: {:.2}", side_length);
    println!(" The perimeter of the octagon is: {:.2} units", perimeter);
    println!(" The area of the octagon is: {:.2} square units", area);
    println!(
        "
                                                           _______
                                                         /       \\
                                                        /         \\
                                                       /           \\
                                                       \\           /
                                                        \\         /
                                                         \\_______/
        "
    );
    println!("|------------------------------------------------------------|");
}

fn handle_rhombus() {
    let diagonal1 = get_user_input("Please enter the first diagonal length of the rhombus:");
    let diagonal2 = get_user_input("Please enter the second diagonal length of the rhombus:");
    let area = 0.5 * diagonal1 * diagonal2;

    println!("The dimensions of the rhombus:");
    println!("|------------------------------------------------------------|");
    println!(" First diagonal length: {:.2}", diagonal1);
    println!(" Second diagonal length: {:.2}", diagonal2);
    println!(" The area of the rhombus is: {:.2} square units", area);
    println!(
        "
                                              ______
                                            /\\      /\\
                                           /  \\    /  \\
                                          /    \\  /    \\
                                         /      \\/      \\
                                         \\      /\\      /
                                          \\    /  \\    /
                                           \\  /    \\  /
                                            \\/______\\/
    "
    );
    println!("|------------------------------------------------------------|");
}

fn handle_parallelogram() {
    let base = get_user_input("Please enter the base length of the parallelogram:");
    let height = get_user_input("Please enter the height of the parallelogram:");
    let area = base * height;
    let perimeter = 2.0 * (base + height);

    println!("The dimensions of the parallelogram:");
    println!("|------------------------------------------------------------|");
    println!(" Base length: {:.2}", base);
    println!(" Height: {:.2}", height);
    println!(" The perimeter of the parallelogram is: {:.2} units", perimeter);
    println!(" The area of the parallelogram is: {:.2} square units", area);
    println!("
                                                                  /--------------
                                                                /                 \\
                                                              /                    \\
                                                            /                       \\
                                                          /                          \\
                                                        /                             \\
                                                      /_____________________________\\
    ");
    println!("|------------------------------------------------------------|");
}

fn handle_equilateral_triangle() {
    let side_length = get_user_input("Please enter the side length of the equilateral triangle:");
    let area = (side_length.powi(2) * f64::sqrt(3.0)) / 4.0;
    let perimeter = 3.0 * side_length;

    println!("The dimensions of the equilateral triangle:");
    println!("|------------------------------------------------------------|");
    println!(" Side length: {:.2}", side_length);
    println!(" The area of the equilateral triangle is: {:.2} square units", area);
    println!(" The perimeter of the equilateral triangle is: {:.2} units", perimeter);
    println!("
                                                             /\\
                                                           /  \\
                                                         /    \\
                                                       /      \\
                                                     /        \\
                                                   /__________\\
    ");
    println!("|------------------------------------------------------------|");
}

fn handle_isosceles_triangle() {
    let base = get_user_input("Please enter the base length of the isosceles triangle:");
    let side = get_user_input("Please enter the side length of the isosceles triangle:");
    let height = f64::sqrt(side.powi(2) - (base / 2.0).powi(2));
    let area = 0.5 * base * height;
    let perimeter = 2.0 * side + base;

    println!("The dimensions of the isosceles triangle:");
    println!("|------------------------------------------------------------|");
    println!(" Base length: {:.2}", base);
    println!(" Side length: {:.2}", side);
    println!(" Height: {:.2}", height);
    println!(" The area of the isosceles triangle is: {:.2} square units", area);
    println!(" The perimeter of the isosceles triangle is: {:.2} units", perimeter);
    println!("
                                                             /\\
                                                           /  \\
                                                         /    \\
                                                       /      \\
                                                     /        \\
                                                   /__________\\
    ");
    println!("|------------------------------------------------------------|");
}

fn handle_scalene_triangle() {
    let side1 = get_user_input("Please enter the first side length of the scalene triangle:");
    let side2 = get_user_input("Please enter the second side length of the scalene triangle:");
    let side3 = get_user_input("Please enter the third side length of the scalene triangle:");
    let perimeter = side1 + side2 + side3;
    let s = perimeter / 2.0;
    let area = f64::sqrt(s * (s - side1) * (s - side2) * (s - side3));

    println!("The dimensions of the scalene triangle:");
    println!("|------------------------------------------------------------|");
    println!(" Side length 1: {:.2}", side1);
    println!(" Side length 2: {:.2}", side2);
    println!(" Side length 3: {:.2}", side3);
    println!(" The area of the scalene triangle is: {:.2} square units", area);
    println!(" The perimeter of the scalene triangle is: {:.2} units", perimeter);
    println!("
                                                             /\\
                                                           /  \\
                                                         /    \\
                                                       /      \\
                                                     /        \\
                                                   /__________\\
    ");
    println!("|------------------------------------------------------------|");
}

fn handle_ellipse() {
  let major_axis = get_user_input("Please enter the length of the major axis of the ellipse:");
  let minor_axis = get_user_input("Please enter the length of the minor axis of the ellipse:");
  let area = std::f64::consts::PI * (major_axis / 2.0) * (minor_axis / 2.0);
  let perimeter = std::f64::consts::PI * (3.0 * (major_axis + minor_axis) - ((3.0 * major_axis + minor_axis) * (major_axis + 3.0 * minor_axis)).sqrt());

  println!("The dimensions of the Ellipse:");
  println!("|-------------------------------------------------------------------------------|");
  println!("Major Axis: {:.2}", major_axis);
  println!("Minor Axis: {:.2}", minor_axis);
  println!("The area of the Ellipse is: {:.2} square units", area);
  println!("The approximate perimeter of the Ellipse is: {:.2} units", perimeter);
  println!(
      "
                                                         ________
                                                    /                 \\
                                                  /                     \\
                                                /                         \\
                                              /                             \\
                                             \\                              /
                                               \\                          /
                                                 \\                      /
                                                   \\                 /
                                                     \\________/
      "
  );
  println!("|--------------------------------------------------------------------------------|")
}