use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Widget},
};

/// Renders a left border outside the content background.
/// The border cell has no background fill, so it inherits the terminal background.
#[derive(Clone, Copy, Debug)]
pub struct LeftBorderPanel {
    border_color: Color,
    content_bg: Option<Color>,
    padding: Padding,
}

impl LeftBorderPanel {
    pub const fn new() -> Self {
        Self {
            border_color: Color::Reset,
            content_bg: None,
            padding: Padding::ZERO,
        }
    }

    #[must_use]
    pub const fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    #[must_use]
    pub const fn content_bg(mut self, color: Color) -> Self {
        self.content_bg = Some(color);
        self
    }

    #[must_use]
    pub const fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn render(self, area: Rect, buf: &mut Buffer) -> Rect {
        let border_block = Block::new()
            .borders(Borders::LEFT)
            .border_type(BorderType::Thick)
            .border_style(Style::new().fg(self.border_color));

        let after_border = border_block.inner(area);
        border_block.render(area, buf);

        let mut content_block = Block::new().padding(self.padding);
        if let Some(bg) = self.content_bg {
            content_block = content_block.style(Style::new().bg(bg));
        }

        let inner = content_block.inner(after_border);
        content_block.render(after_border, buf);

        inner
    }
}

impl Default for LeftBorderPanel {
    fn default() -> Self {
        Self::new()
    }
}
