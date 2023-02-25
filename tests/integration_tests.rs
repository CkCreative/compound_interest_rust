#[test]
fn test_compound_interest() {
    let principal = 100.0;
    let interest_rate = 1.0;
    let days = 70.0;
    let output_data = trader::compound_interest(principal, interest_rate, days);
    assert_eq!(output_data.len(), 70);
    assert_eq!(output_data[0].0, 1);
    assert_eq!(output_data[0].1, 101.00);
    assert_eq!(output_data[0].2, 1.00);
    assert_eq!(output_data[0].3, 1.00);
    assert_eq!(output_data[69].0, 70);
    assert_eq!(output_data[69].1, 200.68);
    assert_eq!(output_data[69].2, 1.99);
    assert_eq!(output_data[69].3, 100.68);
}
