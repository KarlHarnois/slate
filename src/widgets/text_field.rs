use crate::states::TextInput;
use crate::widgets::Block;

use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Clear, Widget},
};

pub struct TextField<'a> {
    pub title: String,
    pub text: &'a TextInput,
}

impl<'a> Widget for TextField<'a> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        Clear.render(area, buffer);

        self.block().into_widget().render(area, buffer);

        let inner_area = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });

        let max_width = inner_area.width as usize;

        buffer.set_stringn(
            inner_area.x,
            inner_area.y,
            self.text.value.clone(),
            max_width,
            Style::default(),
        );
    }
}

impl<'a> TextField<'a> {
    pub fn block(&self) -> Block {
        Block {
            title: self.title.to_string(),
            is_focused: true,
            ..Default::default()
        }
    }
}
