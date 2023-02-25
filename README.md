# Compound Interest Calculator

This program calculates compound interest for a given principal, interest rate, and number of days, and saves the results to a CSV file.

## Usage

The program reads the input values from the command line. To run the program, enter the following command:

```bash
cargo run
```

You will be prompted to enter the **principal**, **interest rate**, and number of **days**, separated by commas:

```bash
Enter principal, rate, and days (separated by commas):
```

Enter the values in the format principal,rate,days, for example:

```bash
Enter principal, rate, and days (separated by commas):
1000,5,365
```

The program will then calculate the compound interest and save the results to a CSV file named `principal-rate-days.csv`, for example:

```bash
Output saved to 1000-5-365.csv
```

## Implementation

The program consists of three functions:

`read_input()`: Reads the input values from the command line and returns them as a tuple.
`compound_interest(principal: f64, interest_rate: f64, days: f64) -> Vec<(usize, f64, f64, f64)>`: Calculates the compound interest for the given input values and returns a vector of tuples containing the day number, amount, increase, and total percentage increase for each day.
`write_output_to_csv(principal: f64, interest_rate: f64, days: f64, output_data: &[(usize, f64, f64, f64)]):` Writes the output data to a CSV file.
The program uses the `std::fs::File and std::io::{self, Write}` modules to write the output data to a file.

## Efficiency

To improve efficiency, the program calculates the compound interest and saves the results to a vector of tuples, and then writes the data to a file in batches. This approach reduces the number of file operations, which can improve performance when dealing with large amounts of data.
