mod check_balance;
mod check_mina;
use clap::Parser;

use youchoose;
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

    let mut menu = youchoose::Menu::new(0..2);

    menu.name("Choose a command")
        .item("Check balance", check_balance::check_balance)
        .item("Check mina", check_mina::check_mina);

    let subcmd = commands::parse().subcmd;

    if subcmd == "check_balance" {
        check_balance::check_balance().await;
    } else if subcmd == "check_mina" {
        check_mina::check_mina().await;
    } else {
        println!("Please enter a valid command");
    }
}
