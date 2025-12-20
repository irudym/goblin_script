mod console_animator;
mod console_logger;
use log::LevelFilter;

use console_animator::ConsoleAnimator;
use console_logger::ConsoleLogger;
use game_core::bt::blackboard::BlackboardValue;
use game_core::bt::leafs::{IsAtTarget, MoveToTarget, NextWaypoint, Wait};
use game_core::bt::nodes::{Selector, Sequence};
use game_core::bt::{BTNode, Blackboard};
use game_core::CharacterLogic;
use platform::logger::{LogType, Logger};
use platform::types::Vector2D;

fn main() {
    colog::basic_builder()
        .target(env_logger::Target::Stdout) // Forces output to stdout
        .filter_level(LevelFilter::Trace)
        .init();

    let logger = ConsoleLogger::new();
    logger.log(LogType::info, "Running console app: GoblinScript");

    let animator = ConsoleAnimator::new();

    let mut character = CharacterLogic::new(Box::new(animator), Box::new(logger));

    // test patrol
    let route = vec![
        Vector2D::new(0.0, 0.0),
        Vector2D::new(5.0, 0.0), // Move 5 tiles East
        Vector2D::new(5.0, 5.0), // Move 5 tiles South
        Vector2D::new(0.0, 5.0), // Return home
    ];
    let main_logger = ConsoleLogger::new();

    main_logger.log(LogType::info, &format!("Patrol points: {:?}", route));

    let blackboard = Blackboard::new();
    let first_point = Vector2D {
        x: route[0].x * 32.0,
        y: route[0].y * 32.0,
    };
    blackboard.set("target_pos", BlackboardValue::Vector(first_point));

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

    let mut root = Box::new(Selector::new(vec![
        Box::new(Sequence::new(vec![
            Box::new(IsAtTarget::new("target_pos")),
            Box::new(Wait::new(0.8)),
            Box::new(NextWaypoint::new(route, "target_pos")),
        ])),
        Box::new(MoveToTarget::new("target_pos")),
    ]));

    // run 10 cycles
    for i in 0..100 {
        main_logger.log(LogType::info, &format!("Cycle: {}", i));
        main_logger.log(
            LogType::debug,
            &format!("Character\nposition: {:?}", character.get_position()),
        );
        root.tick(&mut character, &blackboard, 0.15);
        character.process(0.15);
    }
}
