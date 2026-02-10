mod console_animator;
mod console_logger;
use log::LevelFilter;

use console_animator::ConsoleAnimator;
use console_logger::ConsoleLogger;
use game_core::ai::worker::init_bt_system;
use game_core::bt::leafs::{IsAtTarget, MoveToTarget, NextWaypoint, Wait};
use game_core::bt::nodes::{Selector, Sequence};
use game_core::bt::BehaviourTree;
use game_core::CharacterLogic;
use platform::logger::LogType;
use platform::types::Vector2D;
use std::sync::Arc;
use std::time::Duration;

use platform::shared::logger_global::{init_logger, log};
use platform::{log, log_info};

fn main() {
    colog::basic_builder()
        .target(env_logger::Target::Stdout) // Forces output to stdout
        .filter_level(LevelFilter::Trace)
        .init();

    init_logger(Box::new(ConsoleLogger));
    log_info!("Running console app: GoblinScript");

    let animator = ConsoleAnimator::new();

    // test patrol
    let route = vec![
        Vector2D::new(10.0, 10.0),
        Vector2D::new(15.0, 10.0), // Move 5 tiles East
        Vector2D::new(15.0, 25.0), // Move 5 tiles South
        Vector2D::new(10.0, 25.0), // Return home
    ];

    log!(LogType::Info, "Patrol points: {:?}", route);

    init_bt_system();

    //blackboard.set("target_pos", BlackboardValue::Vector(first_point));

    // Tree structure:
    // Sequence
    //  1. Patrol Logic (update target, return Success when arrived)
    //  2. Wait (pause)
    //  3. WalkToTarget

    // Actually, a better structure for continuous movement is:
    // Selector
    //   1. Sequence (Target Reached?)
    //       a. Patrol (Calculate next point if arrived)
    //       b. Wait (Visualize looking around)
    //   2. WalkToTarget (Keep moving to current target)

    let tree = Arc::new(BehaviourTree::new(Box::new(Selector::new(vec![
        Box::new(Sequence::new(vec![
            Box::new(NextWaypoint::new(route, "target_pos")),
            //Box::new(IsAtTarget::new("target_pos")),
            Box::new(Wait::new(0.032)),
            Box::new(IsAtTarget::new("target_pos")),
        ])),
        Box::new(MoveToTarget::new("target_pos")),
    ]))));

    let mut character = CharacterLogic::new(1, Box::new(animator), 32.0);
    character.bt = tree;
    character.set_position(Vector2D { x: 320.0, y: 320.0 });

    // run 10 cycles
    for i in 0..500 {
        log!(LogType::Info, "Cycle: {}", i);
        log!(
            LogType::Debug,
            "Character\nposition: {:?}",
            character.get_position(),
        );
        //character.process(0.016, &map);
        std::thread::sleep(Duration::from_millis(50));
    }
}
