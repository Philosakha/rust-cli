use clap::Parser;
use reqwest;

/// calling the Mina RPC endpoint to get the balance of an account
use prettytable::{ row, Table };

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
struct Arguement {
    address: String,
}

pub async fn check_balance(public_key: String) {
    let _dummy_address = "B62qp95jhpNec4Cex77dkbPTCQ6zNwuKBYNPrWjaK1Dt6nfCr5boobN";

    let url = format!("https://api.minaexplorer.com/accounts/{}", public_key);

    let client = reqwest::Client::new();

    let response = client.get(&url).header("Accept", "*/*").send().await;

    let body = response.unwrap().text().await.unwrap();

    let json: MinaAccountsData = serde_json::from_str(&body).unwrap();

    let mut table = Table::new();

    table.add_row(row!["Public Key", json.account.publicKey]);
    table.add_row(row!["Total Balance", json.account.balance.total]);
    table.add_row(row!["Unknown Balance", json.account.balance.unknown]);
    table.add_row(row!["Block Height", json.account.balance.blockHeight]);
    table.add_row(
        row!["Locked Balance", json.account.balance.lockedBalance.unwrap_or("None".to_string())]
    );
    table.add_row(row!["Sync Status", json.status.syncStatus]);

    table.printstd();
}
