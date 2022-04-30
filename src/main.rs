mod cmd;
mod csv_parser;
mod processor;
pub mod types;

use cmd::{get_app, get_argument_value};
use csv_parser::parse_csv_data;
use processor::process_transactions;
use types::ClientAccount;

use std::{error::Error, fs::File, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    // Process command line args
    let app = get_app()?;
    let matches = app.get_matches();

    let file_path = get_argument_value(&matches, "file", None)?;

    let br = BufReader::new(File::open(file_path)?);

    // Parse buffered stream
    let (
        client_transaction_map,
        transaction_map,
        transaction_dispute_map,
        transaction_resolve_map,
        transaction_chargeback_map,
    ) = parse_csv_data(Box::new(br))?;

    let accounts_balance = process_transactions(
        client_transaction_map,
        transaction_map,
        transaction_dispute_map,
        transaction_resolve_map,
        transaction_chargeback_map,
    )?;

    // Print account balances
    print_accounts_balance(accounts_balance);

    Ok(())
}

fn print_accounts_balance(accounts_balance: Vec<ClientAccount>) {
    println!("client,available,held,total,locked");

    for account in accounts_balance {
        println!(
            "\n{},{:.4},{:.4},{:.4},{}",
            account.client, account.available, account.held, account.total, account.locked
        );
    }
}

// Test trasaction processor
#[cfg(test)]
mod tests {
    use crate::{
        csv_parser::parse_csv_data,
        processor::process_transactions,
    };

    const SAMPLE_INPUT: &str = 
    "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0";

    #[tokio::test]
    async fn can_parse_input_and_process_transactions() -> Result<(), Box<dyn std::error::Error>> {
        let str_buf = stringreader::StringReader::new(SAMPLE_INPUT);

        // Parse buffered stream
        let (
            client_transaction_map,
            transaction_map,
            transaction_dispute_map,
            transaction_resolve_map,
            transaction_chargeback_map,
        ) = parse_csv_data(Box::new(str_buf))?;

        let accounts_balance = process_transactions(
            client_transaction_map,
            transaction_map,
            transaction_dispute_map,
            transaction_resolve_map,
            transaction_chargeback_map,
        )?;

        // Print account balances
        crate::print_accounts_balance(accounts_balance);
        Ok(())
    }
}
