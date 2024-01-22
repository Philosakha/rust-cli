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
    total: String,
    unknown: String,
    blockHeight: u128,
    // lockedBalnce can be null
    lockedBalance: Option<String>,
}
#[derive(Debug, serde::Deserialize)]
struct Status {
    syncStatus: String,
    blockchainLength: u128,
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

    let client = reqwest::Client::new();

    let response = client.get(&url).header("Accept", "*/*").send().await;

    let body = response.unwrap().text().await.unwrap();

    let json: MinaAccountsData = serde_json::from_str(&body).unwrap();

    print!("The balance of {} is: ", json.account.publicKey);
    print!("{}", json.account.balance.total);
}
