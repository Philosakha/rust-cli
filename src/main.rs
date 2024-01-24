mod check_balance;
mod check_mina;
use clap::Parser;

// declare the parser

#[derive(Parser)]
struct commands {
    subcmd: String,
}

#[tokio::main]
async fn main() {
    // put an if statement here to check if the user has entered an argument
    // if they have, then run the check_balance function
    // if they haven't, then run the check_mina function

    let subcmd = commands::parse().subcmd;

    if subcmd == "check_balance" {
        check_balance::check_balance().await;
    } else if subcmd == "check_mina" {
        check_mina::check_mina().await;
    } else {
        println!("Please enter a valid command");
    }
}
