use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Padding, Paragraph, Widget, Wrap},
};

use super::LeftBorderPanel;

#[derive(Clone, Copy, Debug)]
pub struct InputBottomPanel<'a> {
    input: &'a str,
    focused: bool,
    blink_on: bool,
    border_color: Color,
    content_bg: Color,
    text_color: Color,
    muted_color: Color,
    label_accent: Color,
    bottom_half_bg: Color,
    padding: Padding,
    label_name: &'a str,
    label_model: &'a str,
    label_provider: &'a str,
}

impl<'a> InputBottomPanel<'a> {
    pub const fn new(input: &'a str) -> Self {
        Self {
            input,
            focused: false,
            blink_on: false,
            border_color: Color::Reset,
            content_bg: Color::Reset,
            text_color: Color::Reset,
            muted_color: Color::Reset,
            label_accent: Color::Reset,
            bottom_half_bg: Color::Reset,
            padding: Padding::ZERO,
            label_name: "",
            label_model: "",
            label_provider: "",
        }
    }

    #[must_use]
    pub const fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    #[must_use]
    pub const fn blink_on(mut self, blink_on: bool) -> Self {
        self.blink_on = blink_on;
        self
    }

    #[must_use]
    pub const fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    #[must_use]
    pub const fn content_bg(mut self, color: Color) -> Self {
        self.content_bg = color;
        self
    }

    #[must_use]
    pub const fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    #[must_use]
    pub const fn muted_color(mut self, color: Color) -> Self {
        self.muted_color = color;
        self
    }

    #[must_use]
    pub const fn label_accent(mut self, color: Color) -> Self {
        self.label_accent = color;
        self
    }

    #[must_use]
    pub const fn bottom_half_bg(mut self, color: Color) -> Self {
        self.bottom_half_bg = color;
        self
    }

    #[must_use]
    pub const fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    #[must_use]
    pub const fn label(mut self, name: &'a str, model: &'a str, provider: &'a str) -> Self {
        self.label_name = name;
        self.label_model = model;
        self.label_provider = provider;
        self
    }

    pub fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = LeftBorderPanel::new()
            .border_color(self.border_color)
            .content_bg(self.content_bg)
            .padding(self.padding)
            .render(area, buf);
        if inner.is_empty() {
            return;
        }

        Paragraph::new(self.input_lines())
            .wrap(Wrap { trim: false })
            .render(inner, buf);

        if inner.height >= 3 {
            let label_area = Rect::new(inner.x, inner.y + inner.height - 2, inner.width, 1);
            Paragraph::new(self.label_line()).render(label_area, buf);

            let half_space_y = inner.y + inner.height - 1;
            let content_x = area.x.saturating_add(1);
            for x in content_x..area.x + area.width {
                if let Some(cell) = buf.cell_mut((x, half_space_y)) {
                    cell.set_symbol("▀")
                        .set_style(Style::new().fg(self.content_bg).bg(self.bottom_half_bg));
                }
            }
            if let Some(cell) = buf.cell_mut((area.x, half_space_y)) {
                cell.set_symbol("╹")
                    .set_style(Style::new().fg(self.border_color).bg(self.bottom_half_bg));
            }
        }
    }

    fn input_lines(self) -> Vec<Line<'a>> {
        let mut lines: Vec<Line> = self
            .input
            .split('\n')
            .map(|line| Line::styled(line, Style::new().fg(self.text_color)))
            .collect();
        let cursor = if self.focused {
            if self.blink_on {
                "\u{2588}"
            } else {
                " "
            }
        } else {
            "\u{2592}"
        };
        let cursor_style = if self.focused {
            Style::new().fg(self.text_color)
        } else {
            Style::new().fg(self.muted_color)
        };
        if let Some(last) = lines.last_mut() {
            last.spans.push(Span::styled(cursor, cursor_style));
        } else {
            lines.push(Line::styled(cursor, cursor_style));
        }

        lines
    }

    fn label_line(self) -> Line<'a> {
        Line::from(vec![
            Span::styled(self.label_name, Style::new().fg(self.label_accent)),
            Span::styled(" \u{00b7} ", Style::new().fg(self.muted_color)),
            Span::styled(self.label_model, Style::new().fg(self.text_color)),
            Span::styled(" ", Style::new().fg(self.muted_color)),
            Span::styled(self.label_provider, Style::new().fg(self.muted_color)),
        ])
    }
}
