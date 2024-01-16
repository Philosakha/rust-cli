use clap::Parser;
use reqwest;

/// calling the Mina RPC endpoint to get the balance of an account

use prettytable::{ Table, Row, Cell };

// declare the struct for the response
#[derive(Debug, serde::Deserialize)]
struct MinaAccountsData {
    account: Account,
    status: Status,
}

#[derive(Debug, serde::Deserialize)]
struct Account {
    publicKey: String,
    balance: Balance,
}

#[derive(Debug, serde::Deserialize)]
struct Balance {
    total: f64,
    lockedBalnce: f64,
    blockHeight: f64,
    unknown: f64,
}
#[derive(Debug, serde::Deserialize)]
struct Status {
    syncStatus: String,
    blockchainLength: f64,
}

// declare the parser

#[derive(Parser)]
struct arguement {
    address: String,
}

pub async fn check_balance() {
    let public_key = arguement::parse().address;

    let dummy_address = "B62qp95jhpNec4Cex77dkbPTCQ6zNwuKBYNPrWjaK1Dt6nfCr5boobN";

    let url = format!("https://api.minaexplorer.com/accounts/{}", public_key);

    print!("Checking balance for {}...", public_key);
}
