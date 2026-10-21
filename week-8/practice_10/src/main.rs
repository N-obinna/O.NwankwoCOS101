fn main() {
    let b: (i32, bool, f64) = (30, true, 4.9);
    print_tuple(b);
}

fn print_tuple(x: (i32, bool, f64)) {
    println!("Inside print method");
    // assigns a tuple to distinct variables[cite: 32]
    let (age, is_male, cgpa) = x;
    println!("Age is {}, isMale? {}, cgpa is {}", age, is_male, cgpa);
}