pub fn compound_interest(
    principal: f64,
    interest_rate: f64,
    days: f64,
) -> Vec<(usize, f64, f64, f64)> {
    let mut output_data = Vec::with_capacity(days as usize);
    let mut amount = principal;
    for i in 1..=(days as usize) {
        let rate_per_period = interest_rate / 100.0 / 1.0;
        let final_value = amount * (1.0 + rate_per_period).powf(1.0);
        let increase = final_value - amount;

        amount += increase;
        let total_percentage_increase = (amount - principal) / principal * 100.0;
        output_data.push((i, amount, increase, total_percentage_increase));
    }
    output_data
}
