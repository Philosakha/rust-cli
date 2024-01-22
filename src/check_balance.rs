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
    public_key: String,
    balance: Balance,
}

#[derive(Debug, serde::Deserialize)]
struct Balance {
    total: f64,
    locked_balance: f64,
    block_height: f64,
    unknown: f64,
}
#[derive(Debug, serde::Deserialize)]
struct Status {
    sync_status: String,
    blockchain_length: f64,
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

    /// split the data around \

    let data = response.unwrap().text().await.unwrap();

    let data = data.split("\\");
    let data = data.collect::<Vec<&str>>();

    let data = data[0];
    print!("{} ", data);
}
