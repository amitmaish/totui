use crossterm::event::Event;
use ratatui::{
    DefaultTerminal, Frame,
    style::Style,
    widgets::{Block, List, ListDirection, ListState},
};

use crate::task::Task;

#[derive(Default, Debug)]
pub struct App {
    tasks: Vec<Task>,
    selected: ListState,
}

impl App {
    #[must_use]
    pub fn new(tasks: Vec<Task>) -> App {
        let mut selected = ListState::default();
        selected.select_first();
        App { tasks, selected }
    }

    /// # Errors
    /// will error if can't read a key
    pub fn draw(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        use crossterm::event::KeyCode as K;
        loop {
            terminal.draw(|frame| self.render(frame))?;
            match crossterm::event::read()? {
                Event::Key(key) => match key.code {
                    K::Up | K::Char('k') => self.selected.select_previous(),
                    K::Down | K::Char('j') => self.selected.select_next(),
                    K::Backspace | K::Delete => {
                        if let Some(index) = self.selected.selected() {
                            _ = self.tasks.remove(index);
                        }
                    }
                    K::Esc | K::Char('q') => break Ok(()),
                    _ => (),
                },
                _ => todo!(),
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let list = List::new(self.tasks.iter().map(Task::to_string))
            .block(Block::bordered().title("tasks"))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(" > ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        frame.render_stateful_widget(list, frame.area(), &mut self.selected);
    }
}
