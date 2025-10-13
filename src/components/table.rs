use crate::state::{TableState, TableType};

use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    widgets,
    widgets::{Block, BorderType, Borders, Cell, Padding, Row},
};

pub struct Table {}

impl Table {
    pub fn new(state: &TableState) -> widgets::Table {
        widgets::Table::new(
            Self::rows(&state),
            Self::constraints(&state.table_type)
        )
        .header(Self::header(&state))
        .block(
            Block::default()
                .title(format!(" {} ", state.title()))
                .title_style(Style::default().bold())
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(Color::Green))
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .row_highlight_style(Style::default().reversed())
    }

    fn header(state: &TableState) -> Row {
        Row::new(state.header.iter().map(|column_name| {
            Cell::from(column_name.clone()).style(Style::default().fg(Color::Yellow))
        }))
        .style(Style::new().bold())
        .bottom_margin(1)
    }

    fn rows(state: &TableState) -> Vec<Row> {
        state
            .rows
            .iter()
            .map(|row| {
                Row::new(row.iter().map(|cell_title| {
                    Cell::from(cell_title.clone())
                }))
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
            TableType::Tasks => vec![
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ],
        }
    }
}
