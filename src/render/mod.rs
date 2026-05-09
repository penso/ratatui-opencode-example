mod blocks;
mod input;
mod modals;
mod widgets;

pub use {
    blocks::render_message_block_to_buf,
    input::{render_input, render_status_bar},
    modals::{render_command_palette, render_slash_menu, render_theme_modal},
    widgets::{InputBottomPanel, LeftBorderPanel},
};

use ratatui::layout::Rect;

use crate::theme::ColorTheme;

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
