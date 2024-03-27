extern crate rand;

use macroquad::prelude::*;
use macroquad::window::Conf;
use rand::seq::index;

fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: 500,
        window_height: 500,
        ..Default::default()
    }
}

#[derive(Clone, Debug)]
enum CellType {
    Mine,
    Number(usize),
    Empty,
}

#[derive(Clone, Debug)]
enum CellState {
    Visible,
    Hidden,
}

#[derive(Clone, Debug)]
struct Cell {
    cell_type: CellType,
    cell_state: CellState,
}

impl Cell {
    pub fn new(cell_type: CellType, cell_state: CellState) -> Self {
        Self {
            cell_type,
            cell_state,
        }
    }

    pub fn update_state(&mut self, cell_state: CellState) {
        self.cell_state = cell_state;
    }
}

type State = Vec<Vec<Cell>>;

struct Board {
    x_cells: usize,
    y_cells: usize,
    gap: f32,
    padding: f32,
    tile_width: f32,
    state: State,
}

impl Board {
    pub fn new(x_cells: usize, y_cells: usize, gap: f32, padding: f32, mines: usize) -> Self {
        let mut state: State =
            vec![vec![Cell::new(CellType::Empty, CellState::Hidden); x_cells]; y_cells];

        let mut rng = rand::thread_rng();

        let flattened_indexes = index::sample(&mut rng, x_cells * y_cells, mines);

        // Convert flattened index to row and column indexes
        let mine_positions = flattened_indexes
            .iter()
            .map(|i| (i.div_ceil(x_cells).saturating_sub(1), i % x_cells)); // div_floor does not work very weird

        for pos in mine_positions {
            let (x, y) = pos;
            state[x][y].cell_type = CellType::Mine;

            // Put in the numbers
            let x_upper = if x + 1 >= y_cells { x } else { x + 1 };
            let x_lower = x.saturating_sub(1);

            let y_upper = if y + 1 >= y_cells { y } else { y + 1 };
            let y_lower = y.saturating_sub(1);

            for x in x_lower..=x_upper {
                for y in y_lower..=y_upper {
                    match state[x][y].cell_type {
                        CellType::Mine => {}
                        CellType::Empty => {
                            state[x][y].cell_type = CellType::Number(1);
                        }
                        CellType::Number(n) => {
                            state[x][y].cell_type = CellType::Number(n + 1);
                        }
                    }
                }
            }
        }

        Self {
            x_cells,
            y_cells,
            gap,
            padding,
            tile_width: (screen_width() - padding - gap * x_cells as f32) / x_cells as f32,
            state,
        }
    }

    pub fn draw(&self) {
        for row in 0..self.y_cells {
            for col in 0..self.x_cells {
                let x = self.padding as f32 + col as f32 * (self.gap + self.tile_width);
                let y = self.padding as f32 + row as f32 * (self.gap + self.tile_width);
                let cell = &self.state[row][col];
                match cell.cell_state {
                    CellState::Hidden => {
                        draw_rectangle(x, y, self.tile_width, self.tile_width, GRAY);
                    }
                    CellState::Visible => match cell.cell_type {
                        CellType::Mine => {
                            draw_rectangle(x, y, self.tile_width, self.tile_width, BLACK);
                        }
                        CellType::Empty => {
                            draw_rectangle(x, y, self.tile_width, self.tile_width, GREEN);
                        }
                        CellType::Number(n) => {
                            draw_rectangle(x, y, self.tile_width, self.tile_width, PINK);
                            draw_text(
                                n.to_string().as_str(),
                                x + self.tile_width / 2.0 - 5.0,
                                y + self.tile_width - 5.0,
                                self.tile_width,
                                BLACK,
                            );
                        }
                    },
                };
            }
        }
    }

    fn on_gap(&self, col: usize, row: usize, mouse_pos: (f32, f32)) -> bool {
        // Check if the mouse position is on gap
        !(mouse_pos.0 <= self.tile_width * (col as f32 + 1.0) + self.gap * col as f32)
            || !(mouse_pos.1 <= self.tile_width * (row as f32 + 1.0) + self.gap * row as f32)
    }

    pub fn update(&mut self, mouse_pos: (f32, f32)) {
        let col = (mouse_pos.0 / (self.tile_width + self.gap)).floor() as usize;
        let row = (mouse_pos.1 / (self.tile_width + self.gap)).floor() as usize;
        // println!("{}", self.on_gap(col, row, mouse_pos))
        if !self.on_gap(col, row, mouse_pos) {
            if let Some(clicked_cell) = self.state[row].get_mut(col) {
                match clicked_cell.cell_state {
                    CellState::Hidden => {
                        clicked_cell.update_state(CellState::Visible);
                    }
                    CellState::Visible => {}
                }
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mines = 50;
    // changing screen size
    let mut board = Board::new(16, 16, 1.0, 2.0, mines);

    let mut mouse_pos: (f32, f32);

    loop {
        clear_background(WHITE);

        // Update
        if is_mouse_button_pressed(MouseButton::Left) {
            mouse_pos = mouse_position();
            board.update(mouse_pos);
        }

        // Draw
        board.draw();

        next_frame().await
    }
}
