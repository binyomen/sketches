use nannou::image::{self, RgbaImage};
use rand::Rng;
use std::cell::RefCell;

pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
pub const DPI: f32 = 2.0;
const GRID_W: usize = (WIDTH as f32 * DPI / SCALE as f32) as usize;
const GRID_H: usize = (HEIGHT as f32 * DPI / SCALE as f32) as usize;
const SCALE: usize = 1;

const NUM_DROPS: usize = 2000;
const DROP_SIZE: usize = 10;

const DIFFUSION_RATE_A: f32 = 1.0;
const DIFFUSION_RATE_B: f32 = 0.5;
const FEED_RATE: f32 = 0.055;
const KILL_RATE: f32 = 0.062;
const TIME_DELTA: f32 = 1.0;

const CENTER_WEIGHT: f32 = -1.0;
const ADJACENT_WEIGHT: f32 = 0.2;
const DIAGONAL_WEIGHT: f32 = 0.05;

pub struct Grid {
    cols: Vec<Vec<Cell>>,
    next_cols: RefCell<Vec<Vec<Cell>>>,
}

impl Grid {
    pub fn new() -> Self {
        let mut cols = Vec::with_capacity(GRID_W);
        for x in 0..GRID_W {
            let mut col = Vec::with_capacity(GRID_H);
            for y in 0..GRID_H {
                col.push(Cell::new(x, y));
            }
            cols.push(col);
        }

        // seed the chemical solution
        let mut rng = rand::thread_rng();
        for _ in 0..NUM_DROPS {
            let x = rng.gen_range(0, GRID_W - DROP_SIZE);
            let y = rng.gen_range(0, GRID_H - DROP_SIZE);
            for i in 0..DROP_SIZE {
                for j in 0..DROP_SIZE {
                    cols[x + i][y + j].b = 1.0;
                }
            }
        }

        Grid {
            cols: cols.clone(),
            next_cols: RefCell::new(cols),
        }
    }

    fn get_value(&self, x: usize, y: usize, get: &(dyn Fn(&Cell) -> f32)) -> f32 {
        get(&self.cols[x][y])
    }

    pub fn update(&mut self) {
        for col in &mut *self.next_cols.borrow_mut() {
            for cell in col {
                cell.update(self);
            }
        }

        std::mem::swap(&mut self.cols, &mut *self.next_cols.borrow_mut());
    }

    pub fn view(&self, mut img: &mut RgbaImage) {
        for col in &self.cols {
            for cell in col {
                cell.view(&mut img);
            }
        }
    }
}

#[derive(Clone)]
struct Cell {
    a: f32,
    b: f32,

    x: usize,
    y: usize,
}

impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Cell {
            a: 1.0,
            b: 0.0,

            x: x,
            y: y,
        }
    }

    fn get_adjacent(&self, grid: &Grid, get: &(dyn Fn(&Cell) -> f32)) -> Vec<f32> {
        let x = self.x;
        let y = self.y;

        let mut neighbors = vec![];

        if y + 1 < GRID_H {
            neighbors.push(grid.get_value(x, y + 1, &get));
        }
        if x + 1 < GRID_W {
            neighbors.push(grid.get_value(x + 1, y, &get));
        }
        if y > 0 {
            neighbors.push(grid.get_value(x, y - 1, &get));
        }
        if x > 0 {
            neighbors.push(grid.get_value(x - 1, y, &get));
        }

        neighbors
    }

    fn get_diagonal(&self, grid: &Grid, get: &(dyn Fn(&Cell) -> f32)) -> Vec<f32> {
        let x = self.x;
        let y = self.y;

        let mut neighbors = vec![];

        if x + 1 < GRID_W && y + 1 < GRID_H {
            neighbors.push(grid.get_value(x + 1, y + 1, &get));
        }
        if x + 1 < GRID_W && y > 0 {
            neighbors.push(grid.get_value(x + 1, y - 1, &get));
        }
        if x > 0 && y > 0 {
            neighbors.push(grid.get_value(x - 1, y - 1, &get));
        }
        if x > 0 && y + 1 < GRID_H {
            neighbors.push(grid.get_value(x - 1, y + 1, &get));
        }

        neighbors
    }

    fn laplace(&self, grid: &Grid, get: impl Fn(&Cell) -> f32) -> f32 {
        let mut sum = 0.0;

        sum += grid.get_value(self.x, self.y, &get) * CENTER_WEIGHT;

        for neighbor in self.get_adjacent(grid, &get) {
            sum += neighbor * ADJACENT_WEIGHT;
        }

        for neighbor in self.get_diagonal(grid, &get) {
            sum += neighbor * DIAGONAL_WEIGHT;
        }

        sum
    }

    fn update(&mut self, grid: &Grid) {
        let a = grid.get_value(self.x, self.y, &|cell| cell.a);
        let b = grid.get_value(self.x, self.y, &|cell| cell.b);
        const DA: f32 = DIFFUSION_RATE_A;
        const DB: f32 = DIFFUSION_RATE_B;
        let la = self.laplace(grid, |cell| cell.a);
        let lb = self.laplace(grid, |cell| cell.b);
        const F: f32 = FEED_RATE;
        const K: f32 = KILL_RATE;
        const T: f32 = TIME_DELTA;

        self.a = a + ((DA * la) - (a * b * b) + (F * (1.0 - a))) * T;
        self.b = b + ((DB * lb) + (a * b * b) - ((K + F) * b)) * T;
    }

    fn view(&self, img: &mut RgbaImage) {
        let x = self.x * SCALE;
        let y = self.y * SCALE;

        let difference = self.a - self.b;
        let color = if difference < 0.0 {
            0
        } else {
            (difference * 255.0) as u8
        };
        let pixel = image::Rgba([color, color, color, 255]);

        for i in 0..SCALE {
            for j in 0..SCALE {
                img.put_pixel((x + i) as u32, (y + j) as u32, pixel);
            }
        }
    }
}
