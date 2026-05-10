use {
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind},
    ratatui::layout::Rect,
    ratatui_opentui_loader::{KittLoader, Theme as LoaderTheme},
};

use crate::{
    commands::{self, filtered_slash_commands},
    messages::{self, MessageBlock},
    theme::{self, ColorTheme},
};

pub const INPUT_HEIGHT: u16 = 5;

#[derive(Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Normal,
    SlashMenu,
    CommandPalette,
    ThemePicker,
}

pub struct ViewOptions {
    pub sidebar_visible: bool,
    pub scrollbar_visible: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentMode {
    #[default]
    Build,
    Plan,
}

impl AgentMode {
    const fn next(self) -> Self {
        match self {
            Self::Build => Self::Plan,
            Self::Plan => Self::Build,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Build => "Build",
            Self::Plan => "Plan",
        }
    }

    pub const fn accent(self, theme: &ColorTheme) -> ratatui::style::Color {
        match self {
            Self::Build => theme.primary,
            Self::Plan => theme.palette_selected,
        }
    }
}

enum KeyChord {
    CtrlX,
}

pub struct App {
    pub messages: Vec<MessageBlock>,
    pub scroll_offset: u16,
    pub loader: KittLoader,
    pub mouse_pos: (u16, u16),
    pub tool_block_rects: Vec<(usize, Rect)>,
    pub input: String,
    pub input_history: Vec<String>,
    pub history_index: Option<usize>,
    pub mode: Mode,
    pub command_selected: usize,
    pub command_search: String,
    pub command_modal_rect: Rect,
    pub command_list_y: u16,
    pub blink_on: bool,
    pub tick_count: u32,
    pub theme: ColorTheme,
    pub loader_theme: LoaderTheme,
    pub slash_selected: usize,
    pub theme_selected: usize,
    pub theme_saved: usize,
    pub theme_scroll: usize,
    pub theme_modal_rect: Rect,
    pub loader_visible_ticks: u32,
    pub view: ViewOptions,
    pub agent_mode: AgentMode,
    pending_chord: Option<KeyChord>,
}

impl App {
    pub fn new() -> Self {
        Self {
            messages: messages::demo_messages(),
            scroll_offset: u16::MAX,
            loader: KittLoader::new(),
            mouse_pos: (0, 0),
            tool_block_rects: Vec::new(),
            input: String::new(),
            input_history: Vec::new(),
            history_index: None,
            mode: Mode::Normal,
            command_selected: 0,
            command_search: String::new(),
            command_modal_rect: Rect::ZERO,
            command_list_y: 0,
            blink_on: true,
            tick_count: 0,
            theme: theme::OPENCODE,
            loader_theme: LoaderTheme::Opencode,
            slash_selected: 0,
            theme_selected: 0,
            theme_saved: 0,
            theme_scroll: 0,
            theme_modal_rect: Rect::ZERO,
            loader_visible_ticks: 0,
            view: ViewOptions {
                sidebar_visible: true,
                scrollbar_visible: true,
            },
            agent_mode: AgentMode::default(),
            pending_chord: None,
        }
    }

    pub fn filtered_commands(&self) -> Vec<&'static commands::Command> {
        commands::filtered_commands(&self.command_search)
    }

    pub fn submit_input(&mut self) {
        let text = self.input.trim().to_string();
        if text.is_empty() {
            return;
        }
        self.input.clear();
        if text == "/themes" {
            let current = theme::all_themes()
                .iter()
                .position(|(lt, _)| lt.name() == self.loader_theme.name())
                .unwrap_or(0);
            self.mode = Mode::ThemePicker;
            self.theme_selected = current;
            self.theme_saved = current;
            return;
        }
        self.input_history.push(text.clone());
        self.history_index = None;
        self.messages.push(MessageBlock::UserMessage(text));
        self.scroll_offset = u16::MAX;
        self.loader_visible_ticks = 200; // ~8s at 40ms tick rate
    }

    pub fn apply_theme(&mut self, idx: usize) {
        if let Some(&(loader_theme, color_theme)) = theme::all_themes().get(idx) {
            self.theme = color_theme;
            self.loader_theme = loader_theme;
            self.loader.set_theme(loader_theme);
        }
    }

    pub fn history_up(&mut self) {
        if self.input_history.is_empty() {
            return;
        }
        let idx = match self.history_index {
            None => self.input_history.len() - 1,
            Some(0) => 0,
            Some(i) => i - 1,
        };
        self.history_index = Some(idx);
        self.input = self.input_history[idx].clone();
    }

    pub fn history_down(&mut self) {
        match self.history_index {
            None => {},
            Some(i) if i + 1 >= self.input_history.len() => {
                self.history_index = None;
                self.input.clear();
            },
            Some(i) => {
                self.history_index = Some(i + 1);
                self.input = self.input_history[i + 1].clone();
            },
        }
    }

