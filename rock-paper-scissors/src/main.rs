mod entity;
mod texture_handler;

use macroquad::prelude::*;
use entity::Entity;
use texture_handler::TextureHandler;

pub enum MovementType {
    LINEAR,
}

pub fn move_ent(entity: &mut Entity, movement: MovementType) {
    let (curr_x, curr_y) = entity.get_position();
    let (oriented_x, oriented_y) = entity.get_orientation();
    let speed = entity.get_speed() as f32;
    let delta = get_frame_time();

    match movement {
        MovementType::LINEAR => {
            let updated_x = curr_x + delta * speed * oriented_x;
            let updated_y = curr_y + delta * speed * oriented_y;
            entity.set_position(updated_x, updated_y);
        },
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

    let mut ente = Entity::new((0., 0.), (1., 1.), 100, String::from("test"));

    let mut texture_handler = TextureHandler::new();
    texture_handler.add_texture("test", texture);

    loop {
        clear_background(BEIGE);

        move_ent(&mut ente, MovementType::LINEAR);
        texture_handler.draw(&ente);

        next_frame().await;
    }
}
