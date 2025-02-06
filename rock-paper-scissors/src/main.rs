mod entity;
mod texture_handler;

use macroquad::prelude::*;
use entity::*;
use texture_handler::TextureHandler;

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

    let pos = Coords::new(200., 200.);
    let orientation = Degrees::new(290.);
    let mut ente = Entity::new(pos, orientation, 50, String::from("test"), MovementType::RANDOM);

    //let pos = Coords::new(500., 500.);
    //let orientation = Degrees::new(75.);
    //let mut ente2: Entity = Entity::new(pos, orientation, 50, String::from("test"), MovementType::LINEAR);

    let mut texture_handler = TextureHandler::new();
    texture_handler.add_texture("test", texture);

    let mut counter = 0;

    loop {
        clear_background(BEIGE);

        ente.move_body();
        //ente2.move_body();
        texture_handler.draw(&ente);
        //texture_handler.draw(&ente2);

        counter += 1;
        if counter == 100 {
            ente.set_orientation(Degrees::new(45.));
        }

        next_frame().await;
    }
}
