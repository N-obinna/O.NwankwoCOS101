fn main() {
    // Define the given variables
    let principal: f64 = 520_000_000.0;
    let rate: f64 = 10.0;
    let years: i32 = 5;

    // Step 1: Calculate the total amount (A)
    // Formula: A = P * (1 + R / 100)^n
    let amount = principal * (1.0 + rate / 100.0).powi(years);

    // Step 2: Calculate the compound interest (CI)
    // Formula: CI = A - P
    let compound_interest = amount - principal;

    // Display the results formatted to 2 decimal places
    println!("Total Amount (A): ₦{:.2}", amount);
    println!("Compound Interest (CI): ₦{:.2}", compound_interest);
}