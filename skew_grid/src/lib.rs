use {
    nannou::draw::Draw,
    rand::{thread_rng, Rng},
};

pub const WIDTH_1: u32 = 600;
const HEIGHT_1: u32 = 600;

pub const WIDTH: u32 = WIDTH_1;
pub const HEIGHT: u32 = HEIGHT_1;

const HALF_WIDTH: u32 = WIDTH / 2;
const HALF_HEIGHT: u32 = HEIGHT / 2;

const NUM_CELLS_X: u32 = 10;
const NUM_CELLS_Y: u32 = 10;
const CELL_WIDTH: f32 = (WIDTH as f32) / (NUM_CELLS_X as f32);
const CELL_HEIGHT: f32 = (HEIGHT as f32) / (NUM_CELLS_Y as f32);
const QUARTER_CELL_WIDTH: f32 = CELL_WIDTH / 4.0;
const QUARTER_CELL_HEIGHT: f32 = CELL_HEIGHT / 4.0;

#[derive(Default)]
pub struct Grid {
    points: Vec<Vec<Point>>,
}

impl Grid {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        Grid {
            points: generate_points(&mut rng),
        }
    }

    pub fn view(&self, draw: &Draw) {
        for row in &self.points {
            for point in row {
                point.view(draw);
            }
        }
    }
}

fn generate_points(rng: &mut impl Rng) -> Vec<Vec<Point>> {
    let mut point_rows = Vec::with_capacity((NUM_CELLS_Y + 1) as usize);
    for j in 0..point_rows.capacity() {
        let y = (j as f32) * CELL_HEIGHT - (HALF_HEIGHT as f32);

        let mut point_cols = Vec::with_capacity((NUM_CELLS_X + 1) as usize);
        for i in 0..point_cols.capacity() {
            let x = (i as f32) * CELL_WIDTH - (HALF_WIDTH as f32);

            let is_on_edge = i == 0
                || i == point_cols.capacity() - 1
                || j == 0
                || j == point_rows.capacity() - 1;

            let (skewed_x, skewed_y) = if is_on_edge {
                (x, y)
            } else {
                (
                    x + rng.gen_range(-QUARTER_CELL_WIDTH..QUARTER_CELL_WIDTH),
                    y + rng.gen_range(-QUARTER_CELL_HEIGHT..QUARTER_CELL_HEIGHT),
                )
            };

            point_cols.push(Point {
                x: skewed_x,
                y: skewed_y,
            });
        }

        point_rows.push(point_cols);
    }

    point_rows
}

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn view(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(3.0)
            .rgb(1.0, 1.0, 1.0);
    }
}
