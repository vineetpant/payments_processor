# payments_processor

## About
Given a CSV representing a series of transactions, implement a simple toy transactions engine that processes the payments crediting and debiting accounts. After processing the complete set of payments output the client account balances

## How to run
You should be able to run your payments engine like

```sh
$ cargo run -- transactions.csv > accounts.csv
```
The input file is the first and only argument to the binary. Output should be written to std out

## How to test
You can run simple cargo tests 

```sh
cargo test -- --nocapture
```

## Input

The input will be a CSV file with the columns type, client, tx, and amount. You can assume the type is a string, the client column is a valid u16 client ID, the tx is a valid u32 transaction ID, and the amount is a decimal value with a precision of up to four places past the decimal.

For example.

```sh
type, deposit, deposit, deposit, withdrawal, withdrawal,
client, tx, amount 1, 1, 1.0 2, 2, 2.0 1, 3, 2.0 1, 4, 1.5 2, 5, 3.0
```

The client ID will be unique per client though are not guaranteed to be ordered. Transactions to the client account 2 could occur before transactions to the client account 1. Likewise, transaction IDs (tx) are globally unique, though are also not guaranteed to be ordered. You can assume the transactions occur chronologically in the file, so if transaction b appears after a in the input file then you can assume b occurred chronologically after a. Whitespaces and decimal precisions (up to four places past the decimal) must be accepted by your program.

## Output
The output should be a list of client IDs (client), available amounts (available), held amounts (held), total amounts (total), and whether the account is locked (locked).

For example

```sh
client, available, held, total, locked
1, 1.5, 0.0, 1.5, false
2, 2.0, 0.0, 2.0, false
```