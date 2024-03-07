use macroquad::prelude::*;
use macroquad::window::Conf;

fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: 500,
        window_height: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // changing screen size
    loop {
        clear_background(WHITE);
        // Input
        let pos = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) {
            println!("x: {}, y: {}", pos.0, pos.1)
        }

        // Update

        // Draw

        next_frame().await
    }
}
