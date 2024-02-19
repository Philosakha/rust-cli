use colored::Colorize;
use prettytable::{row, Table};

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

pub async fn take_snapshot() {
    let url = "https://api.minaexplorer.com/summary";
    let client = reqwest::Client::new();

    let response = client.get(url).send().await.unwrap();

    let body = response.text().await.unwrap();

    let json: Main = serde_json::from_str(&body).unwrap();

    let snapshot = json.stateHash;

    println!("Snapshot: {}", snapshot.green().bold());
}
