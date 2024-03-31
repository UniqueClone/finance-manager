use std::io;

const DEPOSITPER: f64 = 0.1;
const SOLFEES: f64 = 2500.0;
const EVALFEES: f64 = 150.0;
const SURVEYFEES: f64 = 500.0;
const STAMPDUTYPER: f64 = 0.01;

/**
 * Calculate the savings required for a house.
 */
pub fn calculate_savings_for_house() {
    println!("\nEnter the price of the house: €");

    let mut house_price = String::new();

    // Read the input from the user
    io::stdin()
        .read_line(&mut house_price)
        .expect("Failed to read line");

    // Convert the input to a number
    let house_price: f64 = house_price
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Calculate the required savings
    let deposit: f64 = house_price * DEPOSITPER;
    let stamp_duty: f64 = house_price * STAMPDUTYPER;
    let savings_required = calculate_total_fees(deposit, stamp_duty);

    // Print the result
    let result = format!("\nDeposit:\t\t €{:.2}\nSolicitor Fees:\t\t €{:.2}\nEvaluation Fees:\t €{:.2}\nSurveyor Fees:\t\t €{:.2}\nStamp Duty:\t\t €{:.2}\n\nRequired Savings:\t €{:.2}", deposit, SOLFEES, EVALFEES, SURVEYFEES, stamp_duty, savings_required);
    println!("{}\n", result);
}

/**
 * Calculate the total fees required for a house given the deposit and stamp duty.
 */
fn calculate_total_fees(deposit: f64, stamp_duty: f64) -> f64 {
    let savings_required = deposit + SOLFEES + EVALFEES + SURVEYFEES + stamp_duty;

    return savings_required;
}
