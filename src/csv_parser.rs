use crate::types::{Transaction, TransactionType};
use csv::{ReaderBuilder, Trim};
use std::collections::HashMap;
use std::{io::Read, error::Error};

pub fn parse_csv_data(
    br: Box<dyn Read>
) -> Result<
    (
        HashMap<u16, Vec<u32>>,
        HashMap<u32, Transaction>,
        HashMap<u32, Transaction>,
        HashMap<u32, Transaction>,
        HashMap<u32, Transaction>,
    ),
    Box<dyn Error>,
> {
   
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::All)
        .from_reader(br);

    let records = rdr.deserialize();

    let mut client_transaction_map: HashMap<u16, Vec<u32>> = HashMap::new();

    let mut transaction_map: HashMap<u32, Transaction> = HashMap::new(); // Map to hold deposit and withdrawal transactions
    let mut transaction_dispute_map: HashMap<u32, Transaction> = HashMap::new(); // Map to hold disputed transactions
    let mut transaction_resolve_map: HashMap<u32, Transaction> = HashMap::new(); // Map to hold resolved transactions
    let mut transaction_chargeback_map: HashMap<u32, Transaction> = HashMap::new(); // Map to hold chargeback transactions

    for record in records {
        let trx: Transaction = record?;

        match trx.r#type {
            TransactionType::Deposit | TransactionType::Withdrawal => { // If the transaction is deposit or withdrawal, it should be mapped with client 
                let existing_val = client_transaction_map.get_mut(&trx.client);

                // If client,transaction pair exists add the transaction to existing list of tractions mapped with client id
                match existing_val {
                    Some(transactions) => {
                        transactions.push(trx.tx);
                    }
                    None => {
                        client_transaction_map.insert(trx.client, [trx.tx].to_vec());
                    }
                }
                // Insert deposit/withdrawal trancsations to map
                transaction_map.insert(trx.tx, trx);
            }
            TransactionType::Dispute => {
                transaction_dispute_map.insert(trx.tx, trx);
            }
            TransactionType::Resolve => {
                transaction_resolve_map.insert(trx.tx, trx);
            }
            TransactionType::Chargeback => {
                transaction_chargeback_map.insert(trx.tx, trx);
            }
        }
    }
    Ok((
        client_transaction_map,
        transaction_map,
        transaction_dispute_map,
        transaction_resolve_map,
        transaction_chargeback_map,
    ))
}
