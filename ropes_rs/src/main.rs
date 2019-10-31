use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

use ropes::{Rope, HEIGHT, NUM_ROPES, WIDTH};

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    num_updates: u32,
    ropes: Vec<Rope>,

    noise: Box<dyn NoiseFn<[f64; 3]>>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .with_title("Ropes")
        .with_dimensions(WIDTH, HEIGHT)
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();
    let mut noise: Box<dyn NoiseFn<[f64; 3]>> = Box::new(Perlin::new());

    let mut ropes = vec![];
    for i in 0..NUM_ROPES {
        // evenly space the ropes
        let x = (i as f32 + 0.5) * (WIDTH as f32 / NUM_ROPES as f32) - (WIDTH as f32 / 2.0);
        ropes.push(Rope::new(
            &mut rng,
            &mut noise,
            pt2(x, -(HEIGHT as f32 / 2.0)),
        ));
    }

    Model {
        num_updates: 0,
        ropes: ropes,
        noise: noise,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.num_updates += 1;

    for rope in &mut model.ropes {
        rope.update(&mut model.noise);
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    if model.num_updates <= 1 {
        draw.background().rgb(0.116, 0.116, 0.116);
    }

    for rope in &model.ropes {
        rope.view(&draw);
    }

    draw.to_frame(&app, &frame).unwrap();
}
