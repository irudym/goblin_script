use crate::ai::worker::*;
use crate::bt::command::BTCommand;
use crate::bt::job::BTJob;
use crate::bt::result::BTResult;
use crate::bt::BTRef;
use crate::bt::BehaviourTree;
use crate::bt::Blackboard;
use crate::character::request::StateRequest;
use crate::character::snapshot::CharacterSnapshot;
use crate::character::CharacterId;
use crate::fsm::FSM;
use crate::fsm::{IdleState, RunState, TurnState, WalkState};
use crate::StateType;
use platform::logger::LogType;
use platform::types::Vector2Di;
use platform::types::{Direction, Vector2D};
use platform::Animator;
use std::sync::{Arc, Mutex};

use platform::log_debug;
use platform::shared::logger_global::log;

use crate::map::LogicMap;

pub struct CharacterLogic {
    pub direction: Direction,
    pub speed: f32,
    pub current_speed: f32,
    state: Option<Box<dyn FSM>>, // the active state machine, accessible only by Main Thread

    pending_request: Arc<Mutex<Option<StateRequest>>>, // the request buffer, thread safe

    animator: Box<dyn Animator>,

    cell_size: f32, //default value: 32px

    // pending_commands: Vec<BTCommand>,
    pub id: CharacterId,
    pub bt: BTRef,
    pub blackboard: Box<Blackboard>,

    current_cell: Vector2Di,
    prev_cell: Vector2Di,
}

impl CharacterLogic {
    pub fn new(id: CharacterId, animator: Box<dyn Animator>, cell_size: f32) -> Self {
        log(LogType::Info, "Create struct CharacterLogic");
        Self {
            id,
            direction: Direction::SOUTH,
            speed: 100.0,
            current_speed: 0.0,
            state: None,
            pending_request: Arc::new(Mutex::new(Some(StateRequest::Idle))),
            animator,
            cell_size,
            // pending_commands: Vec::new(),
            bt: Arc::new(BehaviourTree::default()),
            blackboard: Box::new(Blackboard::new()),

            current_cell: Vector2Di::new(0, 0),
            prev_cell: Vector2Di::new(0, 0),
        }
    }

    // TODO: need to get the cell size as a parameter
    pub fn snap_to_cell(&mut self) {
        // get cell i,j
        let position = self.get_position();
        let i = f32::round(position.x / self.cell_size) as i32;
        let j = f32::round(position.y / self.cell_size) as i32;

        self.set_position(Vector2D {
            x: (i as f32 * self.cell_size) as f32,
            y: (j as f32 * self.cell_size) as f32,
        });
    }

    pub fn get_position(&self) -> Vector2D {
        self.animator.get_position()
    }

    pub fn set_position(&mut self, position: Vector2D) {
        self.animator.set_position(position);
    }

    // Set character position in tile grid coordinates: I,J
    pub fn set_cell_position(&mut self, i: i32, j: i32) {
        self.set_position(Vector2D {
            x: (i as f32 * self.cell_size) as f32,
            y: (j as f32 * self.cell_size) as f32,
        });
    }

    pub fn set_cell_size(&mut self, size: f32) {
        self.cell_size = size;
    }

    // check if the character is in the idle state
    pub fn is_idle(&self) -> bool {
        if let Some(val) = self.state.as_ref() {
            val.get_type() == StateType::IDLE
        } else {
            false
        }
    }

    pub fn set_current_speed(&mut self, speed: f32) {
        self.current_speed = speed;
    }

    /*
     * Set and play animation by construction animation name using Character direction:
     * for example, animation is run, the direction is WEST, than animation name will be run_west
     */
    pub fn play_animation_with_direction(&mut self, animation_name: &str) {
        /*
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        */
        let animation = format!("{}_{}", animation_name, self.direction);
        self.animator.play(&animation);
    }

    pub fn play_animation(&mut self, animation_name: &str) {
        self.animator.play(animation_name);
    }

    pub fn is_animation_playing(&self) -> bool {
        self.animator.is_playing()
    }

    /*
     * Thread-safe method to request a state change.
     * Can be called from Input, Behaviour Tree, or other threads
     */
    pub fn request_state(&self, request: StateRequest) {
        log_debug!(
            "Character::request_state: {:?}, current direction: {}",
            request,
            self.direction
        );
        if let Ok(mut pending) = self.pending_request.lock() {
            // "last win strategy" - if multiple systems request a state in the same frame, the last one overrides
            *pending = Some(request);
        }
    }

