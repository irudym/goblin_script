use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

use crate::character::Character;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Scene {
    base: Base<Node2D>,
    characters: Vec<Gd<Character>>,
}

#[godot_api]
impl INode2D for Scene {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            characters: Vec::new(),
            base,
        }
    }

    fn ready(&mut self) {
        // get list of children
        let children = self.base().get_children();
        godot_print!("Children of the scene: {:?}", children);
        for node in children.iter_shared() {
            if let Ok(character) = node.try_cast::<Character>() {
                godot_print!("==>> Child: {}", &character.get_name());
                godot_print!("==>> Child type: {}", &character.get_class());
                self.characters.push(character);
            }
        }
    }

    fn process(&mut self, _delta: f32) {
        self.characters.retain(|c| c.is_instance_valid());

        //for char in self.characters.iter_mut() {
        //let mut character = char.bind_mut();
        //character.update_state(delta);
        //}
    }
}
