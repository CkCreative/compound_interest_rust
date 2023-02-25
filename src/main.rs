use std::fs::File;
use std::io::{self, Write};

extern crate trader;

fn main() {
    let (principal, interest_rate, days) = read_input();

    let output_data = trader::compound_interest(principal, interest_rate, days);

    write_output_to_csv(principal, interest_rate, days, &output_data);
}

fn read_input() -> (f64, f64, f64) {
    // Read input values from a single line separated by commas
    println!("Enter principal, rate, and days (separated by commas):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let values: Vec<f64> = input
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid input"))
        .collect();

    // Extract input values from the vector
    let principal = values[0];
    let rate = values[1];
    let days = values[2];

    // Return the values as a tuple
    (principal, rate, days)
}

fn write_output_to_csv(
    principal: f64,
    interest_rate: f64,
    days: f64,
    output_data: &[(usize, f64, f64, f64)],
) {
    let filename = format!("{}-{}-{:.0}.csv", principal, interest_rate, days);
    let mut file = File::create(&filename).expect("Failed to create file");
    writeln!(file, "day,amount,increase,total_percentage_increase")
        .expect("Failed to write to file");

    for (i, amount, increase, total_percentage_increase) in output_data.iter() {
        writeln!(
            file,
            "{},{:.2},{:.2},{:.2}%",
            i, amount, increase, total_percentage_increase
        )
        .expect("Failed to write to file");
    }

    println!("Output saved to {}", filename);
}
