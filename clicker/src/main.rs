use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
      window_title: "Clicker".to_owned(), // This field is a must!
      fullscreen:false,
      // More configurations...
      ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    loop {
        clear_background(BEIGE); // Clears screen and set this color
        
        next_frame().await; // Waits for the next frame to be rendered before continuing the loop
    }
}
