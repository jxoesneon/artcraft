//! Artcraft commands.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "hub.new_project", label: "New Project…" },
    Command { id: "hub.open_project", label: "Open Project…" },
];
