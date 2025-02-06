pub struct Coords {
    x: f32,
    y: f32,
}

impl Coords {
    pub fn new(x: f32, y: f32) -> Self {
        Coords {
            x,
            y,    
        }
    }

    pub fn pair(&self) -> (f32, f32) { (self.x, self.y) }

    pub fn set(&mut self, x: f32, y:f32) {
        self.x = x;
        self.y = y;
    }

    pub fn x(&self) -> f32 { self.x }

    pub fn y(&self) -> f32 { self.y }
}

pub struct Orientation {
    to: Coords,
}

impl Orientation {
    pub fn new(x: f32, y: f32) -> Self {
        if x < -1. || x > 1. {
            panic!("X ({x}) must be a number between -1 and 1.");
        }
        else if y < -1. || y > 1. {
            panic!("Y ({y}) must be a number between -1 and 1.");
        } else {
            Orientation { to: Coords::new(x, y) }
        }
    }

    pub fn set(&mut self, x: f32, y: f32) { self.to.set(x, y); }

    pub fn get(&self) -> &Coords { &self.to }
}

pub struct Entity {
    pos: Coords,
    orientation: Orientation,
    speed: u16,
    texture_id: String,
}

// TOTHINK: Unidad de movimiento que se consume.

impl Entity {
    pub fn new(pos: Coords, orientation: Orientation, speed: u16, texture_id: String) -> Self {
        Entity {
            pos,
            orientation,
            speed,
            texture_id,
        }
    }

    // GETTER & SETTER //
    pub fn set_position(&mut self, x: f32, y: f32) { self.pos.set(x, y); }

    pub fn get_position(&self) -> &Coords { &self.pos }

    pub fn set_orientation(&mut self, x: f32, y: f32) { self.orientation.set(x, y); }

    pub fn get_orientation(&self) -> &Orientation { &self.orientation }

    pub fn set_speed(&mut self, speed: u16) { self.speed = speed; }

    pub fn get_speed(&self) -> u16 { self.speed }

    pub fn set_texture_id(&mut self, texture_id: String) { self.texture_id = texture_id; }

    pub fn get_texture_id(&self) -> &str { &self.texture_id }
    // GETTER & SETTER //
}
