use {
    ratatui::{
        Frame,
        layout::{Alignment, Rect},
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Padding, Paragraph, Widget, Wrap},
    },
    ratatui_opentui_loader::KittLoader,
};

use {super::render_bordered_panel, crate::theme::ColorTheme};

pub fn render_input(
    frame: &mut Frame,
    area: Rect,
    input: &str,
    focused: bool,
    blink_on: bool,
    t: &ColorTheme,
) {
    let buf = frame.buffer_mut();
    let inner = render_bordered_panel(
        buf,
        area,
        t.primary,
        Some(t.bg_element),
        Padding::new(1, 1, 1, 1),
    );
    if inner.is_empty() {
        return;
    }

    let mut lines: Vec<Line> = input
        .split('\n')
        .map(|l| Line::styled(l, Style::new().fg(t.text)))
        .collect();
    let cursor = if focused {
        if blink_on {
            "\u{2588}"
        } else {
            " "
        }
    } else {
        "\u{2592}"
    };
    let cursor_style = if focused {
        Style::new().fg(t.text)
    } else {
        Style::new().fg(t.text_muted)
    };
    if let Some(last) = lines.last_mut() {
        last.spans.push(Span::styled(cursor, cursor_style));
    } else {
        lines.push(Line::styled(cursor, cursor_style));
    }
    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .render(inner, buf);

    if inner.height >= 2 {
        let label_area = Rect::new(inner.x, inner.y + inner.height - 1, inner.width, 1);
        let label = Line::from(vec![
            Span::styled("Build", Style::new().fg(t.primary).bold()),
            Span::styled(" \u{00b7} ", Style::new().fg(t.text_muted)),
            Span::styled("GPT-5.5", Style::new().fg(t.text)),
            Span::styled(" ", Style::new().fg(t.text_muted)),
            Span::styled("OpenAI", Style::new().fg(t.text_muted)),
        ]);
        Paragraph::new(label).render(label_area, buf);
    }
}

pub fn render_status_bar(
    frame: &mut Frame,
    area: Rect,
    loader: &KittLoader,
    t: &ColorTheme,
    show_loader: bool,
) {
    frame.render_widget(Block::default().style(Style::new().bg(t.bg)), area);
    let left = if show_loader {
        let loader_line = loader.into_line(10);
        Line::from(
            [vec![Span::raw("   ")], loader_line.spans.clone(), vec![
                Span::raw("  "),
                Span::styled("esc ", Style::new().fg(t.text_muted).bold()),
                Span::styled("interrupt", Style::new().fg(t.text_muted)),
            ]]
            .concat(),
        )
    } else {
        Line::from(vec![Span::raw("   ")])
    };
    frame.render_widget(Paragraph::new(left), area);
    let right = Line::from(vec![
        Span::styled("170.0K (42%)", Style::new().fg(t.text_muted)),
        Span::raw("  "),
        Span::styled("ctrl+p ", Style::new().fg(t.text_muted).bold()),
        Span::styled("commands", Style::new().fg(t.text_muted)),
        Span::raw("  "),
    ]);
    frame.render_widget(Paragraph::new(right).alignment(Alignment::Right), area);
}
