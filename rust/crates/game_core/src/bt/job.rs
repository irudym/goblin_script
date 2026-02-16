use super::CharacterSnapshot;
use crate::{bt::BTRef, character::CharacterId};

pub struct BTJob {
    pub character_id: CharacterId,
    pub snapshot: CharacterSnapshot,
    pub bt: BTRef,
    pub delta: f32,
}
