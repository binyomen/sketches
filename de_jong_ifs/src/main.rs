// Based on https://www.youtube.com/watch?v=vLlbEZt-3j0

use nannou::prelude::*;

const SCALE: u32 = 100;

fn main() {
    nannou::app(model).view(view).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .with_title("De Jong IFS")
        .with_dimensions(4 * SCALE, 4 * SCALE)
        .build()
        .unwrap();
    app.set_loop_mode(LoopMode::loop_once());

    Model {}
}

fn view(app: &App, _model: &Model, frame: &Frame) {
    let draw = app.draw();

    const BACKGROUND_COLOR: f32 = 30.0 / 255.0;
    draw.background()
        .rgb(BACKGROUND_COLOR, BACKGROUND_COLOR, BACKGROUND_COLOR);

    let mut current_point = pt2(0.0, 0.0);
    for i in 0..99999 {
        let color = hsv(
            ((i % 100) as f32 + 80.0) / 255.0,
            100.0 / 255.0,
            150.0 / 255.0,
        );
        draw.ellipse()
            .x_y(
                current_point.x * SCALE as f32,
                current_point.y * SCALE as f32,
            )
            .radius(0.5)
            .color(color);

        current_point = ifs(current_point);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn ifs(p: Vector2) -> Vector2 {
    const A: f32 = 0.97;
    const B: f32 = -1.9;
    const C: f32 = 1.38;
    const D: f32 = -1.5;

    let x = (A * p.y).sin() - (B * p.x).cos();
    let y = (C * p.x).sin() - (D * p.y).cos();
    return pt2(x, y);
}