    /*
     * Internal method to process the queue
     * This acts like a factory that converts enum to structs
     */
    fn handle_transitions(&mut self) {
        //1. Check if there is a request
        let request_opt = {
            let mut lock = self.pending_request.lock().unwrap();
            lock.take()
        };

        if let Some(req) = request_opt {
            self.try_transition(req);
        }
    }

    /*
     * Transition logic with validation
     */
    fn try_transition(&mut self, req: StateRequest) {
        let can_exit = if let Some(val) = self.state.as_ref() {
            val.can_exit()
        } else {
            true
        };

        log(
            LogType::Debug,
            &format!("check if can exit from the current state: {}", can_exit),
        );

        //1. Map request -> Target state type
        let target_type = match req {
            StateRequest::Idle => StateType::IDLE,
            StateRequest::Run => StateType::RUN,
            StateRequest::Turn(_) => StateType::TURN,
            StateRequest::WalkTo(_) => StateType::RUN,
        };

        //2. validate transition rules
        if !can_exit {
            // the state is locked
            return;
        }

        let new_state: Box<dyn FSM> = match req {
            StateRequest::Idle => Box::new(IdleState::new()),
            StateRequest::Run => Box::new(RunState::new()),
            StateRequest::Turn(direction) => Box::new(TurnState::new(direction)),
            StateRequest::WalkTo(target) => Box::new(WalkState::new(target)),
        };

        // perform the swap
        if let Some(old_state) = self.state.take() {
            if !old_state.can_transition_to(target_type) {
                self.state = Some(old_state);
                return;
            }
            old_state.exit(self);
        }
        log_debug!("Enter to new state: {:?}", &new_state.get_type());
        let mut next_state = new_state;
        next_state.enter(self);
        self.state = Some(next_state);
    }

    pub fn snapshot(&self) -> CharacterSnapshot {
        CharacterSnapshot {
            id: self.id,
            position: self.animator.get_position(),
            direction: self.direction.clone(),
            velocity: self.current_speed * self.direction.to_vector(),
            is_idle: self.is_idle(),
            blackboard: self.blackboard.clone(),
            current_speed: self.current_speed,
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
                self.direction = direction;
            }
            SnapToCell => {
                self.snap_to_cell();
            }

            _ => (),
        }
    }

    pub fn process_commands(&mut self, result: BTResult) {
        for cmd in result.commands {
            self.apply(cmd);
        }
    }

    pub fn tick_ai(&mut self) {
        if let Some(tx) = JOB_TX.get() {
            let _ = tx.send(BTJob {
                character_id: self.id,
                snapshot: self.snapshot(),
                bt: self.bt.clone(),
            });
        }

        if let Some(result) = take_result(self.id) {
            self.process_commands(result);
        }
    }

    pub fn process(&mut self, delta: f32, logic_map: &Arc<LogicMap>) {
        let state_type = if let Some(state) = &self.state {
            Some(state.get_type())
        } else {
            None
        };
        log_debug!(
            "Character[{}]::process\nDirection: {}\ncurrent_state: {:?}\ncurrent_pos: {:?}",
            self.id,
            self.direction,
            state_type,
            self.get_position()
        );

        self.tick_ai();

        // Process pending request
        self.handle_transitions();

        if self.current_speed > 0.0 {
            self.prev_cell = self.current_cell;

            //update character position
            let new_position =
                self.get_position() + self.direction.to_vector() * self.current_speed * delta;
            self.set_position(new_position);

            //
            //let new_cell = world_to_cell(new_position);
            //self.current_cell = new_cell;
            //
        }

        let pos = self.get_position();

        let cell_x = (pos.x / 64.0) as i32;
        let cell_y = (pos.y / 64.0) as i32;

        if !logic_map.is_walkable(cell_x, cell_y) {
            log_debug!("Blocked cell: {}, {}", cell_x, cell_y);
        }

        // Update the current state
        if let Some(mut state) = self.state.take() {
            state.update(delta, self);
            self.state = Some(state);
        }

        // Update animation
        self.animator.process(delta);
    }
}
