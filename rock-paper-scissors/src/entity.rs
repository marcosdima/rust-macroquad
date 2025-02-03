use macroquad::prelude::*;

pub struct Entity {
    pub pos: Vec2,
    pub texture: Texture2D,
}

impl Entity {
    pub fn new(x: f32, y: f32, texture: Texture2D) -> Self {
        Entity {
            pos: vec2(x, y),
            texture,
        }
    }

    pub fn draw(&self) {
        draw_texture(&self.texture, self.pos.x, self.pos.y, BEIGE);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.pos = vec2(x, y);
    }
}

/*
Require:
- Draw Texture. -> draw_texture(texture, x, y, color);
- Move.
- Animate.
- Make a sound.
*/