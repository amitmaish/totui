mod app;
mod task;

use crate::{app::App, task::Task};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut app = App::new(vec![
        Task::new(false, "first task!!"),
        Task::new(true, "A completed task!!"),
        Task::new(true, "buy a motorcycle"),
    ]);
    ratatui::run(|terminal| app.draw(terminal))?;

    Ok(())
}
