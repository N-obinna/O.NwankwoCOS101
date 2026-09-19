// Rust program to calculate employee annual incentive based on experience and age

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Is the employee experienced? (true/false): ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let is_experienced: bool = input1.trim().to_lowercase().parse().expect("Please enter true or false");

    println!("Enter age of employee: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age: i32 = input2.trim().parse().expect("Not a valid number");

    let incentive: i32;

    if is_experienced {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            // For ages 28 and 29
            incentive = 1_300_000;
        }
    } else {
        incentive = 100_000;
    }

    println!("The annual incentive for the employee is: N{}", incentive);
}