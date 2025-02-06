use macroquad::rand;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct Degrees(f32);

impl Degrees {
    pub fn new(angle: f32) -> Self {
        Self(angle.rem_euclid(360.0)) 
    }

    pub fn to_radians(self) -> f32 {
        self.0.to_radians()
    }

    pub fn add(self, other: f32) -> Self {
        Self::new(self.0 + other)
    }

    pub fn sub(self, other: f32) -> Self {
        Self::new(self.0 - other)
    }
}
pub enum MovementType {
    LINEAR,
    RANDOM,
    STATIC,
}

struct MovementHandler {
    movement_type: MovementType,
    trayectory: Box<dyn FnMut(f32) -> Coords>,
    counter: u32,
}

impl MovementHandler {
    pub fn new(movement_type: MovementType) -> Self {
        MovementHandler {
            movement_type,
            trayectory: Box::new(move |_e: f32| { Coords::new(0., 0.) }),
            counter: 0,
        }
    }

    pub fn update_trayectory(&mut self, point: Coords, angle: Degrees) {
        self.counter = 0;
        let (curr_x, curr_y) = point.pair();
        let r = angle.to_radians();

        match self.movement_type {
            MovementType::LINEAR => {
                let linear_movement = move |t: f32| -> Coords {
                    Coords {
                        x: curr_x + t * r.cos(),
                        y: curr_y - t * r.sin()
                    }
                };
        
                self.trayectory = Box::new(linear_movement);
            },
            MovementType::RANDOM => {
                let mut x = curr_x;
                let mut y = curr_y;

                self.trayectory = Box::new(
                    move |_t: f32| -> Coords {
                        let bool = rand::gen_range(0, 2);

                        if bool == 0 {
                            x += r.cos();
                        } else {
                            y -= r.sin();
                        }

                        Coords { x, y }
                    }
                );
            }
            _ => {}
        }
    }

    pub fn get_next_coords(&mut self) -> Coords {
        self.counter += 1;
        (self.trayectory)(self.counter as f32)
    } 
}

pub struct Entity {
    pos: Coords,
    orientation: Degrees,
    speed: u16,
    texture_id: String,
    movement_handler: MovementHandler,
}

// TOTHINK: Unidad de movimiento que se consume.

impl Entity {
    pub fn new(pos: Coords, orientation: Degrees, speed: u16, texture_id: String, movement_type: MovementType) -> Self {
        let mut ent = Entity {
            pos,
            orientation,
            speed,
            texture_id,
            movement_handler: MovementHandler::new(movement_type),
        };

        // Updates trayectory.
        ent.set_orientation(orientation);

        ent
    }

    // GETTER & SETTER //
    pub fn set_position(&mut self, x: f32, y: f32) { self.pos.set(x, y); }

    pub fn get_position(&self) -> Coords { self.pos }

    pub fn set_orientation(&mut self, degrees: Degrees) {
        self.orientation = degrees;

        let point = self.get_position();
        let angle = self.get_orientation();
        let movement_handler = &mut self.movement_handler;
        
        movement_handler.update_trayectory(point, angle);
    }

    pub fn get_orientation(&self) -> Degrees { self.orientation }

    pub fn set_speed(&mut self, speed: u16) { self.speed = speed; }

    pub fn get_speed(&self) -> u16 { self.speed }

    pub fn set_texture_id(&mut self, texture_id: String) { self.texture_id = texture_id; }

    pub fn get_texture_id(&self) -> &str { &self.texture_id }
    // GETTER & SETTER //

    pub fn move_body(&mut self) { self.pos = self.movement_handler.get_next_coords(); } 
}
