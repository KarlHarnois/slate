use crate::states::{TableState, TableType};

use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    widgets,
    widgets::{Block, BorderType, Borders, Cell, Padding, Row},
};

pub struct Table;

impl Table {
    pub fn build<'a>(state: &'a TableState) -> widgets::Table<'a> {
        widgets::Table::new(
            Self::rows(state),
            Self::constraints(&state.table_type),
        )
        .header(Self::header(state))
        .block(
            Block::default()
                .title(format!(" {} ", state.title()))
                .title_style(Self::title_style(state))
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Self::border_style(state))
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .row_highlight_style(Style::default().reversed())
        .row_highlight_style(if state.is_focused {
            Style::default().bg(Color::DarkGray).fg(Color::White)
        } else {
            Style::default()
        })
    }

    fn header<'a>(state: &'a TableState) -> Row<'a> {
        Row::new(state.header.iter().map(|column_name| {
            let cell = Cell::from(column_name.as_str());
            let style = Style::default();

            if state.is_focused {
                cell.style(style.fg(Color::Yellow))
            } else {
                cell.style(style)
            }
        }))
        .style(Style::new().bold())
        .bottom_margin(1)
    }

    fn rows<'a>(state: &'a TableState) -> Vec<Row<'a>> {
        state
            .rows
            .iter()
            .map(|row| {
                Row::new(
                    row.iter().map(|cell_title| Cell::from(cell_title.clone())),
                )
            })
            .collect()
    }

    fn constraints(table_type: &TableType) -> Vec<Constraint> {
        match table_type {
            TableType::Projects => vec![
                Constraint::Percentage(60),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ],
            TableType::Tasks => {
                vec![Constraint::Percentage(10), Constraint::Percentage(90)]
            }
        }
    }

    fn title_style(state: &TableState) -> Style {
        if state.is_focused {
            Style::default().bold()
        } else {
            Style::default()
        }
    }

    fn border_style(state: &TableState) -> Style {
        if state.is_focused {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        }
    }
}
