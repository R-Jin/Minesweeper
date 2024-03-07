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

#[derive(Clone, Debug)]
enum CellState {
    Mine,
    Number(i32),
    Empty,
}

struct Board {
    x_cells: usize,
    y_cells: usize,
    state: Vec<Vec<CellState>>,
}

impl Board {
    pub fn new(x_cells: usize, y_cells: usize) -> Self {
        let row = vec![CellState::Empty; x_cells];
        Self {
            x_cells,
            y_cells,
            state: vec![row; y_cells],
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // changing screen size

    // Initialize a board of size 16x16

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
