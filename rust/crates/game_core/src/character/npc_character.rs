use crate::ai::worker::*;
use crate::bt::command::BTCommand;
use crate::bt::job::BTJob;
use crate::bt::result::BTResult;
use crate::character::snapshot::CharacterSnapshot;
use crate::map::LogicMap;
use crate::StateRequest;
use platform::animator::Animator;
use platform::log_debug;
use platform::logger::LogType;
use platform::types::{Direction, Vector2D, Vector2Di};
use std::sync::Arc;

use crate::{
    bt::{BTRef, BehaviourTree},
    character::CharacterId,
    CharacterLogic,
};

pub struct NPCCharacterLogic {
    base: CharacterLogic,
    pub bt: BTRef,
    generation: u32,
}

impl NPCCharacterLogic {
    pub fn new(id: CharacterId, animator: Box<dyn Animator>) -> Self {
        NPCCharacterLogic {
            base: CharacterLogic::new(id, animator),
            bt: Arc::new(BehaviourTree::default()),
            generation: 0,
        }
    }

    pub fn process(&mut self, delta: f32, logic_map: &Arc<LogicMap>) {
        self.tick_ai(delta);
        self.base.process(delta, logic_map);
    }

    pub fn set_logic_map(&mut self, map: Arc<LogicMap>) {
        self.base.set_logic_map(map);
    }

    pub fn set_cell_position(&mut self, i: i32, j: i32) -> Vector2Di {
        self.base.set_cell_position(i, j)
    }

    pub fn get_position(&self) -> Vector2D {
        self.base.get_position()
    }

    pub fn get_id(&self) -> CharacterId {
        self.base.id
    }

    pub fn snapshot(&self) -> CharacterSnapshot {
        self.base.snapshot()
    }

    // check if the character is in the idle state
    pub fn is_idle(&self) -> bool {
        self.base.is_idle()
    }

    pub fn get_direction(&self) -> Direction {
        self.base.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.base.direction = direction;
    }

    /*
     * Thread-safe method to request a state change.
     * Can be called from Input, Behaviour Tree, or other threads
     */
    pub fn request_state(&self, request: StateRequest) {
        self.base.request_state(request);
    }

    pub fn try_transition(&mut self, req: StateRequest) -> Result<(), String> {
        self.base.try_transition(req)
    }

    pub fn get_cell_position(&self) -> Vector2Di {
        self.base.get_cell_position()
    }

    // Reset character to its initial state (position, FSM, BT blackboard)
    pub fn reset(&mut self) {
        self.base.reset();
        // Increment generation so any in-flight BT jobs from before this reset
        // are considered stale and their results will be discarded.
        self.generation = self.generation.wrapping_add(1);
    }

    pub fn set_start_cell(&mut self, cell: Vector2Di) {
        self.base.start_cell = cell;
    }

    pub fn snap_to_cell(&mut self) -> Vector2Di {
        self.base.snap_to_cell()
    }

    pub fn tick_ai(&mut self, delta: f32) {
        if let Some(tx) = JOB_TX.get() {
            let _ = tx.send(BTJob {
                character_id: self.base.id,
                snapshot: self.snapshot(),
                bt: self.bt.clone(),
                delta,
                generation: self.generation,
            });
        }

        if let Some(result) = take_result(self.base.id, self.generation) {
            self.process_commands(result);
        }
    }

    pub fn process_commands(&mut self, result: BTResult) {
        for cmd in result.commands {
            self.apply(cmd);
        }
    }

    // Process the command
    pub fn apply(&mut self, cmd: BTCommand) {
        log_debug!("received command: {:?}", cmd);
        use BTCommand::*;
        match cmd {
            ChangeState(state) => {
                self.request_state(state);
            }
            SetDirection(direction) => {
                self.base.direction = direction;
            }
            SnapToCell => {
                self.snap_to_cell();
            }

            _ => (),
        }
    }
}
