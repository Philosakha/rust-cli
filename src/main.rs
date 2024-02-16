mod check_balance;
mod check_mina;
mod create_frontend;
use clap::Parser;
use prettycli::*;

use terminal_menu::{button, label, menu, mut_menu, run};

// declare the parser

#[derive(Parser)]
struct commands {
    subcmd: String,
}

async fn check_balance(public_key: String) {
    check_balance::check_balance(public_key).await;
}

async fn check_mina() {
    check_mina::check_mina().await;
}

async fn create_frontend() {
    create_frontend::create_frontend().await;
}

#[tokio::main]
async fn main() {
    info("Say hello to Mina");

    let menu = menu(vec![
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
        button("Check Mina"),
        button("Create Frontend"),
    ]);

    run(&menu);

    println!("Selected: {}", mut_menu(&menu).selected_item_name());

    if mut_menu(&menu).selected_item_name() == "Check Balance" {
        info("Please enter your public key");
        let mut public_key = String::new();
        std::io::stdin()
            .read_line(&mut public_key)
            .expect("Failed to read line");

        wait("Checking balance...");

        check_balance(public_key).await;
    } else if mut_menu(&menu).selected_item_name() == "Check Mina" {
        wait("Checking Mina...");
        check_mina().await;
    } else if mut_menu(&menu).selected_item_name() == "Create Frontend" {
        wait("Creating frontend...");
        create_frontend().await;
    } else {
        println!("Please enter a valid command");
    }
}
