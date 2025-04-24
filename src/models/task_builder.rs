use crate::{Priority, Task, TaskStatus};

macro_rules! impl_setters {
    ($($field:ident: $type:ty),*) => {
        $(
            pub fn $field(mut self, $field: $type) -> Self {
                self.$field = Some($field);
                self
            }
        )*
    };
}

pub struct TaskBuilder {
    id: Option<u32>,
    title: Option<String>,
    description: Option<String>,
    priority: Option<Priority>,
    status: Option<TaskStatus>,
    due: Option<String>,
}

impl Default for TaskBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskBuilder {
    pub fn new() -> Self {
        TaskBuilder {
            id: None,
            title: None,
            description: None,
            priority: Some(Priority::Low),
            status: Some(TaskStatus::OnGoing),
            due: None,
        }
    }

    impl_setters!(
        id: u32,
        title: String,
        description: String,
        priority: Priority,
        status: TaskStatus,
        due: String
    );

    pub fn build(self) -> Result<Task, String> {
        if self.title.is_none() {
            return Err("Title is required".to_string());
        }

        if self.due.is_none() {
            return Err("Due date is required".to_string());
        }

        Ok(Task::new(
            self.id.unwrap_or(0),
            self.title.unwrap(),
            self.description,
            self.priority.unwrap(),
            self.status.unwrap(),
            self.due.unwrap(),
        ))
    }
}
