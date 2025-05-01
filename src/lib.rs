pub mod cli;
pub mod models;
pub mod utils;

use std::vec;

use chrono::prelude::*;
pub use models::task_builder::TaskBuilder;
pub use utils::time::estimate_due_datetime;

// * Macros
// macro_rules! combine_flags {
//     ($($option:expr),*) => {
//         {
//             let mut combined = Vec::new();
//             $(
//                 combined.extend($option.flag());
//             )*
//             combined
//         }
//     };
// }

// commands
//     -- new <title> [options]
// options
//     -- description <description>
//     -- priority <priority>
//     -- time <time estimate>
//     -- due <due date>
//     -- status <status>

#[derive(Debug)]
pub enum CommandFlags {
    NewTask,
}

pub trait Command {
    fn flag(&self) -> String;
    fn parse(args: &[String]) -> Self;
    fn execute(&self, args: &[String]) -> bool;
}

impl CommandFlags {
    pub fn min_length(&self) -> usize {
        match self {
            CommandFlags::NewTask => 3 + CommandOptions::required(CommandFlags::NewTask).len(),
        }
    }

    pub fn create(self: &CommandFlags, args: &[String]) -> Task {
        let mut task = TaskBuilder::new();
        let required_options = CommandOptions::required(CommandFlags::NewTask);
        let optional_options = CommandOptions::optional(CommandFlags::NewTask);

        // Add the title to the task
        task = task.title(args[2].to_string());

        // Checking the required options
        for option in required_options {
            let mut position: usize = 0;
            let option_flags = option.flag();

            // Check if the option is present in the arguments for multiple flags
            for flag in option_flags {
                let index: Option<usize> = args.iter().position(|x: &String| x == flag);
                match index {
                    Some(i) => {
                        position = i;
                        break;
                    }
                    None => continue,
                }
            }

            // Stop if the required option is not found
            if position == 0 {
                panic!("Missing required option: {:?}", option);
            } else {
                let value_position = position + 1;

                // If the value position is out of bounds, panic
                if value_position > args.len() {
                    panic!("Missing value for option: {:?}", option);
                } else {
                    // else update the task with the value
                    let value: &String = &args[value_position];
                    // Update the task with the value
                    task = option.update_task(task, value);
                }
            }
        }

        for option in optional_options {
            let option_flags = option.flag();

            // Check if the option is present in the arguments for multiple flags
            for flag in option_flags {
                let index: Option<usize> = args.iter().position(|x: &String| x == flag);
                match index {
                    Some(i) => {
                        let value_position = i + 1;
                        // Check if the value position is out of bounds
                        if value_position > args.len() {
                            panic!("Missing value for option: {:?}", option);
                        } else {
                            // else update the task with the value
                            let value: &String = &args[value_position];
                            // Update the task with the value
                            task = option.update_task(task, value);
                        }
                        break;
                    }
                    None => continue,
                }
            }
        }

        println!("{:?}", task);
        task.build().expect("Failed to build task")
    }
}

impl Command for CommandFlags {
    fn flag(&self) -> String {
        match self {
            CommandFlags::NewTask => "--new".to_string(),
        }
    }

    fn parse(args: &[String]) -> Self {
        if args.len() < 2 {
            panic!("Not enough arguments");
        }

        match args[1].as_str() {
            "--new" => CommandFlags::NewTask,
            _ => panic!("Invalid command"),
        }
    }

    fn execute(&self, args: &[String]) -> bool {
        match self {
            CommandFlags::NewTask => {
                println!("{}", self.min_length());

                // Check if the command meeds the minimum length requirement
                if args.len() < self.min_length() {
                    panic!("Not enough arguments");
                }

                // Process the command
                let task = self.create(args);
                println!("{:?}", task);

                true
            }
        }
    }
}

#[derive(Debug)]
pub enum CommandOptions {
    Description,
    Priority,
    TimeEstimate,
}

impl CommandOptions {
    fn flag(&self) -> Vec<&str> {
        match self {
            CommandOptions::Description => vec!["-d", "--description"],
            CommandOptions::Priority => vec!["-p", "--priority"],
            CommandOptions::TimeEstimate => vec!["-t", "--time"],
        }
    }

    fn required(command: CommandFlags) -> Vec<CommandOptions> {
        match command {
            CommandFlags::NewTask => vec![CommandOptions::TimeEstimate],
        }
    }

    fn optional(command: CommandFlags) -> Vec<CommandOptions> {
        match command {
            CommandFlags::NewTask => vec![CommandOptions::Description, CommandOptions::Priority],
        }
    }

    fn update_task(&self, task: TaskBuilder, value: &String) -> TaskBuilder {
        match self {
            CommandOptions::Description => {
                // Check if the value is empty
                if value.is_empty() {
                    panic!("Description cannot be empty");
                }
                task.description(value.to_string())
            }
            CommandOptions::Priority => {
                // Check if the value is empty
                if value.is_empty() {
                    panic!("Priority cannot be empty");
                }
                let priority_value: u8 = value.parse().expect("Invalid priority value");
                task.priority(Priority::from(priority_value))
            }
            CommandOptions::TimeEstimate => {
                // Check if the value is empty
                if value.is_empty() {
                    panic!("Time estimate cannot be empty");
                }
                task.due(value.to_string())
            }
        }
    }
}

#[derive(Debug)]
pub enum Priority {
    DoesNotExist,
    Low,
    Medium,
    High,
}

impl Priority {
    fn from(value: u8) -> Self {
        match value {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            _ => Priority::DoesNotExist,
        }
    }

    #[allow(dead_code)]
    fn value(&self) -> u8 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::DoesNotExist => 0,
        }
    }
}

#[derive(Debug)]
pub enum TaskStatus {
    OnGoing,
    Completed,
    Cancelled,
    Delayed,
}

impl TaskStatus {
    pub fn from(value: u8) -> Self {
        match value {
            1 => TaskStatus::OnGoing,
            2 => TaskStatus::Completed,
            3 => TaskStatus::Cancelled,
            4 => TaskStatus::Delayed,
            _ => panic!("Invalid task status value"),
        }
    }
}

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: TaskStatus,
    pub due: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    #[allow(unused_variables)]
    pub fn create(args: Vec<String>) -> Task {
        todo!("Implement task creation logic");
    }

    pub fn new(
        id: u32,
        title: String,
        description: Option<String>,
        priority: Priority,
        status: TaskStatus,
        due: String,
    ) -> Task {
        let now = Utc::now();
        let due_date = estimate_due_datetime(&due);
        Task {
            id,
            title,
            description,
            priority,
            status,
            due: due_date,
            created_at: now,
            updated_at: now,
        }
    }
}