    /// Detect the effective mode (Normal auto-transitions to `SlashMenu` when input starts with /).
    pub fn effective_mode(&self) -> &Mode {
        if self.mode == Mode::Normal && self.input.starts_with('/') {
            &Mode::SlashMenu
        } else {
            &self.mode
        }
    }
}

/// Process one event. Returns true if the app should exit.
#[allow(clippy::needless_pass_by_value)]
pub fn handle_event(app: &mut App, ev: Event) -> bool {
    match ev {
        Event::Key(key) => {
            if key.kind != KeyEventKind::Press {
                return false;
            }
            handle_key(app, key.code, key.modifiers)
        },
        Event::Mouse(mouse) => {
            app.mouse_pos = (mouse.column, mouse.row);
            handle_mouse(app, mouse);
            false
        },
        _ => false,
    }
}

fn handle_key(app: &mut App, code: KeyCode, modifiers: KeyModifiers) -> bool {
    match app.effective_mode() {
        Mode::ThemePicker => handle_key_theme(app, code),
        Mode::CommandPalette => handle_key_command(app, code),
        Mode::SlashMenu => handle_key_slash(app, code, modifiers),
        Mode::Normal => handle_key_normal(app, code, modifiers),
    }
}

const THEME_VISIBLE_ROWS: usize = 15;

fn handle_key_theme(app: &mut App, code: KeyCode) -> bool {
    let count = theme::all_themes().len();
    match code {
        KeyCode::Esc => {
            app.apply_theme(app.theme_saved);
            app.mode = Mode::Normal;
        },
        KeyCode::Up => {
            app.theme_selected = app.theme_selected.saturating_sub(1);
            if app.theme_selected < app.theme_scroll {
                app.theme_scroll = app.theme_selected;
            }
            app.apply_theme(app.theme_selected);
        },
        KeyCode::Down => {
            app.theme_selected = (app.theme_selected + 1).min(count.saturating_sub(1));
            if app.theme_selected >= app.theme_scroll + THEME_VISIBLE_ROWS {
                app.theme_scroll = app.theme_selected + 1 - THEME_VISIBLE_ROWS;
            }
            app.apply_theme(app.theme_selected);
        },
        KeyCode::Enter => {
            app.apply_theme(app.theme_selected);
            app.theme_saved = app.theme_selected;
            app.mode = Mode::Normal;
        },
        _ => {},
    }
    false
}

fn handle_key_command(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Up => {
            app.command_selected = app.command_selected.saturating_sub(1);
        },
        KeyCode::Down => {
            let max = app.filtered_commands().len().saturating_sub(1);
            app.command_selected = (app.command_selected + 1).min(max);
        },
        KeyCode::Esc | KeyCode::Enter => {
            app.mode = Mode::Normal;
            app.command_search.clear();
            app.command_selected = 0;
        },
        KeyCode::Backspace => {
            app.command_search.pop();
            app.command_selected = 0;
        },
        KeyCode::Char(c) => {
            app.command_search.push(c);
            app.command_selected = 0;
        },
        _ => {},
    }
    false
}

fn handle_key_slash(app: &mut App, code: KeyCode, modifiers: KeyModifiers) -> bool {
    let cmds = filtered_slash_commands(&app.input);
    match code {
        KeyCode::Esc => {
            app.input.clear();
            app.slash_selected = 0;
        },
        KeyCode::Up => {
            app.slash_selected = app.slash_selected.saturating_sub(1);
        },
        KeyCode::Down => {
            let max = cmds.len().saturating_sub(1);
            app.slash_selected = (app.slash_selected + 1).min(max);
        },
        KeyCode::Enter => {
            if let Some(cmd) = cmds.get(app.slash_selected) {
                app.input = cmd.name.to_string();
            }
            app.slash_selected = 0;
            app.submit_input();
        },
        KeyCode::Backspace => {
            app.input.pop();
            app.slash_selected = 0;
        },
        KeyCode::Char('c' | 'd') if modifiers.contains(KeyModifiers::CONTROL) => return true,
        KeyCode::Char(c) => {
            app.input.push(c);
            app.slash_selected = 0;
        },
        _ => {},
    }
    false
}

