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

use crate::map::{LogicMap, StepType};

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

    prev_cell: Vector2Di,
}

impl CharacterLogic {
    pub fn new(id: CharacterId, animator: Box<dyn Animator>) -> Self {
        log(LogType::Info, "Create struct CharacterLogic");
        Self {
            id,
            direction: Direction::SOUTH,
            speed: 100.0,
            current_speed: 0.0,
            state: None,
            pending_request: Arc::new(Mutex::new(Some(StateRequest::Idle))),
            animator,
            cell_size: 64.0,
            // pending_commands: Vec::new(),
            bt: Arc::new(BehaviourTree::default()),
            blackboard: Box::new(Blackboard::new()),

            prev_cell: Vector2Di::new(0, 0),
        }
    }

    // TODO: need to get the cell size as a parameter
    pub fn snap_to_cell(&mut self) {
        // get cell i,j
        let coord = self.get_cell_position();
        self.set_cell_position(coord.x, coord.y);
    }

    pub fn get_position(&self) -> Vector2D {
        self.animator.get_position()
    }

    pub fn set_position(&mut self, position: Vector2D) {
        self.animator.set_position(position);
    }

    // Set character position in tile grid coordinates: I,J, reset the prev_cell as well
    pub fn set_cell_position(&mut self, i: i32, j: i32) -> Vector2Di {
        self.set_position(Vector2D {
            x: i as f32 * self.cell_size + self.cell_size / 2.0,
            y: j as f32 * self.cell_size + self.cell_size / 2.0,
        });
        let position = Vector2Di { x: i, y: j };
        self.prev_cell = position;
        position
    }

