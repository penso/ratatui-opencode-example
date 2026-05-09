use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Padding, Paragraph, Widget, Wrap},
};

use {
    super::LeftBorderPanel,
    crate::{
        messages::{MessageBlock, TOOL_MAX_OUTPUT_LINES},
        theme::ColorTheme,
    },
};

pub fn render_message_block_to_buf(
    buf: &mut Buffer,
    msg: &MessageBlock,
    area: Rect,
    hovered: bool,
    t: &ColorTheme,
) {
    match msg {
        MessageBlock::AssistantText(text) => render_assistant_text(buf, text, area, t),
        MessageBlock::UserMessage(text) => render_user_message(buf, text, area, t),
        MessageBlock::Thinking { summary, body } => {
            render_thinking_block(buf, summary, body, area, t);
        },
        MessageBlock::ToolOutput {
            title,
            command,
            output,
            collapsed,
        } => {
            render_tool_output(buf, title, command, output, *collapsed, hovered, area, t);
        },
        MessageBlock::InlineResult { icon, label } => {
            render_inline_result(buf, icon, label, area, t);
        },
        MessageBlock::Error(text) => render_error_block(buf, text, area, t),
    }
}

fn render_user_message(buf: &mut Buffer, text: &str, area: Rect, t: &ColorTheme) {
    if area.height < 2 {
        return;
    }
    let block_area = Rect::new(
        area.x,
        area.y + 1,
        area.width,
        area.height.saturating_sub(1),
    );
    let inner = LeftBorderPanel::new()
        .border_color(t.primary)
        .content_bg(t.bg_user)
        .padding(Padding::new(1, 1, 1, 1))
        .render(block_area, buf);
    if inner.is_empty() {
        return;
    }
    Paragraph::new(Text::styled(text, Style::new().fg(t.text)))
        .wrap(Wrap { trim: false })
        .render(inner, buf);
}

fn render_assistant_text(buf: &mut Buffer, text: &str, area: Rect, t: &ColorTheme) {
    if area.height < 2 {
        return;
    }
    let inner = Rect::new(
        area.x + 3,
        area.y + 1,
        area.width.saturating_sub(3),
        area.height.saturating_sub(1),
    );
    Paragraph::new(Text::styled(text, Style::new().fg(t.text)))
        .wrap(Wrap { trim: false })
        .render(inner, buf);
}

fn render_thinking_block(buf: &mut Buffer, summary: &str, body: &str, area: Rect, t: &ColorTheme) {
    if area.height < 2 {
        return;
    }
    let block_area = Rect::new(
        area.x,
        area.y + 1,
        area.width,
        area.height.saturating_sub(1),
    );
    let inner = LeftBorderPanel::new()
        .border_color(t.primary)
        .padding(Padding::new(1, 0, 0, 0))
        .render(block_area, buf);
    if inner.is_empty() {
        return;
    }
    let header = Line::from(vec![
        Span::styled("Thinking: ", Style::new().fg(t.primary).italic()),
        Span::styled(summary, Style::new().fg(t.primary).italic()),
    ]);
    let mut lines = vec![header, Line::raw("")];
    for l in body.split('\n') {
        lines.push(Line::styled(l, Style::new().fg(t.text)));
    }
    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .render(inner, buf);
}

fn render_tool_output(
    buf: &mut Buffer,
    title: &str,
    command: &str,
    output: &[String],
    collapsed: bool,
    hovered: bool,
    area: Rect,
    t: &ColorTheme,
) {
    if area.height < 2 {
        return;
    }
    let block_area = Rect::new(
        area.x,
        area.y + 1,
        area.width,
        area.height.saturating_sub(1),
    );
    let is_expandable = collapsed && output.len() > TOOL_MAX_OUTPUT_LINES;
    let bg = if hovered && is_expandable {
        t.bg_hover
    } else {
        t.bg_panel
    };
    let inner = LeftBorderPanel::new()
        .border_color(t.border)
        .content_bg(bg)
        .padding(Padding::new(1, 1, 1, 1))
        .render(block_area, buf);
    if inner.is_empty() {
        return;
    }
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(vec![
        Span::styled("# ", Style::new().fg(t.text_muted)),
        Span::styled(title, Style::new().fg(t.text_muted)),
    ]));
    if !command.is_empty() {
        lines.push(Line::raw(""));
        lines.push(Line::from(vec![
            Span::styled("$ ", Style::new().fg(t.text_muted)),
            Span::styled(command, Style::new().fg(t.text).bold()),
        ]));
    }
    lines.push(Line::raw(""));
    let truncated = collapsed && output.len() > TOOL_MAX_OUTPUT_LINES;
    let vis = if truncated {
        &output[..TOOL_MAX_OUTPUT_LINES]
    } else {
        output
    };
    for line in vis {
        lines.push(Line::styled(line.as_str(), Style::new().fg(t.text)));
    }
    if truncated {
        lines.push(Line::styled("\u{2026}", Style::new().fg(t.text_muted)));
        lines.push(Line::raw(""));
        lines.push(Line::styled(
            "Click to expand",
            Style::new().fg(t.text_muted),
        ));
    }
    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .render(inner, buf);
}

fn render_inline_result(buf: &mut Buffer, icon: &str, label: &str, area: Rect, t: &ColorTheme) {
    if area.height < 2 {
        return;
    }
    let line = Line::from(vec![
        Span::raw("  "),
        Span::styled(format!("{icon} "), Style::new().fg(t.text_muted)),
        Span::styled(label, Style::new().fg(t.text_muted)),
    ]);
    let y = (area.y + 1).min(area.y + area.height - 1);
    line.render(Rect::new(area.x, y, area.width, 1), buf);
}

fn render_error_block(buf: &mut Buffer, text: &str, area: Rect, t: &ColorTheme) {
    if area.height < 2 {
        return;
    }
    let block_area = Rect::new(
        area.x,
        area.y + 1,
        area.width,
        area.height.saturating_sub(1),
    );
    let inner = LeftBorderPanel::new()
        .border_color(t.error)
        .padding(Padding::new(1, 0, 0, 0))
        .render(block_area, buf);
    Paragraph::new(Text::styled(text, Style::new().fg(t.error)))
        .wrap(Wrap { trim: false })
        .render(inner, buf);
}
