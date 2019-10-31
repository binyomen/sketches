// Based on https://www.youtube.com/watch?v=BV9ny785UNc

use nannou::prelude::*;

use reaction_diffusion::{Grid, HEIGHT, WIDTH};

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    grid: Grid,
}

fn model(app: &App) -> Model {
    app.new_window()
        .with_title("")
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .build()
        .unwrap();

    Model { grid: Grid::new() }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    app.main_window()
        .set_title(&format!("Reaction Diffusion Algorithm: FPS {}", app.fps()));

    model.grid.update();
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    model.grid.view(&draw);

    draw.to_frame(&app, &frame).unwrap();
}
