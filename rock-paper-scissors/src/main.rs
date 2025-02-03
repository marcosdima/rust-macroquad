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
    let x = 0.;
    let y = 0.;
    let texture: Texture2D = load_texture("assets/images/test.png").await.unwrap();
    let ente = Entity::new(x, y, texture);

    loop {
        clear_background(BEIGE);

        ente.draw();

        next_frame().await;
    }
}