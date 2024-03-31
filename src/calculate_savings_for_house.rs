use std::io;

const DEPOSITPER: f64 = 0.1;
const SOLFEES: f64 = 2500.0;
const EVALFEES: f64 = 150.0;
const SURVEYFEES: f64 = 500.0;
const STAMPDUTYPER: f64 = 0.01;

/**
 * Calculate the savings required for a house.
 */
pub fn calculate_savings_for_house(mut verbose: bool) {
    loop {
        println!("\nEnter the price of the house (or 'q' to quit, 'd' to toggle detailed output):");

        let mut house_price = String::new();

        // Read the input from the user
        io::stdin()
            .read_line(&mut house_price)
            .expect("Failed to read line");

        if house_price.trim() == "q" {
            println!("\nGoodbye! Best of luck with your house purchase!\n");
            break;
        } else if house_price.trim() == "d" {
            verbose = !verbose;
            println!(
                "\nDetailed output is now {}.\n",
                if verbose { "enabled" } else { "disabled" }
            );
            continue;
        } else if house_price.trim().is_empty() {
            println!("Please enter a valid number.");
            continue;
        }

        // Convert the input to a number
        let house_price: f64 = match house_price.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Calculate the required savings
        let deposit: f64 = house_price * DEPOSITPER;
        let stamp_duty: f64 = house_price * STAMPDUTYPER;
        let savings_required = calculate_total_fees(deposit, stamp_duty);

        // Print the result
        let mut result = String::new();

        if verbose {
            result.push_str(&format!("\nDeposit:\t\t €{:.2}\n", deposit));
            result.push_str(&format!("Solicitor Fees:\t\t €{:.2}\n", SOLFEES));
            result.push_str(&format!("Evaluation Fees:\t €{:.2}\n", EVALFEES));
            result.push_str(&format!("Surveyor Fees:\t\t €{:.2}\n", SURVEYFEES));
            result.push_str(&format!("Stamp Duty:\t\t €{:.2}\n", stamp_duty));
        }

        result.push_str(&format!("\nRequired Savings:\t €{:.2}", savings_required));

        println!("{}", result);
    }
}

/**
 * Calculate the total fees required for a house given the deposit and stamp duty.
 */
fn calculate_total_fees(deposit: f64, stamp_duty: f64) -> f64 {
    let savings_required = deposit + SOLFEES + EVALFEES + SURVEYFEES + stamp_duty;

    return savings_required;
}
