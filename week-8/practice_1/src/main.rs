fn main() {
    // Using Vec::new()[cite: 32]
    let v: Vec<i64> = Vec::new();

    // printing the size of vector[cite: 32]
    println!("\nThe length of Vec::new is: {}", v.len());

    // Using macro[cite: 32]
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    // printing the size of vector[cite: 32]
    println!("\nThe length of vec macro is: {}", v.len());
}