pub struct Entity {
    pos: (f32, f32),
    orientation: (f32, f32),
    speed: u16,
    texture_id: String,
}

impl Entity {
    pub fn new(pos: (f32, f32), orientation: (f32, f32), speed: u16, texture_id: String) -> Self {
        Entity {
            pos,
            orientation,
            speed,
            texture_id,
        }
    }

    // GETTER & SETTER //
    pub fn set_position(&mut self, x: f32, y: f32) { self.pos = (x, y); }

    pub fn get_position(&self) -> (f32, f32) { self.pos }

    pub fn set_orientation(&mut self, x: f32, y: f32) { self.orientation = (x, y); }

    pub fn get_orientation(&self) -> (f32, f32) { self.orientation }

    pub fn set_speed(&mut self, speed: u16) { self.speed = speed; }

    pub fn get_speed(&self) -> u16 { self.speed }

    pub fn set_texture_id(&mut self, texture_id: String) { self.texture_id = texture_id; }

    pub fn get_texture_id(&self) -> &str { &self.texture_id }
    // GETTER & SETTER //
}
