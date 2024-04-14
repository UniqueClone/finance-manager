use num_format::{Locale, ToFormattedString};
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
        let house_price: f64 = match remove_symbols_from_string(house_price).parse() {
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
            result.push_str(&format!(
                "\nDeposit:\t\t €{}\n",
                convert_number_to_currency(deposit)
            ));
            result.push_str(&format!(
                "Solicitor Fees:\t\t €{}\n",
                convert_number_to_currency(SOLFEES)
            ));
            result.push_str(&format!(
                "Evaluation Fees:\t €{}\n",
                convert_number_to_currency(EVALFEES)
            ));
            result.push_str(&format!(
                "Surveyor Fees:\t\t €{}\n",
                convert_number_to_currency(SURVEYFEES)
            ));
            result.push_str(&format!(
                "Stamp Duty:\t\t €{}\n",
                convert_number_to_currency(stamp_duty)
            ));
        }

        let savings_required = unsafe {
            savings_required
                .to_int_unchecked::<i32>()
                .to_formatted_string(&Locale::en)
        };

        result.push_str(&format!("\nRequired Savings:\t €{}", savings_required));

        println!("{}", result);
        println!("\nNote: All numbers are rounded to the nearest whole number.")
    }
}

/**
 * Calculate the total fees required for a house given the deposit and stamp duty.
 */
fn calculate_total_fees(deposit: f64, stamp_duty: f64) -> f64 {
    let savings_required = deposit + SOLFEES + EVALFEES + SURVEYFEES + stamp_duty;

    return savings_required;
}

#[test]
fn test_calculate_total_fees() {
    assert_eq!(calculate_total_fees(100000.0, 1000.0), 104150.0);
    assert_eq!(calculate_total_fees(200000.0, 2000.0), 205150.0);
}

/**
 * Convert a number to a currency string.
 */
fn convert_number_to_currency(number: f64) -> String {
    return unsafe {
        number
            .to_int_unchecked::<i32>()
            .to_formatted_string(&Locale::en)
    };
}

#[test]
fn test_convert_number_to_currency() {
    assert_eq!(convert_number_to_currency(2000.0), "2,000");
    assert_eq!(convert_number_to_currency(1000000.0), "1,000,000");
}

/**
 * Remove symbols from a string.
 * Removes the following symbols: ',', '€', '$', ' '.
 */
fn remove_symbols_from_string(string: String) -> String {
    return string
        .trim()
        .replace(",", "")
        .replace("€", "")
        .replace("$", "")
        .replace(" ", "");
}

#[test]
fn test_remove_symbols_from_string() {
    assert_eq!(
        remove_symbols_from_string("€1,000.00 ".to_string()),
        "1000.00"
    );

    assert_eq!(
        remove_symbols_from_string("€ 1,000.00".to_string()),
        "1000.00"
    );
}
