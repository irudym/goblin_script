//Animation interface

pub trait Animator {
    fn play(&mut self, name: &str);
    fn is_playing(&self) -> bool;
    fn process(&mut self, delta: f32);
    fn set_position(&mut self, x: f32, y: f32);
    fn get_position(&self) -> (f32, f32);
}
