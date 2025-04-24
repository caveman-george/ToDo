pub mod cli;
pub mod models;
pub mod utils;

use chrono::prelude::*;
pub use models::task_builder::TaskBuilder;
pub use utils::time::estimate_due_datetime;

pub enum CliCommand {
    NewTask,
}

pub enum CliOptions {
    DoesNotExist,
    Description,
    Priority,
    TimeEstimate,
}

impl CliCommand {
    pub fn from(args: Vec<String>) -> Option<Task> {
        // Read the first argument from the command line as it
        // is the command
        match args.get(1) {
            Some(command) => match command.as_str() {
                "--new" => {
                    let task = CliCommand::NewTask;
                    task.execute(&args)
                }
                _ => None,
            },
            None => None,
        }
    }

    #[allow(unused_variables)]
    pub fn execute(&self, args: &[String]) -> Option<Task> {
        todo!("Implement the command execution logic");
        // match self {
        //     CliCommand::NewTask => {
        //         let task = TaskBuilder::new();

        //         // Gets the title
        //         task.title(args[2].to_string());

        //         todo!("Parse the arguments and set the task properties");
        //     }
        // }
    }

    pub fn required_options(&self) -> Vec<CliOptions> {
        match self {
            CliCommand::NewTask => vec![CliOptions::TimeEstimate],
        }
    }

    pub fn optional_options(&self) -> Vec<CliOptions> {
        match self {
            CliCommand::NewTask => vec![CliOptions::Description, CliOptions::Priority],
        }
    }

    pub fn min_length(&self) -> usize {
        match self {
            CliCommand::NewTask => 1 + self.required_options().len(),
        }
    }
}

impl CliOptions {
    pub fn from(value: &str) -> Self {
        match value {
            "-d" => CliOptions::Description,
            "-p" => CliOptions::Priority,
            "-t" => CliOptions::TimeEstimate,
            _ => CliOptions::DoesNotExist,
        }
    }
}

#[derive(Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl From<u8> for Priority {
    fn from(value: u8) -> Self {
        match value {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            _ => panic!("Invalid priority value"),
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
