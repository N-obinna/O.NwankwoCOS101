use std::io;

fn main() {
    // 1. Display the Menu
    println!("=== RESTAURANT MENU ===");
    println!("P - Poundo Yam / Edinkaiko Soup : N3,200");
    println!("F - Fried Rice & Chicken        : N3,000");
    println!("A - Amala & Ewedu Soup          : N2,500");
    println!("E - Eba & Egusi Soup            : N2,000");
    println!("W - White Rice & Stew           : N2,500");
    println!("=======================");

    // 2. Read Food Type Choice
    println!("\nEnter your choice (P, F, A, E, W):");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    
    // Trim whitespace and convert to uppercase for easy matching
    let choice = choice.trim().to_uppercase();

    // 3. Determine Price per Item based on Choice
    let price: f64 = if choice == "P" {
        3200.0
    } else if choice == "F" {
        3000.0
    } else if choice == "A" {
        2500.0
    } else if choice == "E" {
        2000.0
    } else if choice == "W" {
        2500.0
    } else {
        println!("Invalid selection! Defaulting to N0.");
        0.0
    };

    if price == 0.0 {
        return; // Exit early if an invalid menu option was chosen
    }

    // 4. Read Quantity
    println!("Enter quantity:");
    let mut quantity_input = String::new();
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");

    let quantity: f64 = match quantity_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid quantity entered.");
            return;
        }
    };

    // 5. Calculate Total and Apply Discount Rule
    let mut total = price * quantity;
    println!("\nSubtotal: N{:.2}", total);

    if total > 10000.0 {
        let discount = total * 0.05; // 5% discount
        total -= discount;
        println!("Discount applied (5%): -N{:.2}", discount);
    } else {
        println!("No discount applied (Order must exceed N10,000).");
    }

    // 6. Print Final Amount
    println!("Final Total Charge: N{:.2}", total);
}

