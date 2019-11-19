use nannou::{prelude::*, Draw};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 500;
pub const CELL_SIZE: f32 = 50.0;

pub enum Mode {
    BasicSquares,
}

impl Mode {
    pub fn get_loop_mode(&self) -> LoopMode {
        match self {
            Mode::BasicSquares => LoopMode::loop_once(),
        }
    }

    pub fn get_square_op(&self) -> Box<dyn Fn(Square, &Draw)> {
        Box::new(match self {
            Mode::BasicSquares => |sq, draw| {
                draw.rect()
                    .x_y(sq.x, sq.y)
                    .w_h(sq.side, sq.side)
                    .stroke(rgb(1.0, 1.0, 1.0))
                    .no_fill();
            },
        })
    }
}

impl From<String> for Mode {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_ref() {
            "basicsquares" => Mode::BasicSquares,
            _ => unreachable!(),
        }
    }
}

pub struct Square {
    pub x: f32,
    pub y: f32,
    pub side: f32,
}

pub struct Grid {
    width: usize,
    height: usize,
    cell_size: f32,
}

impl Grid {
    pub fn new(cell_size: f32) -> Self {
        Grid {
            width: (WIDTH as f32 / cell_size).round() as usize,
            height: (HEIGHT as f32 / cell_size).round() as usize,
            cell_size: cell_size,
        }
    }

    pub fn draw_for_each(&self, f: Box<dyn Fn(Square, &Draw)>, draw: &Draw) {
        for x in 0..self.width {
            for y in 0..self.height {
                f(
                    Square {
                        x: (x as f32 * self.cell_size) - (WIDTH as f32 / 2.0)
                            + (self.cell_size / 2.0),
                        y: (y as f32 * self.cell_size) - (HEIGHT as f32 / 2.0)
                            + (self.cell_size / 2.0),
                        side: self.cell_size,
                    },
                    draw,
                )
            }
        }
    }
}
