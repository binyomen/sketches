use {
    nannou::{
        color::{rgb, rgb::Rgb},
        draw::Draw,
        geom::Vec2,
    },
    rand::{thread_rng, Rng},
};

pub const WIDTH_1: u32 = 600;
pub const HEIGHT_1: u32 = 600;
pub const WIDTH_2: u32 = 3600;
pub const HEIGHT_2: u32 = 3600;

pub const WIDTH: u32 = WIDTH_2;
pub const HEIGHT: u32 = HEIGHT_2;

const HALF_WIDTH: u32 = WIDTH / 2;
const HALF_HEIGHT: u32 = HEIGHT / 2;

const NUM_CELLS_X: u32 = 10;
const NUM_CELLS_Y: u32 = 10;
const CELL_WIDTH: f32 = (WIDTH as f32) / (NUM_CELLS_X as f32);
const CELL_HEIGHT: f32 = (HEIGHT as f32) / (NUM_CELLS_Y as f32);
const QUARTER_CELL_WIDTH: f32 = CELL_WIDTH / 4.0;
const QUARTER_CELL_HEIGHT: f32 = CELL_HEIGHT / 4.0;

const POINT_RADIUS: f32 = (WIDTH as f32) / 120.0;
const LINE_WEIGHT: f32 = (WIDTH as f32) / 300.0;

#[derive(Default)]
pub struct Grid {
    points: Vec<Vec<Point>>,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let points = generate_points(&mut rng);
        Grid {
            cells: generate_cells(&points, &mut rng),
            points,
        }
    }

    pub fn view(&self, draw: &Draw) {
        for row in &self.cells {
            for cell in row {
                cell.view(draw);
            }
        }

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

        let mut point_row = Vec::with_capacity((NUM_CELLS_X + 1) as usize);
        for i in 0..point_row.capacity() {
            let x = (i as f32) * CELL_WIDTH - (HALF_WIDTH as f32);

            let is_on_edge =
                i == 0 || i == point_row.capacity() - 1 || j == 0 || j == point_rows.capacity() - 1;

            let (skewed_x, skewed_y) = if is_on_edge {
                (x, y)
            } else {
                (
                    x + rng.gen_range(-QUARTER_CELL_WIDTH..QUARTER_CELL_WIDTH),
                    y + rng.gen_range(-QUARTER_CELL_HEIGHT..QUARTER_CELL_HEIGHT),
                )
            };

            point_row.push(Point {
                x: skewed_x,
                y: skewed_y,
            });
        }

        debug_assert_eq!(point_row.len(), (NUM_CELLS_X + 1) as usize);
        point_rows.push(point_row);
    }

    debug_assert_eq!(point_rows.len(), (NUM_CELLS_Y + 1) as usize);
    point_rows
}

fn generate_cells(points: &[Vec<Point>], rng: &mut impl Rng) -> Vec<Vec<Cell>> {
    let mut cell_rows = Vec::with_capacity(NUM_CELLS_Y as usize);
    for j in 0..cell_rows.capacity() {
        let mut cell_row = Vec::with_capacity(NUM_CELLS_X as usize);
        for i in 0..cell_row.capacity() {
            cell_row.push(Cell {
                top_left: points[j][i],
                top_right: points[j][i + 1],
                bottom_right: points[j + 1][i + 1],
                bottom_left: points[j + 1][i],
                color: create_cell_color(rng),
            });
        }

        debug_assert_eq!(cell_row.len(), NUM_CELLS_X as usize);
        cell_rows.push(cell_row);
    }

    debug_assert_eq!(cell_rows.len(), NUM_CELLS_Y as usize);
    cell_rows
}

fn create_cell_color(rng: &mut impl Rng) -> Rgb {
    rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn view(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(POINT_RADIUS)
            .rgb(1.0, 1.0, 1.0);
    }
}

impl From<Point> for Vec2 {
    fn from(point: Point) -> Self {
        Vec2::new(point.x, point.y)
    }
}

struct Cell {
    top_left: Point,
    top_right: Point,
    bottom_right: Point,
    bottom_left: Point,
    color: Rgb,
}

impl Cell {
    fn view(&self, draw: &Draw) {
        draw.polygon()
            .color(self.color)
            .stroke_color(rgb(1.0, 1.0, 1.0))
            .stroke_weight(LINE_WEIGHT)
            .points([
                self.top_left,
                self.top_right,
                self.bottom_right,
                self.bottom_left,
            ]);
    }
}
