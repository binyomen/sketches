// Based on https://www.youtube.com/watch?v=vLlbEZt-3j0

use nannou::{
    color::FromColor,
    image::{self, ImageBuffer, RgbaImage},
    prelude::*,
};

const SCALE: u32 = 150;
const WIDTH: u32 = 4 * SCALE;
const HEIGHT: u32 = 4 * SCALE;

fn main() {
    nannou::app(model).view(view).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .with_title("De Jong IFS")
        .with_dimensions(WIDTH, HEIGHT)
        .build()
        .unwrap();
    app.set_loop_mode(LoopMode::loop_once());

    Model {}
}

fn view(app: &App, _model: &Model, frame: &Frame) {
    let dpi = 2.0;

    let mut img: RgbaImage = ImageBuffer::from_pixel(
        (WIDTH as f32 * dpi + 1.0) as u32,
        (HEIGHT as f32 * dpi + 1.0) as u32,
        image::Rgba([30, 30, 30, 255]),
    );

    let mut current_point = pt2(0.0, 0.0);
    for i in 0..9999999 {
        let color = hsv(
            ((i % 100) as f32 + 80.0) / 255.0,
            100.0 / 255.0,
            150.0 / 255.0,
        );
        let color = Rgb::from_hsv(color);

        let x =
            ((current_point.x * dpi * SCALE as f32) + (WIDTH as f32 * dpi / 2.0)).round() as u32;
        let y =
            ((current_point.y * dpi * SCALE as f32) + (HEIGHT as f32 * dpi / 2.0)).round() as u32;

        let r = (255.0 * color.red) as u8;
        let g = (255.0 * color.green) as u8;
        let b = (255.0 * color.blue) as u8;
        let pixel = image::Rgba([r, g, b, 255]);

        img.put_pixel(x, y, pixel);

        current_point = ifs(current_point);
    }

    nannou_utils::draw_image(&img, dpi, &app, &frame);
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
