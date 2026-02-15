use crate::bt::command::BTCommand;
use crate::bt::result::BTResult;
use crate::bt::{blackboard::BlackboardValue, BTNode, NodeStatus};
use crate::character::request::StateRequest;
use crate::character::snapshot::CharacterSnapshot;
//use platform::log_debug;
//use platform::logger::LogType;
//use platform::shared::logger_global::log;
use platform::types::{Direction, Vector2D};
// use platform::{log, log_info};

pub struct FindTarget {
    target_key: String, // "target_pos"
    id: usize,
}

impl FindTarget {
    pub fn new(key: &str) -> Self {
        Self {
            target_key: key.to_string(),
            id: 0,
        }
    }
}

impl BTNode for FindTarget {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn reset(&mut self) {}

    fn tick(&self, snapshot: &CharacterSnapshot, _delta: f32) -> (NodeStatus, BTResult) {
        //TODO: placeholder!
        //
        let player_found = true;
        let player_position = Vector2D::new(100.0, 200.0);

        if player_found {
            // write to memory
            snapshot
                .blackboard
                .set(&self.target_key, BlackboardValue::Vector(player_position));
            return (NodeStatus::SUCCESS, BTResult::empty());
        }

        return (NodeStatus::FAILURE, BTResult::empty());
    }
}

pub struct MoveToTarget {
    target_key: String,
    id: usize,
}

impl MoveToTarget {
    pub fn new(key: &str) -> Self {
        Self {
            target_key: key.to_string(),
            id: 0,
        }
    }
}

impl BTNode for MoveToTarget {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn reset(&mut self) {}

    fn tick(&self, snapshot: &CharacterSnapshot, _delta: f32) -> (NodeStatus, BTResult) {
        let bb = &snapshot.blackboard;

        let mut commands = vec![];
        //1. read from memory (blackboard) the target position
        let target_pos = match bb.get(&self.target_key) {
            Some(BlackboardValue::Vector(v)) => v,
            _ => return (NodeStatus::FAILURE, BTResult { commands }), //no target set, or wrong value type
        };

        // check if the character has arrived to the to target pos
        let current_pos = snapshot.position;
        let distance = current_pos.distance_to(target_pos);

        if distance < 6.0 {
            // the character has arrived
            // fix his position
            commands.push(BTCommand::SnapToCell);
            commands.push(BTCommand::ChangeState(StateRequest::Idle));
            return (NodeStatus::SUCCESS, BTResult { commands });
        }

        //2. calculate direction logic
        let direction_vector = (target_pos - current_pos).normalized();

        let new_direction = if direction_vector.x.abs() > direction_vector.y.abs() {
            if direction_vector.x > 0.0 {
                Direction::EAST
            } else {
                Direction::WEST
            }
        } else {
            if direction_vector.y > 0.0 {
                Direction::SOUTH
            } else {
                Direction::NORTH
            }
        };

        //3. update FSM state
        if snapshot.direction != new_direction {
            //need to turn
            commands.push(BTCommand::ChangeState(StateRequest::Turn(new_direction)));
        } else {
            //trigger run state
            commands.push(BTCommand::ChangeState(StateRequest::Run));
        }

        return (NodeStatus::RUNNING, BTResult { commands });
    }
}

pub struct NextWaypoint {
    waypoints: Vec<Vector2D>,
    target_key: String,
    tile_size: f32,
    id: usize,
}

impl NextWaypoint {
    pub fn new(waypoints: Vec<Vector2D>, key: &str, tile_size: f32) -> Self {
        Self {
            waypoints,
            target_key: key.to_string(),
            tile_size,
            id: 0,
        }
    }
}

impl BTNode for NextWaypoint {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }
    fn reset(&mut self) {}

    fn tick(&self, snapshot: &CharacterSnapshot, _delta: f32) -> (NodeStatus, BTResult) {
        let bb = &snapshot.blackboard;

        let key = format!("{}.nxw", self.id);

        let result = BTResult::empty();

        if self.waypoints.is_empty() {
            return (NodeStatus::FAILURE, result);
        }

        let mut current_index = match bb.get(&key) {
            Some(BlackboardValue::Int(val)) => val as usize,
            _ => 0,
        };

        current_index = (current_index + 1) % self.waypoints.len();

        // TODO: need to have an external function to calculate next position taking into account map tile size and current map coordinates (in case if scrolling).
        let half_tile = Vector2D::new(self.tile_size / 2.0, self.tile_size / 2.0);
        let next_pos = self.waypoints[current_index] * self.tile_size + half_tile;

        bb.set(&self.target_key, BlackboardValue::Vector(next_pos));

        bb.set(&key, BlackboardValue::Int(current_index as i32));
        (NodeStatus::SUCCESS, result)
    }
}

