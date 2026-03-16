use ratatui::{DefaultTerminal, Frame, widgets::List};

use std::fmt::Display;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let app = App::new(vec![
        Task::new(false, "first task!!"),
        Task::new(true, "A completed task!!"),
        Task::new(true, "buy a motorcycle"),
    ]);
    ratatui::run(|terminal| app.draw(terminal))?;

    Ok(())
}

#[derive(Default, Debug)]
pub struct App {
    tasks: Vec<Task>,
}

#[derive(Default, Debug)]
pub struct Task {
    completed: bool,
    description: String,
}

impl App {
    #[must_use]
    pub fn new(tasks: Vec<Task>) -> App {
        App { tasks }
    }

    /// # Errors
    /// will error if can't read a key
    pub fn draw(&self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let list = List::new(self.tasks.iter().map(Task::to_string));

        frame.render_widget(list, frame.area());
    }
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
