use std::io;

fn main() {
    println!("Enter the total cost of the meal:");
    ///This allows a value for the rest of the calulations to be based on


    let mut cost = String::new();
    io::stdin().read_line(&mut cost).expect("Failed to read line");

    let cost: f64 = cost.trim().parse().expect("Please enter a valid number");

    println!("Enter the desired tip percentage:");

    
    let mut tip_percentage = String::new();
    io::stdin().read_line(&mut tip_percentage).expect("Failed to read line");

    let tip_percentage: f64 = tip_percentage.trim().parse().expect("Please enter a valid number");

    let tip = cost * (tip_percentage / 100.0);
    let total = cost + tip;

    println!("Tip: ${:.2}", tip);
    println!("Total: ${:.2}", total);
}