pub struct IsAtTarget {
    target_key: String,
    threshold: f32,
    id: usize,
}

impl IsAtTarget {
    pub fn new(key: &str) -> Self {
        Self {
            target_key: key.to_string(),
            threshold: 16.0,
            id: 0,
        }
    }
}

impl BTNode for IsAtTarget {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn reset(&mut self) {}

    fn tick(&self, snapshot: &CharacterSnapshot, _delta: f32) -> (NodeStatus, BTResult) {
        let target_pos = match snapshot.blackboard.get(&self.target_key) {
            Some(BlackboardValue::Vector(v)) => v,
            _ => return (NodeStatus::FAILURE, BTResult::empty()),
        };

        let dist = snapshot.position.distance_to(target_pos);

        /*
        log_debug!(
            "IsAtTarget: snapshot: {:?}, target_pos: {:?}, dist: {}, direction: {:?}",
            &snapshot,
            target_pos,
            dist,
            &snapshot.direction
        );
        */

        //TODO: need to somehow correct a character coordinate
        if dist <= self.threshold {
            (NodeStatus::SUCCESS, BTResult::empty())
        } else {
            //get direction
            /*
            log_debug!("[IsAtTarget]: direction: {}", snapshot.direction);
            match snapshot.direction {
                Direction::EAST => {
                    if snapshot.position.x > target_pos.x {
                        return (NodeStatus::SUCCESS, BTResult::empty());
                    }
                }
                Direction::WEST => {
                    if snapshot.position.x < target_pos.x {
                        return (NodeStatus::SUCCESS, BTResult::empty());
                    }
                }
                Direction::NORTH => {
                    if snapshot.position.y < target_pos.y {
                        return (NodeStatus::SUCCESS, BTResult::empty());
                    }
                }
                Direction::SOUTH => {
                    if snapshot.position.y > target_pos.y {
                        return (NodeStatus::SUCCESS, BTResult::empty());
                    }
                }
            }
            */
            (NodeStatus::FAILURE, BTResult::empty())
        }
    }
}

pub struct WalkToTarget {
    target_key: String,
    id: usize,
}

impl WalkToTarget {
    pub fn new(key: &str) -> Self {
        Self {
            target_key: key.to_string(),
            id: 0,
        }
    }
}

impl BTNode for WalkToTarget {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn tick(&self, snapshot: &CharacterSnapshot, _delta: f32) -> (NodeStatus, BTResult) {
        //1. read target
        let target_pos = match snapshot.blackboard.get(&self.target_key) {
            Some(BlackboardValue::Vector(v)) => v,
            _ => {
                return (NodeStatus::FAILURE, BTResult::empty());
            }
        };

        //2. check FSM state:
        // Idle and ready to move?
        // Walking, then character is busy
        let mut commands = vec![];
        if snapshot.is_idle {
            let current_pos = snapshot.position;

            if current_pos.distance_to(target_pos) > 1.0 {
                //character.request_state(StateRequest::WalkTo(target_pos));
                commands.push(BTCommand::ChangeState(StateRequest::WalkTo(target_pos)));
            } else {
                return (NodeStatus::SUCCESS, BTResult { commands });
            }
        }
        (NodeStatus::RUNNING, BTResult { commands })
    }

    fn reset(&mut self) {}
}

pub struct Wait {
    delay: f32,
    id: usize,
}

impl Wait {
    pub fn new(delay: f32) -> Self {
        Self { delay, id: 0 }
    }
}

impl BTNode for Wait {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn reset(&mut self) {}

    fn tick(&self, snapshot: &CharacterSnapshot, delta: f32) -> (NodeStatus, BTResult) {
        let bb = &snapshot.blackboard;
        let key = format!("{}.timer", self.id);

        let mut current_time = match bb.get(&key) {
            Some(BlackboardValue::Float(val)) => val,
            _ => 0.0,
        };

        current_time += delta;
        if current_time > self.delay {
            bb.set(&key, BlackboardValue::Float(0.0));
            return (NodeStatus::SUCCESS, BTResult::empty());
        }

        bb.set(&key, BlackboardValue::Float(current_time));
        (NodeStatus::RUNNING, BTResult::empty())
    }
}
