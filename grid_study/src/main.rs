use grid_study::{Grid, Mode, CELL_SIZE, HEIGHT, WIDTH};
use lazy_static::lazy_static;
use nannou::prelude::*;

lazy_static! {
    static ref MODE: Mode = Mode::from(std::env::args().nth(1).unwrap());
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    grid: Grid,
}

fn model(app: &App) -> Model {
    app.new_window()
        .with_title("Grid Study")
        .with_dimensions(WIDTH, HEIGHT)
        .build()
        .unwrap();
    app.set_loop_mode(MODE.get_loop_mode());

    Model {
        grid: Grid::new(CELL_SIZE),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().rgb(0.116, 0.116, 0.116);

    model.grid.draw_for_each(MODE.get_square_op(), &draw);

    draw.to_frame(&app, &frame).unwrap();
}
