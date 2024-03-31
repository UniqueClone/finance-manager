use std::io;

const DEPOSITPER: f64 = 0.1;
const SOLFEES: f64 = 2500.0;
const EVALFEES: f64 = 150.0;
const SURVEYFEES: f64 = 500.0;
const STAMPDUTYPER: f64 = 0.01;

pub fn calculate_savings_for_house() {
    println!("\nEnter the price of the house: €");

    let mut house_price_input = String::new();

    io::stdin()
        .read_line(&mut house_price_input)
        .expect("Failed to read line");

    let house_price: f64 = house_price_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let deposit: f64 = house_price * DEPOSITPER;
    let stamp_duty: f64 = house_price * STAMPDUTYPER;
    let savings_required = calculate_required_savings(house_price);

    let result = format!("\nDeposit:\t\t €{:.2}\nSolicitor Fees:\t\t €{:.2}\nEvaluation Fees:\t €{:.2}\nSurveyor Fees:\t\t €{:.2}\nStamp Duty:\t\t €{:.2}\n\nRequired Savings:\t €{:.2}", deposit, SOLFEES, EVALFEES, SURVEYFEES, stamp_duty, savings_required);
    println!("{}\n", result);
}

fn calculate_required_savings(house_price: f64) -> f64 {
    let deposit: f64 = house_price * DEPOSITPER;
    let stamp_duty: f64 = house_price * STAMPDUTYPER;
    let savings_required = deposit + SOLFEES + EVALFEES + SURVEYFEES + stamp_duty;

    return savings_required;
}
