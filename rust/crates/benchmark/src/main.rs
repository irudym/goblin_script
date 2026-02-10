use game_core::bt::leafs::*;
use game_core::bt::nodes::*;
use game_core::bt::result::BTResult;
use game_core::bt::*;
use game_core::character::snapshot::CharacterSnapshot;
use game_core::CharacterLogic;
use platform::logger::{LogType, Logger};
use platform::shared::logger_global::init_logger;
use platform::types::Vector2D;
use platform::Animator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

fn build_tree(route: Vec<Vector2D>) -> Arc<BehaviourTree> {
    Arc::new(BehaviourTree::new(Box::new(Selector::new(vec![
        Box::new(Sequence::new(vec![
            Box::new(NextWaypoint::new(route.clone(), "target_pos")),
            Box::new(Wait::new(0.032)),
            Box::new(IsAtTarget::new("target_pos")),
        ])),
        Box::new(MoveToTarget::new("target_pos")),
    ]))))
}

fn main() {
    let n_characters = 10_000;
    let ticks = 5000;

    init_logger(Box::new(DummyLogger));

    println!("Running stress test for {n_characters} characters over {ticks} ticks.");

    // route sample
    let route = vec![
        Vector2D::new(0.0, 0.0),
        Vector2D::new(32.0, 0.0),
        Vector2D::new(32.0, 32.0),
        Vector2D::new(0.0, 32.0),
    ];

    let shared_tree = build_tree(route);

    // test in main thread
    let mut characters: Vec<CharacterLogic> = (0..n_characters)
        .map(|i| {
            let mut char = CharacterLogic::new(i, Box::new(DummyAnimator::new()), 32.0);
            char.bt = shared_tree.clone();
            char
        })
        .collect();

    let mut snapshots: Vec<CharacterSnapshot> = characters
        .iter()
        .map(|c| CharacterSnapshot {
            id: c.id,
            position: c.get_position(),
            direction: c.direction.clone(),
            velocity: Vector2D { x: 0.0, y: 0.0 },
            is_idle: c.is_idle(),
            blackboard: Default::default(),
            current_speed: 0.0,
        })
        .collect();

    let start_total = Instant::now();

    for tick in 0..ticks {
        let frame_start = Instant::now();

        let results: Vec<(usize, BTResult)> = snapshots
            .par_iter_mut()
            .enumerate()
            .map(|(i, snapshot)| {
                let result = shared_tree.tick(snapshot, 0.016);
                (i, result)
            })
            .collect();

        for (index, result) in results {
            for command in result.commands {
                characters[index].apply(command);
            }

            //update snapshot from characters
            snapshots[index].position = characters[index].get_position();
            snapshots[index].direction = characters[index].direction.clone();
        }

        let dt = frame_start.elapsed().as_micros();

        println!(
            "Frame {:04} => {:>6} µs ({} agents)",
            tick, dt, n_characters
        );
    }

    let total_time = start_total.elapsed();
    let avg_per_tick = total_time / ticks as u32;

    println!("\n==============================");
    println!(" Stress Test Completed");
    println!(" characters   = {}", n_characters);
    println!(" ticks        = {}", ticks);
    println!(" total time   = {:?}", total_time);
    println!(" avg/tick     = {:?}", avg_per_tick);
    println!(
        " avg/character/tick = {:?}",
        avg_per_tick / n_characters as u32
    );
    println!("==============================");
}

pub struct DummyAnimator {
    frames: HashMap<&'static str, (usize, bool)>, // animation name and (amount of frames, loop)
    current_animation: String,
    current_frame: usize,
    position: Vector2D,
}

impl DummyAnimator {
    pub fn new() -> Self {
        let mut frames = HashMap::new();
        frames.insert("stand_south", (1, true));
        frames.insert("stand_north", (1, true));
        frames.insert("stand_west", (1, true));
        frames.insert("stand_east", (1, true));
        frames.insert("turn_south_east", (3, false));
        frames.insert("turn_east_south", (3, false));
        frames.insert("turn_south_west", (3, false));
        frames.insert("turn_west_north", (3, false));
        frames.insert("turn_north_east", (3, false));

        Self {
            frames,
            current_animation: "".to_string(),
            current_frame: 0,
            position: Vector2D::new(0.0, 0.0),
        }
    }
}

impl Animator for DummyAnimator {
    fn play(&mut self, name: &str) {
        self.current_animation = name.to_string();
        self.current_frame = 0;
    }

    fn is_playing(&self) -> bool {
        if let Some(anim) = self.frames.get(self.current_animation.as_str()) {
            if anim.1 {
                return true;
            }
            if self.current_frame >= anim.0 {
                return false;
            } else {
                return true;
            }
        }
        false
    }

    fn process(&mut self, _delta: f32) {
        if let Some(anim) = self.frames.get(self.current_animation.as_str()) {
            self.current_frame += 1;
            if anim.1 {
                if self.current_frame >= anim.0 {
                    self.current_frame = 0;
                }
            } else {
                if self.current_frame >= anim.0 {
                    self.current_frame = anim.0;
                }
            }
        }
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

struct DummyLogger;

impl Logger for DummyLogger {
    fn log(&self, _t: LogType, _msg: &str) {}
}
