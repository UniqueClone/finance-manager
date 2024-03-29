// slint::include_modules!();

const DEPOSITPER: f64 = 0.1;
const SOLFEES: f64 = 2500.0;
const INSPECTFEES: f64 = 1500.0;
const STAMPDUTYPER: f64 = 0.01;

fn main() {
    let mut house_price_input = String::new();
    println!("Enter the price of the house: ");
    std::io::stdin().read_line(&mut house_price_input).unwrap();

    let house_price: f64 = house_price_input.trim().parse().unwrap();
    let deposit: f64 = house_price * DEPOSITPER;
    // let solicitor_fees: f64 = number * SOLFEES;
    // let inspector_fees: f64 = number * INSPECTFEES;
    let stamp_duty: f64 = house_price * STAMPDUTYPER;
    let savings_required = deposit + SOLFEES + INSPECTFEES + stamp_duty;
    let result = format!("Deposit:\t\t €{:.2}\nSolicitor Fees:\t\t €{:.2}\nInspection Fees:\t €{:.2}\nStamp Duty:\t\t €{:.2}\n\nRequired Savings:\t €{:.2}", deposit, SOLFEES, INSPECTFEES, stamp_duty, savings_required);
    println!("{}", result);
}
