use ratatui::{
    style::{Color, Style, Stylize},
    widgets,
    widgets::{BorderType, Borders, Padding},
};

pub struct Block {
    pub title: String,
    pub is_focused: bool,
}

impl Block {
    pub fn into_widget<'a>(self) -> widgets::Block<'a> {
        widgets::Block::default()
            .title(format!(" {} ", self.title))
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
