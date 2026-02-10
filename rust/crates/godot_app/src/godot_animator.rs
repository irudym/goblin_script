use godot::classes::AnimatedSprite2D;
use godot::prelude::*;
use platform::animator::Animator;
use platform::types::Vector2D;

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

    fn set_position(&mut self, position: Vector2D) {
        self.sprite.set_position(Vector2 {
            x: position.x,
            y: position.y,
        });
    }

    fn get_position(&self) -> Vector2D {
        let pos = self.sprite.get_position();
        Vector2D { x: pos.x, y: pos.y }
    }

    fn get_global_position(&self) -> Vector2D {
        let pos = self.sprite.get_global_position();
        Vector2D { x: pos.x, y: pos.y }
    }

    fn process(&mut self, _delta: f32) {}
}
