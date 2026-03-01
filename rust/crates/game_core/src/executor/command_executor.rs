use platform::log_debug;
use platform::logger::LogType;

use crate::executor::ExecutorResult;
use crate::StateRequest;
use crate::{
    api::commands::{ExecutionPlayerCommand, PlayerCommand},
    map::LogicMap,
    CharacterLogic,
};
use std::collections::VecDeque;
use std::sync::Arc;

pub struct CommandExecutor {
    commands: VecDeque<ExecutionPlayerCommand>,
    current: Option<ExecutionPlayerCommand>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        CommandExecutor {
            commands: VecDeque::new(),
            current: None,
        }
    }

    /// Returns the source line number of the command currently being executed,
    /// or 0 if no command has been dispatched yet.
    pub fn current_line(&self) -> usize {
        self.current.map_or(0, |cmd| cmd.line)
    }

    pub fn tick(
        &mut self,
        _delta: f32,
        character: &mut CharacterLogic,
        logic_map: &Arc<LogicMap>,
    ) -> ExecutorResult {
        // Proceed to the next command only after the character executed the previous one and came
        // to Idle state
        if !character.is_idle() {
            return ExecutorResult::Running;
        }

        let Some(exec_cmd) = self.commands.front() else {
            return ExecutorResult::Empty;
        };

        let cmd = &exec_cmd.command;

        log_debug!(
            "[CommandExecutor]: Command: {:?} (line {}) from commands: {:?}",
            cmd,
            exec_cmd.line,
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
                self.current = Some(*exec_cmd);
                return ExecutorResult::Turn;
            }
        }

        let mut cell_position = character.get_cell_position();
        match cmd {
            PlayerCommand::MoveNorth => {
                // get character cell coordinates
                cell_position.y -= 1;
            }
            PlayerCommand::MoveEast => {
                cell_position.x += 1;
            }
            PlayerCommand::MoveWest => {
                cell_position.x -= 1;
            }
            PlayerCommand::MoveSouth => {
                cell_position.y += 1;
            }
            _ => todo!(),
        };
        let position = logic_map.get_screen_position(cell_position);
        if let Ok(_) = character.try_transition(StateRequest::WalkTo(position)) {
            self.current = self.commands.pop_front();
        }
        ExecutorResult::Running
    }

    pub fn set_commands(&mut self, commands: Vec<ExecutionPlayerCommand>) {
        self.commands.extend(commands);
    }

    pub fn get_current_command(&self) -> Option<ExecutionPlayerCommand> {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::worker::init_bt_system;
    use crate::api::commands::{ExecutionPlayerCommand, PlayerCommand};
    use crate::map::logic_map::{LogicCell, LogicMap};
    use platform::logger::{LogType, Logger};
    use platform::shared::logger_global::init_logger;
    use platform::types::{Direction, Vector2D, Vector2Di};
    use platform::Animator;

    static INIT: std::sync::Once = std::sync::Once::new();

    fn ensure_init() {
        INIT.call_once(|| {
            struct NullLogger;
            impl Logger for NullLogger {
                fn log(&self, _: LogType, _: &str) {}
            }
            init_logger(Box::new(NullLogger));
            init_bt_system();
        });
    }

    /// A test animator that resets `playing` to false on each `process()` call,
    /// so TurnState can detect animation completion and transition to Idle.
    struct TestAnimator {
        position: Vector2D,
        animation: String,
        playing: bool,
    }

    impl TestAnimator {
        fn new(position: Vector2D) -> Self {
            Self {
                position,
                animation: String::new(),
                playing: false,
            }
        }
    }

    impl Animator for TestAnimator {
        fn play(&mut self, name: &str) {
            self.animation = name.to_string();
            self.playing = true;
        }
        fn is_playing(&self) -> bool {
            self.playing
        }
        fn process(&mut self, _delta: f32) {
            // Simulate animation completing each frame so TurnState can exit
            self.playing = false;
        }
        fn set_position(&mut self, position: Vector2D) {
            self.position = position;
        }
        fn get_position(&self) -> Vector2D {
            self.position
        }
        fn get_global_position(&self) -> Vector2D {
            self.position
        }
    }

    /// Build a 3×3 map:
    /// Row j=0: non-walkable (top row)
    /// Row j=1: walkable
    /// Row j=2: walkable (bottom row where character starts)
    fn make_3x3_map() -> Arc<LogicMap> {
        let mut map = LogicMap::new(3, 3);
        for i in 0..3usize {
            // Row 0: non-walkable
            map.set_cell(
                i,
                0,
                Some(LogicCell {
                    walkable: false,
                    height: 0,
                    step_type: crate::map::StepType::None,
                }),
            );
            // Rows 1 and 2: walkable
            map.set_cell(
                i,
                1,
                Some(LogicCell {
                    walkable: true,
                    height: 0,
                    step_type: crate::map::StepType::None,
                }),
            );
            map.set_cell(
                i,
                2,
                Some(LogicCell {
                    walkable: true,
                    height: 0,
                    step_type: crate::map::StepType::None,
                }),
            );
        }
        Arc::new(map)
    }

    fn make_character(cell_x: i32, cell_y: i32) -> CharacterLogic {
        ensure_init();
        let mut ch = CharacterLogic::new(
            (cell_x as u32) * 1000 + cell_y as u32,
            Box::new(TestAnimator::new(Vector2D::new(0.0, 0.0))),
        );
        // set_cell_position sets both the screen position and prev_cell
        ch.set_cell_position(cell_x, cell_y);
        ch
    }

    /// Advance simulation: alternate executor ticks and character process ticks
    /// until the character is idle and the executor command queue is empty,
    /// or until the tick budget is exhausted (to prevent infinite loops in tests).
    fn run_until_idle_or_budget(
        executor: &mut CommandExecutor,
        character: &mut CharacterLogic,
        map: &Arc<LogicMap>,
        max_ticks: usize,
    ) {
        for _ in 0..max_ticks {
            executor.tick(0.016, character, map);
            character.process(0.016, map);
            if character.is_idle() && executor.commands.is_empty() {
                break;
            }
        }
    }

    /// Test: character at (1,2) on a 3×3 map where row 0 is non-walkable.
    /// Commands: MoveNorth, MoveNorth, MoveNorth, MoveEast.
    ///
    /// Expected sequence:
    /// 1. First MoveNorth: (1,2) → (1,1)  — succeeds
    /// 2. Second MoveNorth: tries (1,0)    — non-walkable, bounces back to (1,1), Idle
    /// 3. Third MoveNorth: tries (1,0)     — non-walkable, bounces back to (1,1), Idle
    /// 4. MoveEast: (1,1) → (2,1)         — succeeds
    ///
    /// Final position: cell (2,1)
    #[test]
    fn test_blocked_move_north_then_move_east() {
        let map = make_3x3_map();
        let mut character = make_character(1, 2);

        // Start character facing NORTH so no initial turn is needed for MoveNorth
        character.direction = Direction::NORTH;
        // Put character into Idle state so the executor will start processing
        character.try_transition(StateRequest::Idle).unwrap();

        let mut executor = CommandExecutor::new();
        executor.set_commands(vec![
            ExecutionPlayerCommand {
                command: PlayerCommand::MoveNorth,
                line: 1,
            },
            ExecutionPlayerCommand {
                command: PlayerCommand::MoveNorth,
                line: 2,
            },
            ExecutionPlayerCommand {
                command: PlayerCommand::MoveNorth,
                line: 3,
            },
            ExecutionPlayerCommand {
                command: PlayerCommand::MoveEast,
                line: 4,
            },
        ]);

        // Run with a generous tick budget (4 commands × ~80 ticks each for movement)
        run_until_idle_or_budget(&mut executor, &mut character, &map, 1000);

        let final_cell = character.get_cell_position();
        assert_eq!(
            final_cell,
            Vector2Di::new(2, 1),
            "Character should end up at cell (2,1) after blocked north moves and successful east move, got ({},{})",
            final_cell.x,
            final_cell.y,
        );
    }
}
