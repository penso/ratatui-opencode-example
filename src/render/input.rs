use {
    ratatui::{
        Frame,
        layout::{Alignment, Rect},
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Padding, Paragraph},
    },
    ratatui_opentui_loader::KittLoader,
};

use {
    super::InputBottomPanel,
    crate::{app::AgentMode, theme::ColorTheme},
};

pub fn render_input(
    frame: &mut Frame,
    area: Rect,
    input: &str,
    focused: bool,
    blink_on: bool,
    agent_mode: AgentMode,
    t: &ColorTheme,
) {
    let accent = agent_mode.accent(t);
    let buf = frame.buffer_mut();
    InputBottomPanel::new(input)
        .focused(focused)
        .blink_on(blink_on)
        .border_color(accent)
        .content_bg(t.bg_element)
        .text_color(t.text)
        .muted_color(t.text_muted)
        .label_accent(accent)
        .bottom_half_bg(t.bg)
        .padding(Padding::new(1, 1, 1, 0))
        .agent_mode(agent_mode)
        .model("GPT-5.5", "OpenAI")
        .render(area, buf);
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
            [
                vec![Span::raw("   ")],
                loader_line.spans.clone(),
                vec![
                    Span::raw("  "),
                    Span::styled("esc ", Style::new().fg(t.text_muted).bold()),
                    Span::styled("interrupt", Style::new().fg(t.text_muted)),
                ],
            ]
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
        Span::raw(" "),
    ]);
    frame.render_widget(Paragraph::new(right).alignment(Alignment::Right), area);
}
