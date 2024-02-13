use clap::Parser;

use terminal_menu::{ button, label, menu, mut_menu, run };

pub async fn create_frontend() {
    info("Let's create a starting point for you");

    let nextJSTemplate = "https://github.com/Philosakha/Starter-kit-mina-Auro-NEXTJS-13";
    let reactTemplate = "https://github.com/Philosakha/starter-kit-mina-auro-react";

    let menu = menu(
        vec![
            label("----------------------"),
            label("terminal-menu"),
            label("use wasd or arrow keys"),
            label("enter to select"),
            label("'q' or esc to exit"),
            label("-----------------------"),
            button("NextJS"),
            button("React")
        ]
    );

    run(&menu);

    println!("Selected: {}", mut_menu(&menu).selected_item_name());

    // ask for the name of the project
    info("Please enter the name of your project");
    let mut project_name = String::new();
    std::io::stdin().read_line(&mut project_name).expect("Failed to read line");

    if mut_menu(&menu).selected_item_name() == "NextJS" {
        // clone the nextjs template with the project name

        let cloneCommand = format!("git clone {} {}", nextJSTemplate, project_name);

        std::process::Command
            ::new("sh")
            .arg("-c")
            .arg(cloneCommand)
            .output()
            .expect("Failed to execute command");

        info("NextJS project created successfully");
    }

    if mut_menu(&menu).selected_item_name() == "React" {
        // clone the react template with the project name

        let cloneCommand = format!("git clone {} {}", reactTemplate, project_name);

        std::process::Command
            ::new("sh")
            .arg("-c")
            .arg(cloneCommand)
            .output()
            .expect("Failed to execute command");

        info("React project created successfully");
    }
}
