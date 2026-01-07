use crate::bt::{blackboard::Blackboard, blackboard::BlackboardValue, BTNode, NodeStatus};
use crate::character::command::CharacterCommand;
use crate::character::request::StateRequest;
use crate::character::snapshot::CharacterSnapshot;
use platform::types::{Direction, Vector2D};

pub struct FindTarget {
    target_key: String, // "target_pos"
}

impl FindTarget {
    pub fn new(key: &str) -> Self {
        Self {
            target_key: key.to_string(),
        }
    }
}

impl BTNode for FindTarget {
    fn reset(&mut self) {}
    fn tick(
        &mut self,
        _snapshot: &CharacterSnapshot,
        blackboard: &Blackboard,
        _delta: f32,
        _out: &mut Vec<CharacterCommand>,
    ) -> NodeStatus {
        //TODO: placeholder!
        //
        let player_found = true;
        let player_position = Vector2D::new(100.0, 200.0);

        if player_found {
            // write to memory
            blackboard.set(&self.target_key, BlackboardValue::Vector(player_position));
            return NodeStatus::SUCCESS;
        }

        return NodeStatus::FAILURE;
    }
}

pub struct MoveToTarget {
    target_key: String,
}

impl MoveToTarget {
    pub fn new(key: &str) -> Self {
        Self {
            target_key: key.to_string(),
        }
    }
}

impl BTNode for MoveToTarget {
    fn reset(&mut self) {}
    fn tick(
        &mut self,
        snapshot: &CharacterSnapshot,
        blackboard: &Blackboard,
        _delta: f32,
        out: &mut Vec<CharacterCommand>,
    ) -> NodeStatus {
        println!("[-] Node: MoveToTarget");
        //1. read from memory (blackboard) the target position
        let target_pos = match blackboard.get(&self.target_key) {
            Some(BlackboardValue::Vector(v)) => v,
            _ => return NodeStatus::FAILURE, //no target set, or wrong value type
        };

        // check if the character has arrived to the to target pos
        let current_pos = snapshot.position;
        let distance = current_pos.distance_to(target_pos);

        println!(
            "| ---> target_pos: {:?}, distance: {}",
            target_pos, distance
        );

        if distance < 6.0 {
            // the character has arrived
            // fix his position
            //character.snap_to_cell();
            //character.request_state(StateRequest::Idle);
            out.push(CharacterCommand::ChangeState(StateRequest::Idle));
            return NodeStatus::SUCCESS;
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
            //character.request_state(StateRequest::Turn(new_direction));
            println!(
                "| ---> need to switch to new direction: {} from: {}",
                new_direction, snapshot.direction
            );
            out.push(CharacterCommand::ChangeState(StateRequest::Turn(
                new_direction,
            )))
        } else {
            //trugger run state
            out.push(CharacterCommand::ChangeState(StateRequest::Run));
        }

        return NodeStatus::RUNNING;
    }
}

pub struct NextWaypoint {
    waypoints: Vec<Vector2D>,
    current_index: usize,
    target_key: String,
    tile_size: f32,
}

impl NextWaypoint {
    pub fn new(waypoints: Vec<Vector2D>, key: &str) -> Self {
        Self {
            waypoints,
            current_index: 0,
            target_key: key.to_string(),
            tile_size: 32.0,
        }
    }
}

impl BTNode for NextWaypoint {
    fn reset(&mut self) {}

    fn tick(
        &mut self,
        _snapshot: &CharacterSnapshot,
        blackboard: &Blackboard,
        _delta: f32,
        _out: &mut Vec<CharacterCommand>,
    ) -> NodeStatus {
        println!("[-] Node: NextWaypoint");
        if self.waypoints.is_empty() {
            return NodeStatus::FAILURE;
        }

        self.current_index = (self.current_index + 1) % self.waypoints.len();

        let next_pos = self.waypoints[self.current_index] * self.tile_size;
        blackboard.set(&self.target_key, BlackboardValue::Vector(next_pos));

        NodeStatus::SUCCESS
    }
}

pub struct IsAtTarget {
    target_key: String,
    threshold: f32,
}

impl IsAtTarget {
    pub fn new(key: &str) -> Self {
        Self {
            target_key: key.to_string(),
            threshold: 6.0,
        }
    }
}

impl BTNode for IsAtTarget {
    fn reset(&mut self) {}

    fn tick(
        &mut self,
        snapshot: &CharacterSnapshot,
        blackboard: &Blackboard,
        _delta: f32,
        _out: &mut Vec<CharacterCommand>,
    ) -> NodeStatus {
        println!("[-] Node: IsAtTarget: blackboard: {:?}", &blackboard);
        let target_pos = match blackboard.get(&self.target_key) {
            Some(BlackboardValue::Vector(v)) => v,
            _ => return NodeStatus::FAILURE,
        };

        let dist = snapshot.position.distance_to(target_pos);

        if dist <= self.threshold {
            NodeStatus::SUCCESS
        } else {
            NodeStatus::FAILURE
        }
    }
}

pub struct WalkToTarget {
    target_key: String,
}

impl WalkToTarget {
    pub fn new(key: &str) -> Self {
        Self {
            target_key: key.to_string(),
        }
    }
}

impl BTNode for WalkToTarget {
    fn tick(
        &mut self,
        snapshot: &CharacterSnapshot,
        blackboard: &Blackboard,
        _delta: f32,
        out: &mut Vec<CharacterCommand>,
    ) -> NodeStatus {
        //1. read target
        let target_pos = match blackboard.get(&self.target_key) {
            Some(BlackboardValue::Vector(v)) => v,
            _ => {
                return NodeStatus::FAILURE;
            }
        };

        //2. check FSM state:
        // Idle and ready to move?
        // Walking, then character is busy
        if snapshot.is_idle {
            let current_pos = snapshot.position;

            if current_pos.distance_to(target_pos) > 1.0 {
                //character.request_state(StateRequest::WalkTo(target_pos));
                out.push(CharacterCommand::ChangeState(StateRequest::WalkTo(
                    target_pos,
                )));
            } else {
                return NodeStatus::SUCCESS;
            }
        }
        NodeStatus::RUNNING
    }

    fn reset(&mut self) {}
}

pub struct Wait {
    delay: f32,
    current_time: f32,
}

impl Wait {
    pub fn new(delay: f32) -> Self {
        Self {
            delay,
            current_time: 0.0,
        }
    }
}

impl BTNode for Wait {
    fn reset(&mut self) {
        self.current_time = 0.0;
    }

    fn tick(
        &mut self,
        _snapshot: &CharacterSnapshot,
        _blackboard: &Blackboard,
        delta: f32,
        _out: &mut Vec<CharacterCommand>,
    ) -> NodeStatus {
        println!("[-] Node: Wait({}/{})", &self.current_time, &self.delay);
        self.current_time += delta;
        if self.current_time > self.delay {
            <Wait as BTNode>::reset(self);
            return NodeStatus::SUCCESS;
        }
        NodeStatus::RUNNING
    }
}
