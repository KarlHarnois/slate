use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::actions::{self, StartApp};
use crate::components::Table;
use crate::states::AppState;
use crate::task_repo::{TaskFileRepository, TaskRepository};

#[derive(Debug)]
pub struct App {
    state: AppState,
    repository: Box<dyn TaskRepository>,
}

impl App {
    pub fn new() -> Self {
        let repo = TaskFileRepository::new(
            "/home/karl/Documents/obsidian_vault/slate",
        );

        Self {
            state: AppState::new(),
            repository: Box::new(repo),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let projects = self.repository.fetch_projects()?;
        self.state.apply(actions::UpdateProjects { projects });
        self.state.apply(StartApp);

        while self.state.is_running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let projects_table = Table::build(&self.state.projects_table);
        let tasks_table = Table::build(&self.state.tasks_table);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(66),
            ])
            .split(frame.area());

        frame.render_widget(projects_table, chunks[0]);
        frame.render_widget(tasks_table, chunks[1]);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                self.on_key_event(key)
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        let action = actions::HandleKeyEvent { key };
        self.state.apply(action);
    }
}
