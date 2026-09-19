// Rust program to find the roots of a quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value for a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value for b: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value for c: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    // Calculate discriminant
    let d: f32 = (b * b) - (4.0 * a * c);

    if d > 0.0 {
        let root1: f32 = (-b + d.sqrt()) / (2.0 * a);
        let root2: f32 = (-b - d.sqrt()) / (2.0 * a);
        println!("The equation has two distinct real roots:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if d == 0.0 {
        let root: f32 = -b / (2.0 * a);
        println!("The equation has exactly one real root:");
        println!("Root = {}", root);
    } else {
        println!("The discriminant is negative (d = {}). There are no real roots.", d);
    }
}