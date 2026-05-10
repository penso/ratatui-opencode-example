use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Clear, Paragraph, Widget},
};

use crate::{
    app::App,
    commands::{SLASH_MAX_VISIBLE, filtered_slash_commands},
    theme::{self, ColorTheme},
};

#[allow(clippy::too_many_lines)]
pub fn render_command_palette(frame: &mut Frame, area: Rect, app: &mut App) {
    let t = app.theme;
    let buf = frame.buffer_mut();
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_style(Style::new().fg(t.text_muted).bg(t.bg));
            }
        }
    }
    let modal_w = 50u16.min(area.width.saturating_sub(4));
    let filtered = app.filtered_commands();
    let items_height = filtered.len() as u16;
    let mut section_count = 0u16;
    let mut last_section = "";
    for cmd in &filtered {
        if cmd.section != last_section {
            section_count += 1;
            last_section = cmd.section;
        }
    }
    let content_h = 1 + 1 + 1 + 1 + items_height + section_count * 2 + 1;
    let modal_h = content_h.min(area.height.saturating_sub(4));
    let modal_x = area.x + (area.width.saturating_sub(modal_w)) / 2;
    let modal_y = area.y + (area.height.saturating_sub(modal_h)) / 2;
    let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);

    app.command_modal_rect = modal_rect;

    frame.render_widget(Clear, modal_rect);
    let pbg = t.palette_bg;
    let buf = frame.buffer_mut();
    for y in modal_rect.y..modal_rect.y + modal_rect.height {
        for x in modal_rect.x..modal_rect.x + modal_rect.width {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_style(Style::new().bg(pbg));
            }
        }
    }
    let inner = Rect::new(
        modal_rect.x + 2,
        modal_rect.y + 1,
        modal_rect.width.saturating_sub(4),
        modal_rect.height.saturating_sub(2),
    );
    if inner.is_empty() {
        return;
    }

    Line::from(vec![Span::styled(
        "Commands",
        Style::new().fg(t.text).bg(pbg).bold(),
    )])
    .render(Rect::new(inner.x, inner.y, inner.width, 1), buf);
    let esc_x = inner.x + inner.width.saturating_sub(3);
    Line::styled("esc", Style::new().fg(t.text_muted).bg(pbg))
        .render(Rect::new(esc_x, inner.y, 3, 1), buf);

    let search_y = inner.y + 2;
    if search_y < inner.y + inner.height {
        if app.command_search.is_empty() {
            Line::styled("Search", Style::new().fg(t.text_muted).bg(pbg))
                .render(Rect::new(inner.x, search_y, inner.width, 1), buf);
        }
        Line::from(vec![
            Span::styled(&app.command_search, Style::new().fg(t.text).bg(pbg)),
            Span::styled("\u{2588}", Style::new().fg(t.text).bg(pbg)),
        ])
        .render(Rect::new(inner.x, search_y, inner.width, 1), buf);
    }

    let list_start_y = search_y + 2;
    app.command_list_y = list_start_y;
    let pad = 1u16;
    let mut cy = list_start_y;
    let mut last_section = "";

    for (item_idx, cmd) in filtered.iter().enumerate() {
        if cy >= inner.y + inner.height {
            break;
        }
        if cmd.section != last_section {
            if !last_section.is_empty() {
                cy += 1;
                if cy >= inner.y + inner.height {
                    break;
                }
            }
            last_section = cmd.section;
            Line::styled(cmd.section, Style::new().fg(t.primary).bg(pbg).bold())
                .render(Rect::new(inner.x, cy, inner.width, 1), buf);
            cy += 1;
            if cy >= inner.y + inner.height {
                break;
            }
        }
        let is_selected = item_idx == app.command_selected;
        let row_bg = if is_selected {
            t.palette_selected
        } else {
            pbg
        };
        for x in (modal_rect.x + 1)..(modal_rect.x + modal_rect.width.saturating_sub(1)) {
            if let Some(cell) = buf.cell_mut((x, cy)) {
                cell.set_style(Style::new().bg(row_bg));
            }
        }
        let name = if cmd.name == "Show sidebar" && app.view.sidebar_visible {
            "Hide sidebar"
        } else {
            cmd.name
        };
        Line::styled(name, Style::new().fg(t.text).bg(row_bg)).render(
            Rect::new(inner.x + pad, cy, inner.width.saturating_sub(pad * 2), 1),
            buf,
        );
        let sw = cmd.shortcut.len() as u16;
        let sx = inner.x + inner.width.saturating_sub(sw + pad);
        Line::styled(
            cmd.shortcut,
            Style::new()
                .fg(if is_selected {
                    t.text
                } else {
                    t.text_muted
                })
                .bg(row_bg),
        )
        .render(Rect::new(sx, cy, sw, 1), buf);
        cy += 1;
    }
}

