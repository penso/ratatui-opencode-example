mod blocks;
mod input;
mod modals;

pub use {
    blocks::render_message_block_to_buf,
    input::{render_input, render_status_bar},
    modals::{render_command_palette, render_slash_menu, render_theme_modal},
};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Widget},
};

use crate::theme::ColorTheme;

/// Renders a left ┃ border outside the content background.
/// The border cell has no background fill (inherits terminal bg).
/// The content area gets its own distinct background.
pub fn render_bordered_panel(
    buf: &mut Buffer,
    area: Rect,
    border_color: Color,
    content_bg: Option<Color>,
    padding: Padding,
) -> Rect {
    let border_block = Block::new()
        .borders(Borders::LEFT)
        .border_type(BorderType::Thick)
        .border_style(Style::new().fg(border_color));

    let after_border = border_block.inner(area);
    border_block.render(area, buf);

    let mut content_block = Block::new().padding(padding);
    if let Some(bg) = content_bg {
        content_block = content_block.style(Style::new().bg(bg));
    }

    let inner = content_block.inner(after_border);
    content_block.render(after_border, buf);

    inner
}

/// Renders a message block, dispatching to the appropriate block renderer.
/// Also used by the clipping logic (via temp buffer).
pub fn render_message_block(
    frame: &mut ratatui::Frame,
    msg: &crate::messages::MessageBlock,
    area: Rect,
    hovered: bool,
    t: &ColorTheme,
) {
    let buf = frame.buffer_mut();
    render_message_block_to_buf(buf, msg, area, hovered, t);
}
