use std::env;
use todo::{Command, CommandFlags};

fn show_menu() {
    println!("RobCo Assistant");
    println!("1. Create a task");
    println!("2. View tasks");
    println!("3. Edit a task");
    println!("4. Delete a task");
    println!("5. Exit");
    println!("Please select an option:");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Your command line arguments are: {:?}", args);
        let command = CommandFlags::parse(&args);
        println!("Command: {:?}", command);

        if command.execute(&args) {
            println!("Command executed successfully");
        } else {
            println!("Command execution failed");
        }
        return;
    }

    show_menu();
}
