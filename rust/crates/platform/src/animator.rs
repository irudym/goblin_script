//Animation interface

use crate::types::Vector2D;

pub trait Animator {
    fn play(&mut self, name: &str);
    fn is_playing(&self) -> bool;
    fn process(&mut self, delta: f32);
    fn set_position(&mut self, position: Vector2D);
    fn get_position(&self) -> Vector2D;
    fn get_global_position(&self) -> Vector2D;
}
