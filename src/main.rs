use std::env;

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
        return;
    }

    show_menu();
}