fn handle_key_normal(app: &mut App, code: KeyCode, modifiers: KeyModifiers) -> bool {
    if matches!(app.pending_chord.take(), Some(KeyChord::CtrlX)) {
        match code {
            KeyCode::Char('b') => app.view.sidebar_visible = !app.view.sidebar_visible,
            KeyCode::Char('s') => app.view.scrollbar_visible = !app.view.scrollbar_visible,
            _ => {},
        }
        return false;
    }

    match code {
        KeyCode::Esc => return true,
        KeyCode::Char('c' | 'd') if modifiers.contains(KeyModifiers::CONTROL) => return true,
        KeyCode::Char('x') if modifiers.contains(KeyModifiers::CONTROL) => {
            app.pending_chord = Some(KeyChord::CtrlX);
        },
        KeyCode::Char('p') if modifiers.contains(KeyModifiers::CONTROL) => {
            app.mode = Mode::CommandPalette;
            app.command_search.clear();
            app.command_selected = 0;
        },
        KeyCode::Tab | KeyCode::BackTab => {
            app.agent_mode = app.agent_mode.next();
        },
        KeyCode::Enter | KeyCode::Char('j' | 'J') if modifiers.contains(KeyModifiers::SHIFT) => {
            app.input.push('\n');
        },
        KeyCode::Enter => app.submit_input(),
        KeyCode::Backspace => {
            app.input.pop();
        },
        KeyCode::Up => app.history_up(),
        KeyCode::Down => app.history_down(),
        KeyCode::PageUp => {
            app.scroll_offset = app.scroll_offset.saturating_sub(10);
        },
        KeyCode::PageDown => {
            app.scroll_offset = app.scroll_offset.saturating_add(10);
        },
        KeyCode::Char(c) => {
            app.history_index = None;
            app.input.push(c);
        },
        _ => {},
    }
    false
}

fn handle_mouse(app: &mut App, mouse: event::MouseEvent) {
    match app.effective_mode() {
        Mode::ThemePicker => handle_mouse_theme(app, mouse),
        Mode::CommandPalette => handle_mouse_command(app, mouse),
        Mode::SlashMenu => handle_mouse_slash(app, mouse),
        Mode::Normal => handle_mouse_normal(app, mouse),
    }
}

fn handle_mouse_theme(app: &mut App, mouse: event::MouseEvent) {
    let r = app.theme_modal_rect;
    if r.height == 0 {
        return;
    }
    let list_y = r.y + 3;
    if !r.contains((mouse.column, mouse.row).into()) || mouse.row < list_y {
        return;
    }
    let row = (mouse.row - list_y) as usize + app.theme_scroll;
    let count = theme::all_themes().len();
    if row >= count {
        return;
    }
    match mouse.kind {
        MouseEventKind::Moved => {
            app.theme_selected = row;
            app.apply_theme(row);
        },
        MouseEventKind::Up(event::MouseButton::Left) => {
            app.theme_selected = row;
            app.apply_theme(row);
            app.theme_saved = row;
            app.mode = Mode::Normal;
        },
        MouseEventKind::ScrollDown => {
            let max = count.saturating_sub(THEME_VISIBLE_ROWS);
            app.theme_scroll = (app.theme_scroll + 1).min(max);
        },
        MouseEventKind::ScrollUp => {
            app.theme_scroll = app.theme_scroll.saturating_sub(1);
        },
        _ => {},
    }
}

fn handle_mouse_command(app: &mut App, mouse: event::MouseEvent) {
    let r = app.command_modal_rect;
    if r.height == 0 {
        return;
    }
    let list_y = app.command_list_y;
    if !r.contains((mouse.column, mouse.row).into()) || mouse.row < list_y {
        return;
    }
    let filtered = app.filtered_commands();
    let mut cy = list_y;
    let mut last_section = "";
    let mut target_idx = None;
    for (i, cmd) in filtered.iter().enumerate() {
        if cmd.section != last_section {
            if !last_section.is_empty() {
                cy += 1;
            }
            last_section = cmd.section;
            cy += 1;
        }
        if cy == mouse.row {
            target_idx = Some(i);
            break;
        }
        cy += 1;
    }
    if let Some(idx) = target_idx {
        match mouse.kind {
            MouseEventKind::Moved => {
                app.command_selected = idx;
            },
            MouseEventKind::Up(event::MouseButton::Left) => {
                app.command_selected = idx;
                app.mode = Mode::Normal;
                app.command_search.clear();
            },
            _ => {},
        }
    }
}

fn handle_mouse_slash(app: &mut App, mouse: event::MouseEvent) {
    let cmds = filtered_slash_commands(&app.input);
    if cmds.is_empty() {
        return;
    }
    match mouse.kind {
        MouseEventKind::ScrollDown => {
            let max = cmds.len().saturating_sub(1);
            app.slash_selected = (app.slash_selected + 1).min(max);
        },
        MouseEventKind::ScrollUp => {
            app.slash_selected = app.slash_selected.saturating_sub(1);
        },
        _ => handle_mouse_normal(app, mouse),
    }
}

fn handle_mouse_normal(app: &mut App, mouse: event::MouseEvent) {
    match mouse.kind {
        MouseEventKind::ScrollDown => {
            app.scroll_offset = app.scroll_offset.saturating_add(1);
        },
        MouseEventKind::ScrollUp => {
            app.scroll_offset = app.scroll_offset.saturating_sub(1);
        },
        MouseEventKind::Up(event::MouseButton::Left) => {
            for &(msg_idx, rect) in &app.tool_block_rects {
                if rect.contains((mouse.column, mouse.row).into()) {
                    if let MessageBlock::ToolOutput { collapsed, .. } = &mut app.messages[msg_idx] {
                        *collapsed = !*collapsed;
                    }
                    break;
                }
            }
        },
        _ => {},
    }
}
