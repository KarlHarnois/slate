use crate::components::Block;
use crate::states::{RowEmphasis, RowState, TableState, TableType};

use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style, Stylize},
    widgets,
    widgets::{Cell, Row},
};

pub struct Table;

impl Table {
    pub fn build_widget<'a>(state: &'a TableState) -> widgets::Table<'a> {
        widgets::Table::new(
            Self::rows(state),
            Self::constraints(&state.table_type),
        )
        .header(Self::header(state))
        .block(
            Block {
                title: state.title().clone(),
                is_focused: state.is_focused,
            }
            .into_widget(),
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
        state.rows.iter().map(Self::row).collect()
    }

    fn row<'a>(state: &'a RowState) -> Row<'a> {
        Row::new(state.cells.iter().map(|cell| Cell::from(cell.clone()))).style(
            {
                let style = Style::default();

                match state.emphasis {
                    RowEmphasis::Low => style
                        .add_modifier(Modifier::DIM | Modifier::CROSSED_OUT),
                    RowEmphasis::Medium => style,
                    RowEmphasis::High => style.fg(Color::Yellow),
                }
            },
        )
    }

    fn constraints(table_type: &TableType) -> Vec<Constraint> {
        match table_type {
            TableType::Projects => {
                vec![Constraint::Percentage(80), Constraint::Percentage(20)]
            }
            TableType::Tasks => {
                vec![Constraint::Percentage(10), Constraint::Percentage(90)]
            }
        }
    }
}
