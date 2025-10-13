use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::Constraint,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Table, Row, Cell},
};

use crate::models::Project;
use crate::task_repository::{TaskRepository, TaskFileRepository};

#[derive(Debug)]
pub struct App {
    running: bool,
    projects: Vec<Project>,
    repository: Box<dyn TaskRepository>,
}

impl App {
    pub fn new() -> Self {
        let repo = TaskFileRepository::new("/home/karl/Documents/obsidian_vault/slate");

        Self {
            running: false,
            projects: Vec::new(),
            repository: Box::new(repo),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.projects = self.repository.fetch_projects()?;
        self.running = true;

        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let title = Line::from("Slate")
            .bold()
            .yellow();

        let header = Row::new(vec![
            Cell::from("Project").bold(),
            Cell::from("# Tasks").bold(),
            Cell::from("# Subprojects").bold(),
        ]);

        let rows = self.projects.iter().map(|project| {
            let tasks_count = project.tasks.len();
            let subprojects_count = project.subprojects.len();

            Row::new(vec![
                Cell::from(project.name.clone()),
                Cell::from(tasks_count.to_string()),
                Cell::from(subprojects_count.to_string()),
            ])
        });

        let table = Table::new(rows, vec![
            Constraint::Percentage(60),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
            .header(header)
            .block(Block::bordered().title(title))
            .row_highlight_style(Style::default().reversed());

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
        self.running = false;
    }
}
