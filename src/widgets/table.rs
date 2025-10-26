use crate::states::{RowEmphasis, RowState, TableState, TableType};
use crate::widgets::Block;

use ratatui::{
    layout::Rect,
    prelude::*,
    style::{Color, Modifier, Style, Stylize},
    widgets,
    widgets::{Cell, Row},
};

pub struct Table<'a> {
    pub state: &'a TableState,
}

impl<'a> Widget for Table<'a> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let table = widgets::Table::new(self.rows(), self.constraints())
            .header(self.header())
            .block(self.block())
            .row_highlight_style(if self.state.is_focused {
                Style::default().bg(Color::Yellow).fg(Color::White)
            } else {
                Style::default()
            });

        let mut state = widgets::TableState::new();
        state.select(self.state.selected_row);

        <widgets::Table as StatefulWidget>::render(
            table, area, buffer, &mut state,
        );
    }
}

impl<'a> Table<'a> {
    fn block(&self) -> widgets::Block<'static> {
        Block {
            title: self.state.title().clone(),
            is_focused: self.state.is_focused,
            ..Default::default()
        }
        .into_widget()
    }

    fn header(&self) -> Row<'static> {
        let column_names = self.state.column_names();
        let cells = column_names.iter().map(|column_name| {
            let cell = Cell::from(column_name.clone());
            let style = Style::default();

            if self.state.is_focused {
                cell.style(style.fg(Color::Yellow))
            } else {
                cell.style(style)
            }
        });
        Row::new(cells)
            .bottom_margin(1)
            .style(if self.state.is_focused {
                Style::default().bold()
            } else {
                Style::default()
            })
    }

    fn rows(&self) -> Vec<Row<'static>> {
        self.state.rows.iter().map(|row| self.row(row)).collect()
    }

    fn row(&self, row_state: &RowState) -> Row<'static> {
        Row::new(row_state.cells.iter().map(|cell| Cell::from(cell.clone())))
            .style({
                let style = Style::default();

                match row_state.emphasis {
                    RowEmphasis::Low => {
                        style.fg(Color::DarkGray).add_modifier(Modifier::DIM)
                    }
                    RowEmphasis::Medium => style,
                    RowEmphasis::High => {
                        if self.state.is_focused {
                            style.fg(Color::Yellow)
                        } else {
                            style
                        }
                    }
                }
            })
    }

    fn constraints(&self) -> Vec<Constraint> {
        match self.state.table_type {
            TableType::Projects => {
                vec![Constraint::Length(2), Constraint::Min(1)]
            }
            TableType::Tasks => {
                vec![Constraint::Percentage(10), Constraint::Percentage(90)]
            }
        }
    }
}
