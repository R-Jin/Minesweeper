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
    Hidden,
}

struct Board {
    x_cells: usize,
    y_cells: usize,
    gap: f32,
    padding: f32,
    state: Vec<Vec<CellState>>,
}

impl Board {
    pub fn new(x_cells: usize, y_cells: usize, gap: f32, padding: f32) -> Self {
        let row = vec![CellState::Hidden; x_cells];
        Self {
            x_cells,
            y_cells,
            gap,
            padding,
            state: vec![row; y_cells],
        }
    }

    pub fn draw_board(&self) {
        let tile_width: f32 =
            (screen_width() - self.padding - self.gap * self.x_cells as f32) / self.x_cells as f32;

        for row in 0..self.y_cells {
            for col in 0..self.x_cells {
                let x = self.padding as f32 + col as f32 * (self.gap + tile_width);
                let y = self.padding as f32 + row as f32 * (self.gap + tile_width);
                let c = match self.state[row][col] {
                    CellState::Mine => BLACK,
                    CellState::Empty => GREEN,
                    CellState::Number(_) => PINK,
                    CellState::Hidden => GRAY,
                };
                draw_rectangle(x, y, tile_width, tile_width, c);
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // changing screen size
    let board = Board::new(16, 16, 1.0, 2.0);

    loop {
        clear_background(WHITE);
        // Input
        let pos = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) {
            println!("x: {}, y: {}", pos.0, pos.1)
        }

        // Update

        // Draw
        board.draw_board();

        next_frame().await
    }
}
