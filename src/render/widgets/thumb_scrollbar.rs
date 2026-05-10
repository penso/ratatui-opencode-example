#![allow(dead_code)]

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

#[derive(Clone, Copy, Debug)]
pub struct ThumbScrollbar {
    scroll: u16,
    content_height: u16,
    track_color: Color,
    thumb_color: Color,
    right_padding: u16,
}

impl ThumbScrollbar {
    pub const fn new(scroll: u16, content_height: u16) -> Self {
        Self {
            scroll,
            content_height,
            track_color: Color::Reset,
            thumb_color: Color::Reset,
            right_padding: 0,
        }
    }

    #[must_use]
    pub const fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    #[must_use]
    pub const fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = color;
        self
    }

    #[must_use]
    pub const fn right_padding(mut self, columns: u16) -> Self {
        self.right_padding = columns;
        self
    }
}

impl Widget for ThumbScrollbar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() || self.content_height <= area.height || self.right_padding >= area.width
        {
            return;
        }

        let track_height = area.height;
        let thumb_height = ((u32::from(track_height) * u32::from(track_height))
            / u32::from(self.content_height))
        .max(1)
        .min(u32::from(track_height)) as u16;
        let max_scroll = self.content_height.saturating_sub(track_height);
        let max_thumb_top = track_height.saturating_sub(thumb_height);
        let thumb_top = if max_scroll == 0 {
            0
        } else {
            ((u32::from(self.scroll) * u32::from(max_thumb_top)) / u32::from(max_scroll)) as u16
        };

        let x = area.x + area.width - 1 - self.right_padding;
        for y in area.y..area.y + area.height {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_symbol(" ")
                    .set_style(Style::new().bg(self.track_color));
            }
        }
        for y in area.y + thumb_top..area.y + thumb_top + thumb_height {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_symbol(" ")
                    .set_style(Style::new().bg(self.thumb_color));
            }
        }
    }
}
