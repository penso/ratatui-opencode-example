pub struct Command {
    pub name: &'static str,
    pub shortcut: &'static str,
    pub section: &'static str,
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "Share session",
        shortcut: "session_share",
        section: "Suggested",
    },
    Command {
        name: "Switch session",
        shortcut: "ctrl+x l",
        section: "Session",
    },
    Command {
        name: "New session",
        shortcut: "ctrl+x n",
        section: "Session",
    },
    Command {
        name: "Switch model",
        shortcut: "ctrl+x m",
        section: "Session",
    },
    Command {
        name: "Open editor",
        shortcut: "ctrl+x e",
        section: "Session",
    },
    Command {
        name: "Rename session",
        shortcut: "ctrl+r",
        section: "Session",
    },
    Command {
        name: "Jump to message",
        shortcut: "ctrl+x g",
        section: "Session",
    },
    Command {
        name: "Fork session",
        shortcut: "session_fork",
        section: "Session",
    },
    Command {
        name: "Compact session",
        shortcut: "ctrl+x c",
        section: "Session",
    },
    Command {
        name: "Undo previous message",
        shortcut: "ctrl+x u",
        section: "Session",
    },
    Command {
        name: "Show sidebar",
        shortcut: "ctrl+x b",
        section: "Session",
    },
    Command {
        name: "Disable code concealment",
        shortcut: "ctrl+x h",
        section: "Session",
    },
];

pub struct SlashCommand {
    pub name: &'static str,
    pub description: &'static str,
}

pub const SLASH_COMMANDS: &[SlashCommand] = &[
    SlashCommand {
        name: "/agents",
        description: "Switch agent",
    },
    SlashCommand {
        name: "/compact",
        description: "Compact session",
    },
    SlashCommand {
        name: "/connect",
        description: "Connect provider",
    },
    SlashCommand {
        name: "/copy",
        description: "Copy session transcript",
    },
    SlashCommand {
        name: "/editor",
        description: "Open editor",
    },
    SlashCommand {
        name: "/exit",
        description: "Exit the app",
    },
    SlashCommand {
        name: "/export",
        description: "Export session transcript",
    },
    SlashCommand {
        name: "/fork",
        description: "Fork session",
    },
    SlashCommand {
        name: "/help",
        description: "Help",
    },
    SlashCommand {
        name: "/init",
        description: "guided AGENTS.md setup",
    },
    SlashCommand {
        name: "/model",
        description: "Switch model",
    },
    SlashCommand {
        name: "/plan",
        description: "Toggle plan mode",
    },
    SlashCommand {
        name: "/redo",
        description: "Redo reverted message",
    },
    SlashCommand {
        name: "/rename",
        description: "Rename session",
    },
    SlashCommand {
        name: "/share",
        description: "Share session",
    },
    SlashCommand {
        name: "/themes",
        description: "Switch color theme",
    },
    SlashCommand {
        name: "/thinking",
        description: "Toggle thinking display",
    },
    SlashCommand {
        name: "/timeline",
        description: "Jump to message",
    },
    SlashCommand {
        name: "/timestamps",
        description: "Toggle timestamps",
    },
    SlashCommand {
        name: "/undo",
        description: "Undo previous message",
    },
    SlashCommand {
        name: "/unshare",
        description: "Unshare session",
    },
    SlashCommand {
        name: "/variants",
        description: "Switch model variant",
    },
];

pub const SLASH_MAX_VISIBLE: u16 = 10;

pub fn filtered_slash_commands(input: &str) -> Vec<&'static SlashCommand> {
    let q = input.to_lowercase();
    SLASH_COMMANDS
        .iter()
        .filter(|c| c.name.to_lowercase().starts_with(&q) || q == "/")
        .collect()
}

pub fn filtered_commands(search: &str) -> Vec<&'static Command> {
    if search.is_empty() {
        COMMANDS.iter().collect()
    } else {
        let q = search.to_lowercase();
        COMMANDS
            .iter()
            .filter(|c| c.name.to_lowercase().contains(&q))
            .collect()
    }
}
