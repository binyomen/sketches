use nannou::{draw::Draw, prelude::*};

use std::cell::RefCell;

pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 100;
const SCALE: usize = 1;

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
        let mut cols = Vec::with_capacity(WIDTH / SCALE);
        for x in 0..(WIDTH / SCALE) {
            let mut col = Vec::with_capacity(HEIGHT / SCALE);
            for y in 0..(HEIGHT / SCALE) {
                col.push(Cell::new(x, y));
            }
            cols.push(col);
        }

        for x in 0..10 {
            for y in 0..10 {
                cols[(WIDTH / SCALE) / 2 + x - 5][(HEIGHT / SCALE) / 2 + y - 5].b = 1.0;
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

        self.cols = self.next_cols.borrow().clone();
    }

    pub fn view(&self, draw: &Draw) {
        for col in &self.cols {
            for cell in col {
                cell.view(draw);
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
        const H: usize = HEIGHT / SCALE;
        const W: usize = WIDTH / SCALE;
        let x = self.x;
        let y = self.y;

        let mut neighbors = vec![];

        if y + 1 < H {
            neighbors.push(grid.get_value(x, y + 1, &get));
        }
        if x + 1 < W {
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
        const H: usize = HEIGHT / SCALE;
        const W: usize = WIDTH / SCALE;
        let x = self.x;
        let y = self.y;

        let mut neighbors = vec![];

        if x + 1 < W && y + 1 < H {
            neighbors.push(grid.get_value(x + 1, y + 1, &get));
        }
        if x + 1 < W && y > 0 {
            neighbors.push(grid.get_value(x + 1, y - 1, &get));
        }
        if x > 0 && y > 0 {
            neighbors.push(grid.get_value(x - 1, y - 1, &get));
        }
        if x > 0 && y + 1 < H {
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

    fn view(&self, draw: &Draw) {
        let screen_x = (self.x * SCALE) as f32 - (WIDTH as f32 / 2.0);
        let screen_y = (self.y * SCALE) as f32 - (HEIGHT as f32 / 2.0);

        let c = self.a - self.b;

        let color = rgb(c, c, c);

        draw.rect()
            .x_y(screen_x, screen_y)
            .w_h(SCALE as f32, SCALE as f32)
            .color(color);
    }
}