    //get character cell position in tile grid coordinates: I,J
    // DEPRECATED: use logic_map.get_cell_position(position: Vector2D)
    pub fn get_cell_position(&self) -> Vector2Di {
        let position = self.animator.get_global_position();
        let i = (position.x / self.cell_size) as i32;
        let j = (position.y / self.cell_size) as i32;
        Vector2Di { x: i, y: j }
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
            "Character[{}]: Character::request_state: {:?}, current direction: {}",
            self.id,
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
            let _ = self.try_transition(req);
        }
    }

    /*
     * Transition logic with validation
     */
    pub fn try_transition(&mut self, req: StateRequest) -> Result<(), String> {
        log_debug!("Character[{}]: try_transition to {:?}", self.id, req);
        let can_exit = if let Some(val) = self.state.as_ref() {
            val.can_exit()
        } else {
            true
        };

        log_debug!(
            "Character[{}]: check if can exit from the current state: {}",
            self.id,
            can_exit
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
            return Err("Cannot exit from the current state".to_string());
        }

        let new_state: Box<dyn FSM> = match req {
            StateRequest::Idle => Box::new(IdleState::new()),
            StateRequest::Run => Box::new(RunState::new()),
            StateRequest::Turn(direction) => Box::new(TurnState::new(direction)),
            StateRequest::WalkTo(target) => Box::new(WalkState::new(target)),
        };

        // perform the swap
        if let Some(old_state) = self.state.take() {
            let state_type = &old_state.get_type();
            if !old_state.can_transition_to(target_type) {
                self.state = Some(old_state);
                return Err(format!(
                    "Cannot make transition from {:?} to {:?}",
                    state_type, target_type
                ));
            }
            old_state.exit(self);
        }
        log_debug!(
            "Character[{}]: Entered to new state: {:?}",
            self.id,
            &new_state.get_type()
        );

        let mut next_state = new_state;
        next_state.enter(self);
        log_debug!(
            "Character[{}]: current state {:?}",
            self.id,
            &next_state.get_type()
        );
        self.state = Some(next_state);

        Ok(())
    }

    // Set the state without validations
    // Can be used to switch character to Idle state
    pub fn force_transition(&mut self, req: StateRequest) {
        let mut new_state: Box<dyn FSM> = match req {
            StateRequest::Idle => Box::new(IdleState::new()),
            StateRequest::Run => Box::new(RunState::new()),
            StateRequest::Turn(direction) => Box::new(TurnState::new(direction)),
            StateRequest::WalkTo(target) => Box::new(WalkState::new(target)),
        };

        // perform the swap
        if let Some(old_state) = self.state.take() {
            old_state.exit(self);
        }

        new_state.enter(self);
        self.state = Some(new_state);
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

    pub fn tick_ai(&mut self, delta: f32) {
        if let Some(tx) = JOB_TX.get() {
            let _ = tx.send(BTJob {
                character_id: self.id,
                snapshot: self.snapshot(),
                bt: self.bt.clone(),
                delta,
            });
        }

        if let Some(result) = take_result(self.id) {
            self.process_commands(result);
        }
    }

    fn get_effective_velocity(&self, logic_map: &LogicMap) -> Vector2D {
        let base = self.direction.to_vector() * self.current_speed;

        if self.current_speed == 0.0 {
            return base;
        }

        let cell = logic_map.get_cell_position(self.get_position());

        let step_type = logic_map.get_step_type(cell);

        // On step tiles: east/west movement becomes diagonal
        // Using (speed, -speed) preserves horizontal speed and matches
        // the original offset formula where ΔY = ΔX
        match (step_type, &self.direction) {
            // Left steps: "/" slope — rises eastward
            (StepType::Left, Direction::EAST) => {
                Vector2D::new(self.current_speed, -self.current_speed)
            }
            (StepType::Left, Direction::WEST) => {
                Vector2D::new(-self.current_speed, self.current_speed)
            }
            // Right steps: "\" slope — falls eastward
            (StepType::Right, Direction::EAST) => {
                Vector2D::new(self.current_speed, self.current_speed)
            }
            (StepType::Right, Direction::WEST) => {
                Vector2D::new(-self.current_speed, -self.current_speed)
            }
            // No step or NORTH/SOUTH on steps: normal movement
            _ => base,
        }
    }

    pub fn process(&mut self, delta: f32, logic_map: &Arc<LogicMap>) {
        let state_type = if let Some(state) = &self.state {
            Some(state.get_type())
        } else {
            None
        };

        log_debug!(
            "Character[{}]::process\nDirection: {}\ncurrent_state: {:?}\ncurrent_pos: {:?}\ncurrent_cell: {:?}",
            self.id,
            self.direction,
            state_type,
            self.get_position(),
            self.get_cell_position(),
        );

        self.tick_ai(delta);

        // Process pending request
        self.handle_transitions();

        if self.current_speed > 0.0 {
            let velocity = self.get_effective_velocity(logic_map);
            let new_position = self.get_position() + velocity * delta;
            self.set_position(new_position);
        }

        let mut pos = logic_map.get_cell_position(self.get_position());

        log_debug!(
            "Character[{}]: LogicMap => cell({},{}) -move--> cell({},{}): is_walkable_from: {}",
            self.id,
            self.prev_cell.x,
            self.prev_cell.y,
            pos.x,
            pos.y,
            logic_map.is_walkable_from(self.prev_cell, pos)
        );

        if !logic_map.is_walkable_from(self.prev_cell, pos) {
            // the character got to non walkable cell, set the position to the previous cell
            // and set Idle state
            log_debug!(
                "Character[{}]: got to non-walkable cell, move to the prev cell: ({}, {})",
                self.id,
                self.prev_cell.x,
                self.prev_cell.y
            );
            pos = self.set_cell_position(self.prev_cell.x, self.prev_cell.y);
            // transfer to idle
            let _ = self.force_transition(StateRequest::Idle);
        }

        // Update the current state
        if let Some(mut state) = self.state.take() {
            state.update(delta, self);
            self.state = Some(state);
        }

        // Update animation
        self.animator.process(delta);

        self.prev_cell = pos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::worker::init_bt_system;
    use crate::map::logic_map::{LogicCell, LogicMap};
    use platform::logger::{LogType, Logger};
    use platform::shared::logger_global::init_logger;

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
        fn process(&mut self, _delta: f32) {}
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

    /// 6×1 map for horizontal step traversal:
    /// Col:  0        1        2        3        4        5
    ///       flat,h=0 flat,h=0 step,h=0 step,h=1 flat,h=1 flat,h=1
    fn make_test_map() -> Arc<LogicMap> {
        let mut map = LogicMap::new(6, 1);
        for i in 0..6 {
            let (height, step_type) = match i {
                2 => (0, StepType::Left),
                3 => (1, StepType::Left),
                4 | 5 => (1, StepType::None),
                _ => (0, StepType::None),
            };
            map.set_cell(
                i,
                0,
                Some(LogicCell {
                    walkable: true,
                    height,
                    step_type,
                }),
            );
        }
        Arc::new(map)
    }

    fn make_character(cell_x: i32, cell_y: i32) -> CharacterLogic {
        ensure_init();
        let cell_size: f32 = 64.0;
        let pos = Vector2D {
            x: cell_x as f32 * cell_size + cell_size / 2.0,
            y: cell_y as f32 * cell_size + cell_size / 2.0,
        };
        let mut ch = CharacterLogic::new(
            cell_x as u32 * 1000 + cell_y as u32,
            Box::new(TestAnimator::new(pos)),
        );
        ch.prev_cell = Vector2Di::new(cell_x, cell_y);
        ch
    }

    // --- effective velocity tests ---

    #[test]
    fn test_effective_velocity_flat_ground_east() {
        let map = make_test_map();
        let mut ch = make_character(0, 0);
        ch.direction = Direction::EAST;
        ch.current_speed = 100.0;
        let v = ch.get_effective_velocity(&map);
        assert_eq!(v, Vector2D::new(100.0, 0.0));
    }

    #[test]
    fn test_effective_velocity_step_east() {
        let map = make_test_map();
        let mut ch = make_character(2, 0);
        ch.direction = Direction::EAST;
        ch.current_speed = 100.0;
        let v = ch.get_effective_velocity(&map);
        assert_eq!(v, Vector2D::new(100.0, -100.0));
    }

    #[test]
    fn test_effective_velocity_step_west() {
        let map = make_test_map();
        let mut ch = make_character(3, 0);
        ch.direction = Direction::WEST;
        ch.current_speed = 100.0;
        let v = ch.get_effective_velocity(&map);
        assert_eq!(v, Vector2D::new(-100.0, 100.0));
    }

    #[test]
    fn test_effective_velocity_step_north() {
        let map = make_test_map();
        let mut ch = make_character(2, 0);
        ch.direction = Direction::NORTH;
        ch.current_speed = 100.0;
        let v = ch.get_effective_velocity(&map);
        // N/S on step tiles → no diagonal, just normal direction
        assert_eq!(v, Vector2D::new(0.0, -100.0));
    }

    // --- process integration tests ---

    #[test]
    fn test_process_step_going_up() {
        let map = make_test_map();
        let mut ch = make_character(2, 0);
        ch.direction = Direction::EAST;
        ch.current_speed = 100.0;
        ch.request_state(StateRequest::Run);

        let start_pos = ch.get_position();
        for _ in 0..3 {
            ch.process(0.016, &map);
        }
        let end_pos = ch.get_position();

        assert!(end_pos.x > start_pos.x, "X should increase going east");
        assert!(end_pos.y < start_pos.y, "Y should decrease going up step");
    }

    #[test]
    fn test_process_step_going_down() {
        let map = make_test_map();
        let mut ch = make_character(3, 0);
        ch.direction = Direction::WEST;
        ch.current_speed = 100.0;
        ch.request_state(StateRequest::Run);

        let start_pos = ch.get_position();
        for _ in 0..3 {
            ch.process(0.016, &map);
        }
        let end_pos = ch.get_position();

        assert!(end_pos.x < start_pos.x, "X should decrease going west");
        assert!(end_pos.y > start_pos.y, "Y should increase going down step");
    }

    #[test]
    fn test_process_flat_ground_no_y_change() {
        let map = make_test_map();
        let mut ch = make_character(0, 0);
        ch.direction = Direction::EAST;
        ch.current_speed = 100.0;
        ch.request_state(StateRequest::Run);

        let start_y = ch.get_position().y;
        for _ in 0..3 {
            ch.process(0.016, &map);
        }
        let end_y = ch.get_position().y;

        assert!(
            (end_y - start_y).abs() < f32::EPSILON,
            "Y should not change on flat ground"
        );
    }
}
