use std::collections::HashMap;
use macroquad::prelude::*;
use crate::entity::Entity;

pub struct TextureHandler {
    textures: HashMap<String, Texture2D>,
}

impl TextureHandler {
    pub fn new() -> Self {
        TextureHandler {
            textures: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, id: &str, texture: Texture2D) {
        self.textures.insert(id.to_string(), texture);
    }

    pub fn draw(&self, entity: &Entity) {
        let entity_id = entity.get_texture_id();

        if let Some(texture) = self.textures.get(entity_id) {
            let (x, y) = entity.get_position().pair();
            draw_texture(texture, x, y, WHITE);
        } else {
            panic!("Error: Texture id '{}' was not found!", entity_id)
        }
    }
}
