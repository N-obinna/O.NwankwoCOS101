fn main() {
    // Define the given variables
    let principal: f64 = 210_000.0; // Initial cost of the TV
    let rate: f64 = 5.0;            // Annual depreciation rate (5%)
    let years: i32 = 3;             // Time period in years

    // Calculate the depreciated value (A)
    // Formula: A = P * (1 - R / 100)^n
    let depreciated_value = principal * (1.0 - rate / 100.0).powi(years);

    // Display the result formatted to 2 decimal places
    println!("Value of the TV after {} years: ₦{:.2}", years, depreciated_value);
}