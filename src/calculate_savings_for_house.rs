use num_format::{Locale, ToFormattedString};
use std::env;
use std::fs;
use std::io;

const DEPOSITPER: f64 = 0.1;
const SOLFEES: f64 = 3400.0;
const EVALFEES: f64 = 150.0;
const SURVEYFEES: f64 = 500.0;
const SEARCHFEES: f64 = 250.0;
const STAMPDUTYPER: f64 = 0.01;

fn load_fees() -> Result<(f64, f64, f64, f64, f64, f64), String> {
    let args: Vec<String> = env::args().collect();

    // if args.len() != 2 {
    //     return Err("Please provide a file path.".to_string());
    // }

    let file_path =
        "C:\\Users\\ryanlynch\\Documents\\repos\\personal\\finance-manager\\src\\feesbreakdown.txt"
            .to_string();

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(_) => return Err("Failed to read the file.".to_string()),
    };

    // TODO: be able to label the fees in the file
    let fees: Vec<f64> = contents
        .lines()
        .map(|line| line.parse::<f64>().unwrap())
        .collect();

    if fees.len() != 6 {
        return Err("Please provide 6 fees.".to_string());
    }

    return Ok((fees[0], fees[1], fees[2], fees[3], fees[4], fees[5]));
}

/**
 * Calculate the savings required for a house.
 */
pub fn calculate_savings_for_house(mut verbose: bool) {
    let fees = match load_fees() {
        Ok(fees) => fees,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let (deposit_per, sol_fees, eval_fees, survey_fees, search_fees, stamp_duty_per) = fees;

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
        let deposit: f64 = house_price * deposit_per;
        let stamp_duty: f64 = house_price * stamp_duty_per;
        let savings_required = calculate_total_fees(
            deposit,
            sol_fees,
            eval_fees,
            survey_fees,
            search_fees,
            stamp_duty,
        );

        // Print the result
        let mut result = String::new();

        if verbose {
            result.push_str(&format!(
                "\nHouse Price:\t\t €{}\n",
                convert_number_to_currency(house_price)
            ));
            result.push_str(&format!(
                "\nDeposit:\t\t €{}\n",
                convert_number_to_currency(deposit)
            ));
            result.push_str(&format!(
                "Solicitor Fees:\t\t €{}\n",
                convert_number_to_currency(sol_fees)
            ));
            result.push_str(&format!(
                "Search Fees (Est.):\t €{}\n",
                convert_number_to_currency(search_fees)
            ));
            result.push_str(&format!(
                "Evaluation Fees:\t €{}\n",
                convert_number_to_currency(eval_fees)
            ));
            result.push_str(&format!(
                "Surveyor Fees:\t\t €{}\n",
                convert_number_to_currency(survey_fees)
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
fn calculate_total_fees(
    deposit: f64,
    sol_fees: f64,
    eval_fees: f64,
    survey_fees: f64,
    search_fees: f64,
    stamp_duty: f64,
) -> f64 {
    // let savings_required = deposit + SOLFEES + EVALFEES + SURVEYFEES + SEARCHFEES + stamp_duty;
    let savings_required = deposit + sol_fees + eval_fees + survey_fees + search_fees + stamp_duty;

    return savings_required;
}

#[test]
fn test_calculate_total_fees() {
    // assert_eq!(calculate_total_fees(100000.0, 1000.0), 104150.0);
    // assert_eq!(calculate_total_fees(200000.0, 2000.0), 205150.0);
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
