use std::io;

fn main() {
    println!("=== EY Nigeria Developer Experience Checker ===");

    // Vector of tuples to store candidates: (Name, Years of Experience)[cite: 34]
    let mut candidates: Vec<(&str, u32)> = Vec::new();

    let mut input_count = String::new();
    println!("How many candidates are in the interview?");
    io::stdin().read_line(&mut input_count).expect("Failed to read input");
    let count: u32 = input_count.trim().parse().unwrap_or(0);

    // Hardcoded list or dynamically populated vector of candidate tuples[cite: 34]
    let sample_candidates = vec![
        ("Adebayo", 4),
        ("Chinedu", 9),
        ("Fatima", 6),
        ("Blessing", 11),
        ("Kareem", 8),
    ];

    // Using sample vector for simplicity[cite: 34]
    for candidate in sample_candidates {
        candidates.push(candidate);
    }

    if candidates.is_empty() {
        println!("No candidates found.");
        return;
    }

    // Initialize tracking variables with the first candidate[cite: 34]
    let mut highest_name = candidates[0].0;
    let mut highest_years = candidates[0].1;

    // Loop through the vector of tuples to find the max years[cite: 34]
    for i in 0..candidates.len() {
        println!("Candidate: {}, Experience: {} years", candidates[i].0, candidates[i].1);
        if candidates[i].1 > highest_years {
            highest_years = candidates[i].1;
            highest_name = candidates[i].0;
        }
    }

    println!("\n-------------------------------------------");
    println!("Most Experienced Developer Selected for EY:");
    println!("Name: {}", highest_name);
    println!("Years of Experience: {}", highest_years);
    println!("-------------------------------------------");
}