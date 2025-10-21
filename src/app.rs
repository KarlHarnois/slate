use crate::actions::{self, StartApp};
use crate::selectors::KeyBindingsSelector;
use crate::states::{AppState, TextInput};
use crate::task_repo::{TaskFileRepository, TaskRepository};
use crate::widgets::{Table, TextField};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    widgets,
    widgets::Paragraph,
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
        let chunks = self.chunks(frame.area());
        self.render_table(frame, chunks[0], &self.state.projects_table);
        self.render_table(frame, chunks[1], &self.state.tasks_table);

        if let Some(modal) = self.state.modal.as_ref() {
            let area = self.popup_area(frame.area());
            let text_field = TextField {
                title: modal.title(),
                text: TextInput {
                    value: "Cleanup old todo list, foo bar baz old todo list baz baz".to_string(),
                    _cursor: 0
                }
            };
            frame.render_widget(text_field, area);
        }

        let keybindings = self.state.select(KeyBindingsSelector).join(" | ");

        let footer = Paragraph::new(format!(" {} ", keybindings))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Left);

        frame.render_widget(footer, chunks[2]);
    }

    fn chunks(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(66),
                Constraint::Length(1),
            ])
            .split(area)
    }

    fn render_table(
        &self,
        frame: &mut Frame,
        area: Rect,
        state: &crate::states::TableState,
    ) {
        let table = Table { state }.into_widget();
        let mut widget_state = widgets::TableState::default();
        widget_state.select(state.selected_row);
        frame.render_stateful_widget(table, area, &mut widget_state);
    }

    fn popup_area(&self, area: Rect) -> Rect {
        let [mut top_tier, _] = Layout::vertical([
            Constraint::Percentage(33),
            Constraint::Percentage(66),
        ])
        .areas(area);

        top_tier.height = top_tier.height.saturating_sub(1);

        let [row] = Layout::vertical([Constraint::Length(3)])
            .flex(Flex::Center)
            .areas(top_tier);

        let [popup_area] = Layout::horizontal([Constraint::Length(60)])
            .flex(Flex::Center)
            .areas(row);

        popup_area
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
