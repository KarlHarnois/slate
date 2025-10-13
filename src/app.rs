use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

use crate::components::{Table, TableState};
use crate::models::AppState;
use crate::task_repository::{TaskFileRepository, TaskRepository};

#[derive(Debug)]
pub struct App {
    state: AppState,
    repository: Box<dyn TaskRepository>,
}

impl App {
    pub fn new() -> Self {
        let repo = TaskFileRepository::new("/home/karl/Documents/obsidian_vault/slate");

        Self {
            state: AppState::new(),
            repository: Box::new(repo),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.state.projects = self.repository.fetch_projects()?;
        self.state.running = true;

        while self.state.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let mut state = TableState::new();
        state.title = "Projects".to_string();

        state.header = vec![
            "Name".to_string(),
            "Tasks".to_string(),
            "Subprojects".to_string()
        ];

        state.rows = self.state.projects.iter().map(|project| {
            let tasks_count = project.tasks.len();
            let subprojects_count = project.subprojects.len();
            vec![
                project.name.clone(),
                tasks_count.to_string(),
                subprojects_count.to_string(),
            ]
        })
        .collect();

        let table = Table::new(state);
        frame.render_widget(table, frame.area());
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.state.running = false;
    }
}
