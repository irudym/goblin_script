use platform::log_debug;
use platform::logger::LogType;

use crate::StateRequest;
use crate::{api::commands::PlayerCommand, map::LogicMap, CharacterLogic};
use std::collections::VecDeque;
use std::sync::Arc;

pub struct CommandExecutor {
    commands: VecDeque<PlayerCommand>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        CommandExecutor {
            commands: VecDeque::new(),
        }
    }

    pub fn tick(&mut self, _delta: f32, character: &mut CharacterLogic, logic_map: &Arc<LogicMap>) {
        let Some(cmd) = self.commands.front() else {
            return;
        };

        log_debug!(
            "[CommandExecutor]: Current command: {:?} from commands: {:?}",
            cmd,
            self.commands
        );

        // get direction and compare with the current character direction
        if let Some(direction) = cmd.get_command_direction() {
            if direction != character.direction {
                // synchronously force WalkState→Idle before the Turn request is queued,
                // avoiding the "last-win" overwrite and ensuring the Turn can be accepted
                // from IdleState
                let _ = character.try_transition(StateRequest::Idle);
                character.request_state(StateRequest::Turn(direction));
                return;
            }
        }

        match cmd {
            PlayerCommand::MoveNorth => {
                // get character cell coordinates
                let mut cell_position = character.get_cell_position();
                cell_position.y -= 1;
                let position = logic_map.get_screen_position(cell_position);
                if let Ok(_) = character.try_transition(StateRequest::WalkTo(position)) {
                    self.commands.pop_front();
                }
            }
            PlayerCommand::MoveEast => {
                let mut cell_position = character.get_cell_position();
                cell_position.x += 1;
                let position = logic_map.get_screen_position(cell_position);
                if let Ok(_) = character.try_transition(StateRequest::WalkTo(position)) {
                    self.commands.pop_front();
                }
            }
            _ => todo!(),
        }
    }

    pub fn set_commands(&mut self, commands: Vec<PlayerCommand>) {
        self.commands.extend(commands);
    }
}
