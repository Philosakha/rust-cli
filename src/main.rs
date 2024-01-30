mod check_balance;
mod check_mina;
use clap::Parser;
use prettycli::*;

use terminal_menu::{ button, label, menu, mut_menu, run };

// declare the parser

#[derive(Parser)]
struct commands {
    subcmd: String,
}

async fn check_balance() {
    check_balance::check_balance().await;
}

async fn check_mina() {
    check_mina::check_mina().await;
}

#[tokio::main]
async fn main() {
    info("Say hello to Mina");

    // let menu = menu(
    //     vec![
    //         // label:
    //         //  not selectable, useful as a title, separator, etc...
    //         label("----------------------"),
    //         label("terminal-menu"),
    //         label("use wasd or arrow keys"),
    //         label("enter to select"),
    //         label("'q' or esc to exit"),
    //         label("-----------------------"),
    //         // button:
    //         //  exit the menu
    //         button("Check Balance"),
    //         button("Check Mina")
    //     ]
    // );

    // run(&menu);

    // println!("Selected: {}", mut_menu(&menu).selected_item_name());

    // if mut_menu(&menu).selected_item_name() == "Check Balance" {
    //     check_balance().await;
    // } else if mut_menu(&menu).selected_item_name() == "Check Mina" {
    //     check_mina().await;
    // } else {
    //     println!("Please enter a valid command");
    // }
}
