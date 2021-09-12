use {
    fronds::{Frond, BACKGROUND_COLOR, HEIGHT, WIDTH},
    nannou::{app::App, event::Event, frame::Frame},
    rand::{thread_rng, Rng},
};

struct Model {
    num_updates: u64,
    fronds: Vec<Frond>,
}

fn main() {
    nannou::app(model).event(event).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Fronds")
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();

    let mut rng = thread_rng();

    let num_fronds = rng.gen_range(5..12);
    let mut fronds = Vec::with_capacity(num_fronds);
    for _ in 0..num_fronds {
        let frond_position = rng.gen_range(-(WIDTH as f32) / 2.0..(WIDTH as f32) / 2.0);
        let frond_distance = rng.gen();
        fronds.push(Frond::new(frond_position, frond_distance, &mut rng));
    }

    Model {
        num_updates: 0,
        fronds,
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    model.num_updates += 1;

    for frond in &mut model.fronds {
        frond.event(&event);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if model.num_updates == 0 {
        draw.background()
            .rgb(BACKGROUND_COLOR, BACKGROUND_COLOR, BACKGROUND_COLOR);
    }

    for frond in &model.fronds {
        frond.view(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
