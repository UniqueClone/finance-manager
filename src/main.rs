const DEPOSITPER: f64 = 0.1;
const SOLFEES: f64 = 2500.0;
const EVALFEES: f64 = 150.0;
const SURVEYFEES: f64 = 500.0;
const STAMPDUTYPER: f64 = 0.01;

fn main() {
    let mut house_price_input = String::new();

    println!("Enter the price of the house: €");
    std::io::stdin()
        .read_line(&mut house_price_input)
        .expect("Failed to read line");

    let house_price: f64 = house_price_input
        .trim()
        .parse()
        .expect("Please enter a valid number");
    let deposit: f64 = house_price * DEPOSITPER;
    // let solicitor_fees: f64 = number * SOLFEES;
    // let inspector_fees: f64 = number * INSPECTFEES;
    let stamp_duty: f64 = house_price * STAMPDUTYPER;
    let savings_required = deposit + SOLFEES + EVALFEES + SURVEYFEES + stamp_duty;
    let result = format!("\nDeposit:\t\t €{:.2}\nSolicitor Fees:\t\t €{:.2}\nEvaluation Fees:\t €{:.2}\nSurveyor Fees:\t\t €{:.2}\nStamp Duty:\t\t €{:.2}\n\nRequired Savings:\t €{:.2}", deposit, SOLFEES, EVALFEES, SURVEYFEES, stamp_duty, savings_required);
    println!("{}\n", result);
}
