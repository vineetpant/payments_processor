use crate::types::{ClientAccount, Transaction, TransactionType};
use std::{collections::HashMap, error::Error};

pub fn process_transactions(
    client_transaction_map: HashMap<u16, Vec<u32>>,
    transaction_map: HashMap<u32, Transaction>,
    transaction_dispute_map: HashMap<u32, Transaction>,
    transaction_resolve_map: HashMap<u32, Transaction>,
    transaction_chargeback_map: HashMap<u32, Transaction>,
) -> Result<Vec<ClientAccount>, Box<dyn Error>> {
    let mut client_accounts: Vec<ClientAccount> = vec![];

    for (client, txns) in client_transaction_map {
        let mut lock = false;
        let mut available: f32 = 0.0;
        let mut held: f32 = 0.0;
        let mut total: f32 = 0.0;

        for tx in txns {
            let transaction = transaction_map
                .get(&tx)
                .ok_or("Error in processing transactions")?;

            let amount = transaction
                .amount
                .ok_or("No amount found for deposit/withdrawal transaction")?;

            match transaction.r#type {
                TransactionType::Deposit => {
                    available = available + amount;
                    total = total + amount;
                }
                TransactionType::Withdrawal => {
                    if amount <= available {
                        // Process withdrawal only if amount is less than or equal to available balance
                        available = available - amount;
                        total = total - amount;
                    }
                }
                _ => {}
            }

            // Check for dispute
            let tx_dispute = transaction_dispute_map.get(&tx);
            match tx_dispute {
                Some(_) => {
                    // Update client balance as per disputed transaction
                    held = held + amount;
                    available = available - amount;

                    // Check if disputed transaction is resolved
                    let tx_resolved = transaction_resolve_map.get(&tx);
                    match tx_resolved {
                        Some(_) => {
                            // If transaction is resolved, update client balance
                            held = held - amount;
                            available = available + held;
                        }
                        None => {}
                    }

                    // Check if disputed transaction is resolved
                    let tx_chargeback = transaction_chargeback_map.get(&tx);
                    match tx_chargeback {
                        Some(_) => {
                            // If transaction is chargeback, update client balance and lock account
                            held = held - amount;
                            total = total - amount;
                            lock = true;
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }

        let account = ClientAccount {
            client,
            available,
            held,
            total,
            locked: lock,
        };
        client_accounts.push(account);
    }

    Ok(client_accounts)
}
