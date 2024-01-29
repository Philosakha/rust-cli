mod check_balance;
mod check_mina;
use clap::Parser;

use terminal_menu::{ menu, label, button, run, mut_menu };

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
    // put an if statement here to check if the user has entered an argument
    // if they have, then run the check_balance function
    // if they haven't, then run the check_mina function

    // let subcmd = commands::parse().subcmd;

    // if subcmd == "check_balance" {
    //     check_balance::check_balance().await;
    // } else if subcmd == "check_mina" {
    //     check_mina::check_mina().await;
    // } else {
    //     println!("Please enter a valid command");
    // }

    let menu = menu(
        vec![
            // label:
            //  not selectable, useful as a title, separator, etc...
            label("----------------------"),
            label("terminal-menu"),
            label("use wasd or arrow keys"),
            label("enter to select"),
            label("'q' or esc to exit"),
            label("-----------------------"),

            // button:
            //  exit the menu
            button("Check Balance"),
            button("Check Mina")
        ]
    );

    run(&menu);

    println!("Selected: {}", mut_menu(&menu).selected_item_name());

    if mut_menu(&menu).selected_item_name() == "Check Balance" {
        check_balance().await;
    } else if mut_menu(&menu).selected_item_name() == "Check Mina" {
        check_mina().await;
    } else {
        println!("Please enter a valid command");
    }
}
