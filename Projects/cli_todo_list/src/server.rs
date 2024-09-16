use std::collections::HashMap;
use anyhow::Ok;
use chrono::NaiveDate;
use anyhow::Result;

use crate::Command;
use crate::AppError;

mod task;
use task::Task;

pub struct Server {
    tasks: HashMap<u64, Task>,
    task_id_counter: u64,
}

impl Server {
    pub fn new() -> Self {
        Server {
            tasks: HashMap::new(),
            task_id_counter: 1,
        }
    }

    pub fn run(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Add { task, due } => {
                self.add_task(task, due);
            },
            Command::List { all: _, due: None } => {
                self.list_all_tasks()?;
            },
            Command::List { all: true, due: _ } => {
                self.list_all_tasks()?;
            },
            Command::List { all: _, due: Some(due) } => {
                self.list_tasks_specific_due(due)?;
            },
            Command::Remove { id } => {
                self.remove_task(id)?;
            },
            Command::Update { id, task, due } => {
                self.update_task(id, task, due)?;
            },
            Command::Complete { id } => {
                self.complete_task(id)?;
            },
        };

        Ok(())
    }

    pub fn add_task(&mut self, task: Vec<String>, due: Option<NaiveDate>) {
        let task_name = task.join(" ");
        let task = self.tasks
            .entry(self.task_id_counter)
            .or_insert(Task::new(self.task_id_counter, task_name, due));

        self.task_id_counter = self.task_id_counter.wrapping_add(1);

        println!(
            "Task added: \"{}\" {}(Task id: {})", 
            task.name(), 
            task.due()
                .map(|date| format!("(Due: {}) ", date.format("%Y-%m-%d")))
                .unwrap_or_default(),
            task.id()
        );
    }

    pub fn list_all_tasks(&self) -> Result<()> {
        let tasks = self.tasks.values();

        if tasks.len() == 0 {
            return Err(AppError::NoTasksFound.into());
        }

        println!("ID  Task  Due Date  Status");
        for task in tasks {
            println!("{}", task);
        }

        Ok(())
    }

    pub fn list_tasks_specific_due(&self, due: NaiveDate) -> Result<()> {
        let tasks: Vec<&Task> = self.tasks.values()
            .filter(|task| task.due() == Some(&due))
            .collect();

        if tasks.is_empty() {
            return Err(AppError::NoTasksDueOn { due: due.format("%Y-%m-%d").to_string() }.into());
        }

        println!("Tasks due on {}:", due.format("%Y-%m-%d"));
        println!("ID  Task  Due Date  Status");
        for task in tasks {
            println!("{}", task);
        }

        Ok(())
    }

    pub fn remove_task(&mut self, id: u64) -> Result<()> {
        println!("Removing task with id: {}", id);

        match self.tasks.remove(&id) {
            Some(task) => println!("Task with ID {} removed.", task.name()),
            None => return Err(AppError::TaskNotFound { id }.into()),
        }

        Ok(())
    }

    pub fn update_task(&mut self, id: u64, task_name: Option<Vec<String>>, due: Option<NaiveDate>) -> Result<()> {
        let task = match self.tasks.get_mut(&id) {
            Some(task) => task,
            None => return Err(AppError::TaskNotFound { id }.into()),
        };
        task.update(task_name.map(|t| t.join(" ")), due);

        println!(
            "Task with ID {} updated: {} {})", 
            id, 
            task.name(),
            task.due()
                .map(|date| format!("(Due: {})", date.format("%Y-%m-%d")))
                .unwrap_or_default(),
        );

        Ok(())
    }

    pub fn complete_task(&mut self, id: u64) -> Result<()> {
        let task = match self.tasks.get_mut(&id) {
            Some(task) => task,
            None => return Err(AppError::TaskNotFound { id }.into()),
        };
        task.set_completed();

        println!("Task with ID {} marked as completed.", id);

        Ok(())
    }
}