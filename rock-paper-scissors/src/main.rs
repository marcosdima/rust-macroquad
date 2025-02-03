mod entity;

use macroquad::prelude::*;
use entity::Entity;

fn conf() -> Conf {
    Conf {
      window_title: "Rock - Paper - Scissors".to_owned(),
      fullscreen:false,
      ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut x = 0.;
    let mut y = 0.;
    let texture: Texture2D = load_texture("assets/images/test.png").await.unwrap();
    let mut ente = Entity::new(x, y, texture);

    loop {
        clear_background(BEIGE);

        x += 1.;
        y += 1.;
        ente.set_position(x, y);
        ente.draw();

        next_frame().await;
    }
}