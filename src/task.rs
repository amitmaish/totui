use std::fmt::Display;

#[derive(Default, Debug)]
pub struct Task {
    completed: bool,
    description: String,
}

impl Task {
    #[must_use]
    pub fn new(completed: bool, description: impl Into<String>) -> Task {
        Task {
            completed,
            description: description.into(),
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "[{}] - {}",
            if self.completed { "x" } else { " " },
            self.description
        )
    }
}
