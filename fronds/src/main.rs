use {
    fronds::{Frond, HEIGHT, WIDTH},
    nannou::{app::App, event::Event, frame::Frame},
};

struct Model {
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
        fronds: vec![Frond::new(0.0)],
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.116, 0.116, 0.116);

    for frond in &model.fronds {
        frond.view(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
