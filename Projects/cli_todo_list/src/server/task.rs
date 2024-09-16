use std::fmt::Display;

use chrono::NaiveDate;

#[derive(Debug)]
pub struct Task {
    id: u64,
    name: String,
    due: Option<NaiveDate>,
    completed: bool,
}

impl Task {
    pub fn new(id: u64, name: String, due: Option<NaiveDate>) -> Self {
        Task {
            id,
            name,
            due,
            completed: false,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn due(&self) -> Option<&NaiveDate> {
        self.due.as_ref()
    }

    pub fn set_completed(&mut self) {
        self.completed = true;
    }

    pub fn update(&mut self, name: Option<String>, due: Option<NaiveDate>) {
        if let Some(name) = name {
            self.name = name;
        }

        self.due = due;
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let due = self.due.map(|d| d.to_string()).unwrap_or_default();
        let completed = if self.completed { "Completed" } else { "Not completed" };

        write!(f, "{}  {}  {}  {}", self.id, self.name, due, completed)
    }
}