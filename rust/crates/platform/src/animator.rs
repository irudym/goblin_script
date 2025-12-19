//Animation interface

pub trait Animator: Send {
    fn play(&mut self, name: &str);
    fn is_playing(&self) -> bool;
    fn process(&mut self, delta: f32);
}
