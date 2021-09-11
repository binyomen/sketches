use {
    fronds::{Frond, HEIGHT, WIDTH},
    nannou::{app::App, event::Event, frame::Frame},
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

    Model {
        num_updates: 0,
        fronds: vec![Frond::new(0.0), Frond::new(400.0), Frond::new(-400.0)],
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
        draw.background().rgb(0.116, 0.116, 0.116);
    }

    for frond in &model.fronds {
        frond.view(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