pub fn render_slash_menu(frame: &mut Frame, area: Rect, app: &App, t: &ColorTheme) {
    let cmds = filtered_slash_commands(&app.input);
    if cmds.is_empty() {
        return;
    }

    let total = cmds.len();
    let visible = (total as u16).min(SLASH_MAX_VISIBLE);
    if area.height < visible {
        return;
    }

    let scroll = if app.slash_selected >= total || total as u16 <= visible {
        0
    } else {
        let max_scroll = total - visible as usize;
        app.slash_selected
            .saturating_sub(visible as usize - 1)
            .min(max_scroll)
    };

    let menu_y = area.y + area.height - visible;
    let menu_rect = Rect::new(area.x, menu_y, area.width, visible);

    frame.render_widget(Clear, menu_rect);
    let bg_style = Style::new().bg(t.bg_element);
    let blank_lines: Vec<Line> = (0..visible)
        .map(|_| Line::styled(" ".repeat(menu_rect.width as usize), bg_style))
        .collect();
    frame.render_widget(Paragraph::new(blank_lines), menu_rect);

    let buf = frame.buffer_mut();
    let border_color = t.palette_selected;
    for y in menu_rect.y..menu_rect.y + menu_rect.height {
        if let Some(cell) = buf.cell_mut((menu_rect.x, y)) {
            cell.set_symbol("\u{2503}");
            cell.set_style(Style::new().fg(border_color).bg(t.bg_element));
        }
    }

    let name_col = 15u16;
    for vi in 0..visible as usize {
        let i = scroll + vi;
        if i >= total {
            break;
        }
        let cmd = cmds[i];
        let y = menu_rect.y + vi as u16;
        let is_selected = i == app.slash_selected;
        let row_bg = if is_selected {
            t.palette_selected
        } else {
            t.bg_element
        };
        if is_selected {
            for x in (menu_rect.x + 1)..menu_rect.x + menu_rect.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_style(Style::new().bg(row_bg));
                }
            }
        }
        Line::styled(cmd.name, Style::new().fg(t.text).bg(row_bg).bold())
            .render(Rect::new(menu_rect.x + 2, y, name_col, 1), buf);
        Line::styled(cmd.description, Style::new().fg(t.text_muted).bg(row_bg)).render(
            Rect::new(
                menu_rect.x + 2 + name_col,
                y,
                menu_rect.width.saturating_sub(2 + name_col),
                1,
            ),
            buf,
        );
    }
}

pub fn render_theme_modal(frame: &mut Frame, area: Rect, app: &mut App) {
    let t = app.theme;
    let themes = theme::all_themes();

    let buf = frame.buffer_mut();
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_style(Style::new().fg(t.text_muted).bg(t.bg));
            }
        }
    }

    let visible_rows = 15u16;
    let modal_w = 35u16.min(area.width.saturating_sub(4));
    let list_rows = (themes.len() as u16).min(visible_rows);
    let modal_h = (list_rows + 4).min(area.height.saturating_sub(4));
    let modal_x = area.x + (area.width.saturating_sub(modal_w)) / 2;
    let modal_y = area.y + (area.height.saturating_sub(modal_h)) / 2;
    let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);

    app.theme_modal_rect = modal_rect;

    frame.render_widget(Clear, modal_rect);
    let pbg = t.palette_bg;
    let buf = frame.buffer_mut();
    for y in modal_rect.y..modal_rect.y + modal_rect.height {
        for x in modal_rect.x..modal_rect.x + modal_rect.width {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_style(Style::new().bg(pbg));
            }
        }
    }

    let inner = Rect::new(
        modal_rect.x + 2,
        modal_rect.y + 1,
        modal_rect.width.saturating_sub(4),
        modal_rect.height.saturating_sub(2),
    );
    if inner.is_empty() {
        return;
    }

    Line::from(vec![Span::styled(
        "Themes",
        Style::new().fg(t.text).bg(pbg).bold(),
    )])
    .render(Rect::new(inner.x, inner.y, inner.width, 1), buf);
    let esc_x = inner.x + inner.width.saturating_sub(3);
    Line::styled("esc", Style::new().fg(t.text_muted).bg(pbg))
        .render(Rect::new(esc_x, inner.y, 3, 1), buf);

    let list_y = inner.y + 2;
    let scroll = app.theme_scroll;
    let current_name = app.loader_theme.name();
    let avail_rows = (inner.y + inner.height).saturating_sub(list_y) as usize;
    for vi in 0..avail_rows {
        let i = scroll + vi;
        if i >= themes.len() {
            break;
        }
        let y = list_y + vi as u16;
        let (loader_theme, _) = themes[i];
        let is_selected = i == app.theme_selected;
        let is_active = loader_theme.name() == current_name;
        let row_bg = if is_selected {
            t.palette_selected
        } else {
            pbg
        };
        if is_selected {
            for x in inner.x..inner.x + inner.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_style(Style::new().bg(row_bg));
                }
            }
        }
        let prefix = if is_active {
            "\u{25cf} "
        } else {
            "  "
        };
        Line::from(vec![
            Span::styled(prefix, Style::new().fg(t.primary).bg(row_bg)),
            Span::styled(loader_theme.name(), Style::new().fg(t.text).bg(row_bg)),
        ])
        .render(Rect::new(inner.x, y, inner.width, 1), buf);
    }
}
