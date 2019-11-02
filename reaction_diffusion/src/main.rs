// Based on https://www.youtube.com/watch?v=BV9ny785UNc

use nannou::{
    image::{ImageBuffer, RgbaImage},
    prelude::*,
};
use reaction_diffusion::{Grid, DPI, HEIGHT, WIDTH};
use std::cell::RefCell;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    grid: Grid,
    img: RefCell<RgbaImage>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .with_title("")
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .build()
        .unwrap();

    let img: RgbaImage =
        ImageBuffer::new((WIDTH as f32 * DPI) as u32, (HEIGHT as f32 * DPI) as u32);
    Model {
        grid: Grid::new(),
        img: RefCell::new(img),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    app.main_window()
        .set_title(&format!("Reaction Diffusion Algorithm: FPS {}", app.fps()));

    model.grid.update();
}

fn view(app: &App, model: &Model, frame: &Frame) {
    model.grid.view(&mut *model.img.borrow_mut());
    nannou_utils::draw_image(&*model.img.borrow(), DPI, app, frame);
}
