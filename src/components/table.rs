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
        widgets::Table::new(Self::rows(&state), Self::constraints(&state.table_type))
            .header(Self::header(&state))
            .block(
                Block::default()
                    .title(format!(" {} ", state.title()))
                    .title_style(Style::default().bold())
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Self::border_style(&state))
                    .padding(Padding::new(1, 0, 0, 0)),
            )
            .row_highlight_style(Style::default().reversed())
    }

    fn header(state: &TableState) -> Row {
        Row::new(state.header.iter().map(|column_name| {
            let cell = Cell::from(column_name.clone());
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

    fn rows(state: &TableState) -> Vec<Row> {
        state
            .rows
            .iter()
            .map(|row| Row::new(row.iter().map(|cell_title| Cell::from(cell_title.clone()))))
            .collect()
    }

    fn constraints(table_type: &TableType) -> Vec<Constraint> {
        match table_type {
            TableType::Projects => vec![
                Constraint::Percentage(60),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ],
            TableType::Tasks => vec![Constraint::Percentage(10), Constraint::Percentage(90)],
        }
    }

    fn border_style(state: &TableState) -> Style {
        let style = Style::default();

        if state.is_focused {
            style.fg(Color::Green)
        } else {
            style
        }
    }
}
