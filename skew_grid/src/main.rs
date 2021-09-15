use {
    nannou::{
        app::{App, LoopMode},
        frame::Frame,
    },
    nannou_imageutil::capture::CaptureHelper,
    skew_grid::{Grid, HEIGHT, WIDTH, WIDTH_1},
};

const SIZE_DIVIDEND: u32 = if WIDTH == WIDTH_1 { 1 } else { 0 };

struct Model {
    capture_helper: CaptureHelper,
}

fn main() {
    nannou::app(model).view(view).exit(exit).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Skew Grid")
        .size(WIDTH / SIZE_DIVIDEND, HEIGHT / SIZE_DIVIDEND)
        .build()
        .unwrap();
    app.set_loop_mode(LoopMode::NTimes {
        number_of_updates: 1,
    });

    Model {
        capture_helper: CaptureHelper::from_main_window(app, [WIDTH, HEIGHT]),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let grid = Grid::new();
    grid.view(&draw);

    model.capture_helper.render_image(app, &draw);
    model.capture_helper.display_in_window(&frame);
}

fn exit(app: &App, mut model: Model) {
    model.capture_helper.close(app).unwrap();
}
