fn main() {
    // Array containing the amounts from the sales record
    let amounts = [
        450_000.00,  // Toshiba
        1_500_000.00, // Mac
        750_000.00,   // HP
        2_850_000.00, // Dell
        250_000.00,   // Acer
    ];

    // Calculate the total sum
    let sum: f64 = amounts.iter().sum();

    // Calculate the average
    let count = amounts.len() as f64;
    let average = sum / count;

    // Display the results
    println!("Total Sales Sum: ₦{:.2}", sum);
    println!("Average Sales Amount: ₦{:.2}", average);
}