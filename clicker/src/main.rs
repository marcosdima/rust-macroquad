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
    let (x,y) = (screen_width()/2., screen_height()/2.);
    let r = 70.;
    let circle = Circle::new(x,y,r);
    let mut score = 0;
    
    loop {
        clear_background(BEIGE); // Clears screen and set this color
        
        draw_circle(x, y, r, BROWN);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x,mouse_y) = mouse_position();

            if circle.contains(&vec2(mouse_x, mouse_y)) {
                score += 1;
            }
        }

        draw_text(&format!("Score: {}", score), 20.0, 20.0, 30.0, BLACK);

        next_frame().await; // Waits for the next frame to be rendered before continuing the loop
    }
}
