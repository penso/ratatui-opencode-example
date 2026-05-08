mod app;
mod commands;
mod messages;
mod render;
mod theme;

use std::{
    io,
    time::{Duration, Instant},
};

use {
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture},
        execute,
    },
    ratatui::{
        Frame,
        buffer::Buffer,
        layout::{Constraint, Layout, Margin, Rect},
        style::Style,
        widgets::Block,
    },
};

use {
    app::{App, INPUT_HEIGHT, handle_event},
    messages::message_height,
    render::{
        render_command_palette, render_input, render_message_block_to_buf, render_slash_menu,
        render_status_bar, render_theme_modal,
    },
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    execute!(io::stdout(), EnableMouseCapture)?;
    let result = run(&mut terminal);
    execute!(io::stdout(), DisableMouseCapture)?;
    ratatui::restore();
    while event::poll(Duration::from_millis(10)).unwrap_or(false) {
        let _ = event::read();
    }
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();
    let tick_rate = Duration::from_millis(40);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| draw(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            loop {
                let ev = event::read()?;
                if handle_event(&mut app, ev) {
                    return Ok(());
                }
                if !event::poll(Duration::ZERO)? {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.loader.tick();
            app.tick_count += 1;
            if app.tick_count.is_multiple_of(13) {
                app.blink_on = !app.blink_on;
            }
            if app.loader_visible_ticks > 0 {
                app.loader_visible_ticks -= 1;
            }
            last_tick = Instant::now();
        }
    }
}

#[allow(clippy::too_many_lines)]
fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let t = app.theme;

    frame.render_widget(Block::default().style(Style::new().bg(t.bg)), area);

    let [
        messages_area,
        _msg_gap,
        input_area,
        _gap_top,
        status_area,
        _gap_bottom,
    ] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(INPUT_HEIGHT),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(area);

    let padded_messages = messages_area.inner(Margin::new(2, 0));
    let padded_input = input_area.inner(Margin::new(2, 0));
    let content_width = padded_messages.width;

    let block_heights: Vec<u16> = app
        .messages
        .iter()
        .map(|m| message_height(m, content_width))
        .collect();
    let total_height: u16 = block_heights.iter().sum();

    let max_scroll = total_height.saturating_sub(padded_messages.height);
    app.scroll_offset = app.scroll_offset.min(max_scroll);
    let scroll = app.scroll_offset;

    app.tool_block_rects.clear();

    // Render visible message blocks
    let mut y_cursor: i32 = -i32::from(scroll);
    for (i, msg) in app.messages.iter().enumerate() {
        let h = block_heights[i];
        let block_top = y_cursor;
        let block_bottom = y_cursor + i32::from(h);

        if block_bottom <= 0 {
            y_cursor += i32::from(h);
            continue;
        }
        if block_top >= i32::from(padded_messages.height) {
            break;
        }

        let clip_top = (-block_top).max(0) as u16;
        let viewport_y = i32::from(padded_messages.y) + block_top.max(0);
        let visible_h =
            (h - clip_top).min((i32::from(padded_messages.height) - block_top.max(0)) as u16);
        if visible_h == 0 {
            y_cursor += i32::from(h);
            continue;
        }

        let dest_rect = Rect::new(
            padded_messages.x,
            viewport_y as u16,
            content_width,
            visible_h,
        );

        if matches!(msg, messages::MessageBlock::ToolOutput { .. }) {
            app.tool_block_rects.push((i, dest_rect));
        }

        let hovered = matches!(msg, messages::MessageBlock::ToolOutput { .. })
            && app
                .tool_block_rects
                .last()
                .is_some_and(|&(_, r)| r.contains(app.mouse_pos.into()));

        if clip_top > 0 {
            let full_rect = Rect::new(0, 0, content_width, h);
            let mut temp_buf = Buffer::empty(full_rect);
            for cell in &mut temp_buf.content {
                cell.set_style(Style::new().bg(t.bg));
            }
            render_message_block_to_buf(&mut temp_buf, msg, full_rect, hovered, &t);
            let frame_buf = frame.buffer_mut();
            for row in 0..visible_h {
                for col in 0..content_width {
                    let Some(src) = temp_buf.cell((col, clip_top + row)) else {
                        continue;
                    };
                    if let Some(dst) = frame_buf.cell_mut((dest_rect.x + col, dest_rect.y + row)) {
                        *dst = src.clone();
                    }
                }
            }
        } else {
            render::render_message_block(frame, msg, dest_rect, hovered, &t);
        }

        y_cursor += i32::from(h);
    }

    // Overlays based on current mode
    let effective_mode = app.effective_mode();
    if *effective_mode == app::Mode::SlashMenu {
        let slash_area = Rect::new(
            padded_input.x,
            messages_area.y,
            padded_input.width,
            input_area.y.saturating_sub(messages_area.y),
        );
        render_slash_menu(frame, slash_area, app, &t);
    }

    let input_focused = *effective_mode != app::Mode::CommandPalette;
    render_input(
        frame,
        padded_input,
        &app.input,
        input_focused,
        app.blink_on,
        &t,
    );

    let show_loader = app.loader_visible_ticks > 0;
    render_status_bar(frame, status_area, &app.loader, &t, show_loader);

    match &app.mode {
        app::Mode::CommandPalette => render_command_palette(frame, area, app),
        app::Mode::ThemePicker => render_theme_modal(frame, area, app),
        app::Mode::Normal | app::Mode::SlashMenu => {},
    }
}
