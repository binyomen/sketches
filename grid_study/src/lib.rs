use nannou::{prelude::*, Draw};

mod basic_squares;
mod cubbies;
mod lines;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 500;
pub const CELL_SIZE: f32 = 50.0;

pub enum Mode {
    BasicSquares,
    Lines,
    Cubbies,
}

impl Mode {
    pub fn get_loop_mode(&self) -> LoopMode {
        match self {
            Mode::BasicSquares => LoopMode::loop_once(),
            Mode::Lines => LoopMode::loop_once(),
            Mode::Cubbies => LoopMode::loop_once(),
        }
    }

    pub fn get_square_op(&self) -> fn(Square, &Draw) {
        match self {
            Mode::BasicSquares => basic_squares::draw,
            Mode::Lines => lines::draw,
            Mode::Cubbies => cubbies::draw,
        }
    }
}

impl From<String> for Mode {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_ref() {
            "basicsquares" => Mode::BasicSquares,
            "lines" => Mode::Lines,
            "cubbies" => Mode::Cubbies,
            _ => unreachable!(),
        }
    }
}

pub struct Square {
    pub x: f32,
    pub y: f32,
    pub side: f32,
}

impl Square {
    fn get_sides(&self) -> [Side<f32>; 4] {
        let half_side = self.side / 2.0;
        let x = self.x;
        let y = self.y;

        let top_left = pt2(x - half_side, y + half_side);
        let top_right = pt2(x + half_side, y + half_side);
        let bottom_right = pt2(x + half_side, y - half_side);
        let bottom_left = pt2(x - half_side, y - half_side);

        let top = Side {
            p1: top_left,
            p2: top_right,
        };
        let right = Side {
            p1: top_right,
            p2: bottom_right,
        };
        let bottom = Side {
            p1: bottom_right,
            p2: bottom_left,
        };
        let left = Side {
            p1: bottom_left,
            p2: top_left,
        };

        [top, right, bottom, left]
    }

    fn get_top_y(&self) -> f32 {
        self.y + (self.side / 2.0)
    }

    fn get_right_x(&self) -> f32 {
        self.x + (self.side / 2.0)
    }

    fn get_bottom_y(&self) -> f32 {
        self.y - (self.side / 2.0)
    }

    fn get_left_x(&self) -> f32 {
        self.x - (self.side / 2.0)
    }
}

struct Side<S> {
    p1: Point2<S>,
    p2: Point2<S>,
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

    pub fn draw_for_each(&self, f: fn(Square, &Draw), draw: &Draw) {
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
