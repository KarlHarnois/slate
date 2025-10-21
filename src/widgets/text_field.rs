use crate::states::TextInput;
use crate::widgets::Block;

use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Clear, Widget},
};

pub struct TextField {
    pub title: String,
    pub text: TextInput,
}

impl Widget for TextField {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        Clear.render(area, buffer);

        self.block().into_widget().render(area, buffer);

        let inner_area = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });

        let [icon_column, _, text_column] =
            self.horizontal_constraints().areas(inner_area);

        buffer.set_string(
            icon_column.x,
            inner_area.y,
            ">",
            Style::default().fg(Color::DarkGray),
        );

        let max_width = text_column.width as usize;

        buffer.set_stringn(
            text_column.x,
            inner_area.y,
            self.text.value,
            max_width,
            Style::default(),
        );
    }
}

impl TextField {
    pub fn block(&self) -> Block {
        Block {
            title: self.title.to_string(),
            is_focused: true,
            ..Default::default()
        }
    }

    pub fn horizontal_constraints(&self) -> Layout {
        Layout::horizontal([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
    }
}
