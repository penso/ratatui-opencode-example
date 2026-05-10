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
        event::{
            self, DisableMouseCapture, EnableMouseCapture, KeyboardEnhancementFlags,
            PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
        },
        execute,
    },
    ratatui::{
        Frame,
        buffer::Buffer,
        layout::{Constraint, Layout, Margin, Rect},
        style::{Style, Stylize},
        text::{Line, Span, Text},
        widgets::{Block, Paragraph, Widget, Wrap},
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
    execute!(
        io::stdout(),
        EnableMouseCapture,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    )?;
    let result = run(&mut terminal);
    execute!(
        io::stdout(),
        PopKeyboardEnhancementFlags,
        DisableMouseCapture
    )?;
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

    let show_sidebar = app.view.sidebar_visible && area.width >= 84;
    let (main_area, sidebar_area) = if show_sidebar {
        let sidebar_width = 42.min(area.width.saturating_sub(42));
        let [main_area, _gap, sidebar_area] = Layout::horizontal([
            Constraint::Min(40),
            Constraint::Length(1),
            Constraint::Length(sidebar_width),
        ])
        .areas(area);
        (main_area, Some(sidebar_area))
    } else {
        (area, None)
    };

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
        Constraint::Length(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(main_area);

    let padded_messages = if app.view.scrollbar_visible {
        Rect::new(
            messages_area.x + 2,
            messages_area.y,
            messages_area.width.saturating_sub(2),
            messages_area.height,
        )
    } else {
        messages_area.inner(Margin::new(2, 0))
    };
    let padded_input = if app.view.scrollbar_visible {
        Rect::new(
            input_area.x + 2,
            input_area.y,
            input_area.width.saturating_sub(3),
            input_area.height,
        )
    } else {
        input_area.inner(Margin::new(2, 0))
    };
    let scrollbar_width = if app.view.scrollbar_visible {
        4
    } else {
        0
    };
    let content_width = padded_messages.width.saturating_sub(scrollbar_width);

    let block_heights: Vec<u16> = app
        .messages
        .iter()
        .map(|m| message_height(m, content_width))
        .collect();
    let total_height: u16 = block_heights.iter().sum();

    let max_scroll = total_height.saturating_sub(padded_messages.height);
    app.scroll_offset = app.scroll_offset.min(max_scroll);
    let scroll = app.scroll_offset;

    if app.view.scrollbar_visible {
        render_scrollbar(
            frame,
            padded_messages,
            scroll,
            total_height,
            t.bg_element,
            t.border,
        );
    }

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
        app::Mode::CommandPalette => render_command_palette(frame, main_area, app),
        app::Mode::ThemePicker => render_theme_modal(frame, main_area, app),
        app::Mode::Normal | app::Mode::SlashMenu => {},
    }

    if let Some(sidebar_area) = sidebar_area {
        render_sidebar(frame, sidebar_area, app);
    }
}

fn render_scrollbar(
    frame: &mut Frame,
    area: Rect,
    scroll: u16,
    total_height: u16,
    track_color: ratatui::style::Color,
    color: ratatui::style::Color,
) {
    if area.is_empty() || total_height <= area.height {
        return;
    }

    let track_height = area.height;
    let thumb_height = ((u32::from(track_height) * u32::from(track_height))
        / u32::from(total_height))
    .max(1)
    .min(u32::from(track_height)) as u16;
    let max_scroll = total_height.saturating_sub(track_height);
    let max_thumb_top = track_height.saturating_sub(thumb_height);
    let thumb_top = if max_scroll == 0 {
        0
    } else {
        ((u32::from(scroll) * u32::from(max_thumb_top)) / u32::from(max_scroll)) as u16
    };

    let x = area.x + area.width.saturating_sub(2);
    let buf = frame.buffer_mut();
    for y in area.y..area.y + area.height {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_symbol(" ").set_style(Style::new().bg(track_color));
        }
    }
    for y in area.y + thumb_top..area.y + thumb_top + thumb_height {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_symbol(" ").set_style(Style::new().bg(color));
        }
    }
}

fn render_sidebar(frame: &mut Frame, area: Rect, app: &App) {
    let t = app.theme;
    frame.render_widget(Block::new().style(Style::new().bg(t.bg_panel)), area);

    let inner = area.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    if inner.is_empty() {
        return;
    }

    let footer_height = 2.min(inner.height);
    let [content_area, footer_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(footer_height)]).areas(inner);

    let content = Text::from(vec![
        Line::from(Span::styled(
            "New TUI architecture for Polyphony",
            Style::new().fg(t.text).bold(),
        )),
        Line::raw(""),
        Line::from(Span::styled("Context", Style::new().fg(t.text).bold())),
        Line::styled("36,251 tokens", Style::new().fg(t.text_muted)),
        Line::styled("9% used", Style::new().fg(t.text_muted)),
        Line::styled("$0.00 spent", Style::new().fg(t.text_muted)),
        Line::raw(""),
        Line::from(Span::styled("MCP", Style::new().fg(t.text).bold())),
        Line::from(vec![
            Span::styled("• ", Style::new().fg(t.error)),
            Span::styled("linear ", Style::new().fg(t.text)),
            Span::styled(
                "SSE error: Non-200 status code (405)",
                Style::new().fg(t.text_muted).italic(),
            ),
        ]),
        Line::raw(""),
        Line::from(Span::styled("LSP", Style::new().fg(t.text).bold())),
        Line::styled(
            "LSPs will activate as files are read",
            Style::new().fg(t.text_muted),
        ),
    ]);
    Paragraph::new(content)
        .wrap(Wrap { trim: false })
        .style(Style::new().bg(t.bg_panel))
        .render(content_area, frame.buffer_mut());

    let footer = Line::from(vec![
        Span::styled("• ", Style::new().fg(t.primary)),
        Span::styled("Open", Style::new().fg(t.text_muted).bold()),
        Span::styled("Code ", Style::new().fg(t.text).bold()),
        Span::styled("1.14.41", Style::new().fg(t.text_muted)),
    ]);
    Paragraph::new(footer)
        .style(Style::new().bg(t.bg_panel))
        .render(footer_area, frame.buffer_mut());
}
