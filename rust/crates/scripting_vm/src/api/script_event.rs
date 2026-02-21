use game_core::api::commands::PlayerCommand;

#[derive(Debug, Clone)]
pub enum ScriptEvent {
    Line(usize),
    Command(PlayerCommand),
}
