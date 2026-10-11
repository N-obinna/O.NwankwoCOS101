use std::io;

fn calculate_trapezium() {
    let mut input = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let height: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter base 1:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let base1: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter base 2:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let base2: f64 = input.trim().parse().expect("Invalid input");

    let area = height / 2.0 * (base1 + base2);
    println!("Area of Trapezium = {}\n", area);
}

fn calculate_rhombus() {
    let mut input = String::new();
    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let diagonal1: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let diagonal2: f64 = input.trim().parse().expect("Invalid input");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus = {}\n", area);
}

fn calculate_parallelogram() {
    let mut input = String::new();
    println!("Enter base:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let base: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let altitude: f64 = input.trim().parse().expect("Invalid input");

    let area = base * altitude;
    println!("Area of Parallelogram = {}\n", area);
}

fn calculate_cube() {
    let mut input = String::new();
    println!("Enter side length:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let side: f64 = input.trim().parse().expect("Invalid input");

    let surface_area = 6.0 * side * side;
    println!("Surface Area of Cube = {}\n", surface_area);
}

fn calculate_cylinder() {
    let mut input = String::new();
    println!("Enter radius:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let radius: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let height: f64 = input.trim().parse().expect("Invalid input");

    let pi = std::f64::consts::PI;
    let volume = pi * radius * radius * height;
    println!("Volume of Cylinder = {}\n", volume);
}

fn main() {
    loop {
        println!("=== THE SHAPE CALCULATOR ===");
        println!("1. Trapezium (Area)");
        println!("2. Rhombus (Area)");
        println!("3. Parallelogram (Area)");
        println!("4. Cube (Surface Area)");
        println!("5. Cylinder (Volume)");
        println!("6. Exit");
        println!("Enter your choice (1-6):");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!\n");
                continue;
            }
        };

        match choice {
            1 => calculate_trapezium(),
            2 => calculate_rhombus(),
            3 => calculate_parallelogram(),
            4 => calculate_cube(),
            5 => calculate_cylinder(),
            6 => {
                println!("Exiting Shape Calculator. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please select between 1 and 6.\n"),
        }
    }
}