use crate::actions::{self, StartApp};
use crate::components::Table;
use crate::states::AppState;
use crate::task_repo::{TaskFileRepository, TaskRepository};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    widgets,
};
use std::rc::Rc;

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
        let chunks = self.tables_chunk(frame.area());
        self.render_table(frame, chunks[0], &self.state.projects_table);
        self.render_table(frame, chunks[1], &self.state.tasks_table);

        if let Some(modal) = self.state.modal.as_ref() {
            let block =
                widgets::Block::bordered().title(format!(" {} ", modal.title));
            let area = self.popup_area(frame.area(), 60, 20);
            frame.render_widget(widgets::Clear, area);
            frame.render_widget(block, area);
        }
    }

    fn tables_chunk(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(66),
            ])
            .split(area)
    }

    fn render_table(
        &self,
        frame: &mut Frame,
        area: Rect,
        state: &crate::states::TableState,
    ) {
        let table = Table::build_widget(state);
        let mut widget_state = widgets::TableState::default();
        widget_state.select(state.selected_row);
        frame.render_stateful_widget(table, area, &mut widget_state);
    }

    fn popup_area(&self, area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)])
            .flex(Flex::Center);
        let horizontal =
            Layout::horizontal([Constraint::Percentage(percent_x)])
                .flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
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
