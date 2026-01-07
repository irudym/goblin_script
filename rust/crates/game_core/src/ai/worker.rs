use crate::bt::{Blackboard, BoxBTNode};
use crate::character::command::CharacterCommand;
use crate::character::snapshot::CharacterSnapshot;
use std::sync::mpsc::{Receiver, Sender};

pub fn bt_worker(
    mut tree: BoxBTNode,
    snapshot_rx: Receiver<CharacterSnapshot>,
    command_tx: Sender<Vec<CharacterCommand>>,
) {
    let blackboard = Blackboard::new();
    while let Ok(snapshot) = snapshot_rx.recv() {
        let mut commands = Vec::new();
        tree.tick(&snapshot, &blackboard, 0.016, &mut commands);
        let _ = command_tx.send(commands);
    }
}
