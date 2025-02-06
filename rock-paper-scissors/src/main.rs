mod entity;
mod texture_handler;

use macroquad::prelude::*;
use entity::*;
use texture_handler::TextureHandler;

pub enum MovementType {
    LINEAR,
    RANDOM,
}

pub fn move_ent(entity: &mut Entity, movement: MovementType) {
    let (curr_x, curr_y) = entity.get_position().pair();
    let (oriented_x, oriented_y) = entity.get_orientation().get().pair();
    let speed = entity.get_speed() as f32;
    let delta = get_frame_time();

    let updated_x = curr_x + delta * speed * oriented_x;
    let updated_y = curr_y + delta * speed * oriented_y;

    match movement {
        MovementType::LINEAR => {
            entity.set_position(updated_x, updated_y);
        },
        MovementType::RANDOM => {
            let bool = rand::gen_range(0, 2);
            entity.set_position(
                if bool == 0 { updated_x } else { curr_x }, 
                if bool > 0 { updated_y} else { curr_y }
            );
        }
    }
}

fn conf() -> Conf {
    Conf {
      window_title: "Rock - Paper - Scissors".to_owned(),
      fullscreen:false,
      ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let texture: Texture2D = load_texture("assets/images/test.png").await.unwrap();

    let pos = Coords::new(0., 0.);
    let orientation = Orientation::new(1., 1.);
    let mut ente = Entity::new(pos, orientation, 50, String::from("test"));

    let pos = Coords::new(500., 500.);
    let orientation = Orientation::new(-1., -1.);
    let mut ente2: Entity = Entity::new(pos, orientation, 50, String::from("test"));

    let mut texture_handler = TextureHandler::new();
    texture_handler.add_texture("test", texture);

    loop {
        clear_background(BEIGE);

        move_ent(&mut ente, MovementType::RANDOM);
        move_ent(&mut ente2, MovementType::RANDOM);
        texture_handler.draw(&ente);
        texture_handler.draw(&ente2);

        next_frame().await;
    }
}
