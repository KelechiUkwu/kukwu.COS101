use std::io;

fn main() {
    println!("-----------------------------");
    println!("      Welcome to Rust Diner!");
    println!("-----------------------------");
    println!("Menu:");
    println!("P = Poundo Yam / Edinkaiko Soup  - ₦3,200");
    println!("F = Fried Rice & Chicken         - ₦3,000");
    println!("A = Amala & Ewedu Soup           - ₦2,500");
    println!("E = Eba & Egusi Soup             - ₦2,000");
    println!("W = White Rice & Stew            - ₦2,500");
    println!("---------------------------------------------");

    // Input food type
    println!("Enter the food type (P/F/A/E/W): ");
    let mut food_type = String::new();
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");
    let food_type = food_type.trim().to_uppercase();

    // Input quantity
    println!("Enter the quantity: ");
    let mut quantity_input = String::new();
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");
    let quantity: u32 = quantity_input.trim().parse().expect("Please enter a number");

    // Determine price
    let price_per_unit = match food_type.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food type entered!");
            return;
        }
    };

    let total = price_per_unit * quantity;

    // Apply discount if applicable
    let final_amount = if total > 10_000 {
        let discount = 0.05 * total as f64;
        total as f64 - discount
    } else {
        total as f64
    };

    println!("---------------------------------------------");
    println!("Total cost: ₦{}", total);
    if total > 10_000 {
        println!("You received a 5% discount!");
    }
    println!("Final amount to pay: ₦{:.2}", final_amount);
    println!("---------------------------------------------");
    println!("Thank you for your order!");
}

