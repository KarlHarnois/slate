use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    widgets,
    widgets::{BorderType, Borders, Padding},
};

pub struct Block {
    pub title: String,
    pub is_focused: bool,
    pub title_alignment: Alignment,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            title: String::new(),
            is_focused: false,
            title_alignment: Alignment::Left,
        }
    }
}

impl Block {
    pub fn into_widget<'a>(self) -> widgets::Block<'a> {
        widgets::Block::default()
            .title(format!(" {} ", self.title))
            .title_alignment(self.title_alignment)
            .title_style(self.title_style())
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(self.border_style())
            .padding(Padding::new(1, 0, 0, 0))
    }

    fn title_style(&self) -> Style {
        if self.is_focused {
            Style::default().bold()
        } else {
            Style::default()
        }
    }

    fn border_style(&self) -> Style {
        if self.is_focused {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        }
    }
}
