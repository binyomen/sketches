use {
    fronds::{Frond, BACKGROUND_COLOR, HEIGHT, WIDTH},
    nannou::{app::App, event::Event, frame::Frame},
    rand::{thread_rng, Rng},
};

struct Model {
    num_events: u64,
    fronds: Vec<(f32, Frond)>,
    t: f32,
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
        fronds.push((
            frond_distance,
            Frond::new(frond_position, frond_distance, &mut rng),
        ));
    }

    fronds.sort_by(|(d1, _), (d2, _)| d2.partial_cmp(d1).unwrap());

    Model {
        num_events: 0,
        fronds,
        t: 0.0,
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    model.num_events += 1;

    if let Event::Update(update) = event {
        let t = update.since_last.as_secs_f32();
        model.t += t;
        if model.t > 7.0 {
            model.t = 0.0;
            model.fronds.pop();
        } else if let Some((_, frond)) = model.fronds.last_mut() {
            frond.event(model.t);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if model.num_events == 0 {
        draw.background()
            .rgb(BACKGROUND_COLOR, BACKGROUND_COLOR, BACKGROUND_COLOR);
    }

    if let Some((_, frond)) = model.fronds.last() {
        frond.view(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
