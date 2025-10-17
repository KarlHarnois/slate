use crate::components::Block;
use crate::states::{RowEmphasis, RowState, TableState, TableType};

use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style, Stylize},
    widgets,
    widgets::{Cell, Row},
};

pub struct Table<'a> {
    pub state: &'a TableState,
}

impl<'a> Table<'a> {
    pub fn into_widget(self) -> widgets::Table<'static> {
        widgets::Table::new(self.rows(), self.constraints())
            .header(self.header())
            .block(
                Block {
                    title: self.state.title().clone(),
                    is_focused: self.state.is_focused,
                }
                .into_widget(),
            )
            .row_highlight_style(Style::default().reversed())
            .row_highlight_style(if self.state.is_focused {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            })
    }

    fn header(&self) -> Row<'static> {
        let cells = self.state.header.iter().map(|column_name| {
            let cell = Cell::from(column_name.clone());
            let style = Style::default();

            if self.state.is_focused {
                cell.style(style.fg(Color::Yellow))
            } else {
                cell.style(style)
            }
        });
        Row::new(cells).style(Style::new().bold()).bottom_margin(1)
    }

    fn rows(&self) -> Vec<Row<'static>> {
        self.state.rows.iter().map(|row| self.row(row)).collect()
    }

    fn row(&self, row_state: &RowState) -> Row<'static> {
        Row::new(row_state.cells.iter().map(|cell| Cell::from(cell.clone())))
            .style({
                let style = Style::default();

                match row_state.emphasis {
                    RowEmphasis::Low => style
                        .add_modifier(Modifier::DIM | Modifier::CROSSED_OUT),
                    RowEmphasis::Medium => style,
                    RowEmphasis::High => style.fg(Color::Yellow),
                }
            })
    }

    fn constraints(&self) -> Vec<Constraint> {
        match self.state.table_type {
            TableType::Projects => {
                vec![Constraint::Percentage(80), Constraint::Percentage(20)]
            }
            TableType::Tasks => {
                vec![Constraint::Percentage(10), Constraint::Percentage(90)]
            }
        }
    }
}
