mod check_balance;
mod check_mina;
mod create_frontend;
mod take_snapshot;
use cfonts::{render, say, Fonts, Options};
use clap::Parser;
use colored::Colorize;
use prettycli::*;

use terminal_menu::{button, label, menu, mut_menu, run};

// declare the parser

#[derive(Parser)]
struct Commands {
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

async fn take_snapshot() {
    take_snapshot::take_snapshot().await;
}

#[tokio::main]
async fn main() {
    info("Say hello to Mina");

    // write in cool huge letters
    say(Options {
        text: String::from("Mina"),
        ..Options::default()
    });

    // ask the user to press enter
    wait("Press enter to continue...");

    // do not show the menu if the user doesn't press enter
    let _ = std::io::stdin().read_line(&mut String::new());

    let menu = menu(vec![
        // label:
        //  not selectable, useful as a title, separator, etc...
        label("----------------------"),
        label("use wasd or arrow keys"),
        label("enter to select"),
        label("'q' or esc to exit"),
        label("-----------------------"),
        // button:
        //  exit the menu
        button("Check Balance"),
        button("Check Mina"),
        button("Create Frontend"),
        button("Take Snapshot of Mina"),
        button("exit"),
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
    } else if mut_menu(&menu).selected_item_name() == "Take Snapshot of Mina" {
        wait("Taking snapshot...");
        take_snapshot().await;
    } else if mut_menu(&menu).selected_item_name() == "exit" {
        println!("{}", "Goodbye!".green());
    } else {
        println!("Please enter a valid command");
    }
}
