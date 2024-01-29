use prettytable::{ row, Table };

use clap::Parser;
use reqwest;

#[derive(Debug, serde::Deserialize)]
struct Main {
    blockchainLength: u128,
    chainId: String,
    circulatingSupply: String,
    dateTime: String,
    epoch: u128,
    globalSlot: u128,
    lockedSupply: String,
    minWindowDensity: u128,
    nextEpochLedgerHash: String,
    previousStateHash: String,
    slot: u128,
    snarkedLedgerHash: String,
    stagedLedgerHash: String,
    stakingEpochLedgerHash: String,
    stateHash: String,
    totalCurrency: String,
}

#[derive(Parser)]
struct Arguement {
    attribute: String,
}

pub async fn check_mina() {
    let url = "https://api.minaexplorer.com/summary";
    let client = reqwest::Client::new();

    let response = client.get(url).send().await.unwrap();

    let body = response.text().await.unwrap();

    let json: Main = serde_json::from_str(&body).unwrap();

    let mut table = Table::new();

    table.add_row(row!["Blockchain Length", json.blockchainLength]);
    table.add_row(row!["Circular Supply", json.circulatingSupply]);
    table.add_row(row!["Epoch", json.epoch]);
    table.add_row(row!["State Hash", json.stateHash]);
    table.add_row(row!["Total Currency", json.totalCurrency]);

    table.printstd();
}
