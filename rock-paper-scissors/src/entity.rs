pub struct Entity {
    pos: (f32, f32),
    texture_id: String,
}

impl Entity {
    pub fn new(x: f32, y: f32, texture_id: String) -> Self {
        Entity {
            pos: (x, y),
            texture_id,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.pos = (x, y);
    }

    pub fn get_position(&self) -> (f32, f32) {
        self.pos
    }

    pub fn get_texture_id(&self) -> &str {
        &self.texture_id
    }
}
