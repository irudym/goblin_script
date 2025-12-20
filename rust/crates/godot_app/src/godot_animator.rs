use godot::classes::AnimatedSprite2D;
use godot::prelude::*;
use platform::animator::Animator;

pub struct GodotAnimator {
    sprite: Gd<AnimatedSprite2D>,
}

impl GodotAnimator {
    pub fn new(sprite: Gd<AnimatedSprite2D>) -> Self {
        Self { sprite }
    }
}

impl Animator for GodotAnimator {
    fn play(&mut self, name: &str) {
        self.sprite.set_animation(name);
        self.sprite.play();
    }

    fn is_playing(&self) -> bool {
        self.sprite.is_playing()
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.sprite.set_position(Vector2 { x, y });
    }

    fn get_position(&self) -> (f32, f32) {
        let pos = self.sprite.get_position();
        (pos.x, pos.y)
    }

    fn process(&mut self, _delta: f32) {}
}
