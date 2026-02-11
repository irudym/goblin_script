use godot::prelude::*;
use std::sync::Arc;

use game_core::map::LogicMap;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DebugOverlay {
    base: Base<Node2D>,

    logic_map: Option<Arc<LogicMap>>,
    tile_size: f32,
    visible: bool,
}

#[godot_api]
impl INode2D for DebugOverlay {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            logic_map: None,
            tile_size: 64.0,
            visible: true,
        }
    }

    fn process(&mut self, _delta: f64) {
        if self.visible {
            self.base_mut().queue_redraw();
        }
    }

    fn draw(&mut self) {
        if !self.visible {
            return;
        }

        let Some(map) = &self.logic_map.clone() else {
            return;
        };

        for j in 0..map.height {
            for i in 0..map.width {
                if !map.is_walkable(i as i32, j as i32) {
                    let rect = Rect2::new(
                        Vector2::new(i as f32 * self.tile_size, j as f32 * self.tile_size),
                        Vector2::new(self.tile_size, self.tile_size),
                    );

                    self.base_mut()
                        .draw_rect_ex(rect, Color::from_rgba(1.0, 0.0, 0.0, 0.4))
                        .filled(true)
                        .done();
                    self.base_mut()
                        .draw_rect_ex(rect, Color::from_rgba(0.2, 0.0, 0.0, 0.2))
                        .filled(false)
                        .done();
                }
            }
        }
    }
}

impl DebugOverlay {
    pub fn set_logic_map(&mut self, map: Arc<LogicMap>) {
        self.logic_map = Some(map);
        self.base_mut().queue_redraw();
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        self.base_mut().queue_redraw();
    }
}
