use {
    fronds::{Frond, BACKGROUND_COLOR, HEIGHT, WIDTH, WIDTH_1, WIDTH_2, WIDTH_3},
    nannou::{app::App, event::Event, frame::Frame},
    nannou_imageutil::capture::CaptureHelper,
    rand::{thread_rng, Rng},
    std::fs,
};

const SIZE_DIVIDEND: u32 = if WIDTH == WIDTH_1 {
    1
} else if WIDTH == WIDTH_2 {
    3
} else if WIDTH == WIDTH_3 {
    6
} else {
    0
};
const SECONDS_PER_FROND: f32 = if WIDTH == WIDTH_1 {
    10.0
} else if WIDTH == WIDTH_2 || WIDTH == WIDTH_3 {
    15.0
} else {
    0.0
};

struct Model {
    num_updates: u64,
    fronds: Vec<(f32, Frond)>,
    t: f32,
    generation_complete: bool,
    file_written: bool,
    capture_helper: CaptureHelper,
}

fn main() {
    nannou::app(model).event(event).view(view).exit(exit).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Fronds")
        .size(WIDTH / SIZE_DIVIDEND, HEIGHT / SIZE_DIVIDEND)
        .build()
        .unwrap();

    let mut rng = thread_rng();

    let num_fronds = rng.gen_range(12..22);
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

    fs::create_dir_all(&capture_directory(app)).unwrap();

    Model {
        num_updates: 0,
        fronds,
        t: 0.0,
        generation_complete: false,
        file_written: false,
        capture_helper: CaptureHelper::from_main_window(app, [WIDTH, HEIGHT]),
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::Update(_) = event {
        model.num_updates += 1;

        let t = 1.0 / 60.0;
        model.t += t;
        if model.t > SECONDS_PER_FROND {
            model.t = 0.0;
            model.fronds.pop();
        } else if let Some((_, frond)) = model.fronds.last_mut() {
            frond.event(model.t);
        } else if model.generation_complete {
            model.file_written = true;
        } else {
            model.generation_complete = true;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // If we've already written out the file, we're done here.
    if model.file_written {
        return;
    }

    let draw = app.draw();

    if model.num_updates <= 1 {
        draw.background()
            .rgb(BACKGROUND_COLOR, BACKGROUND_COLOR, BACKGROUND_COLOR);
    }

    if let Some((_, frond)) = model.fronds.last() {
        frond.view(&draw);
    }

    model.capture_helper.render_image(app, &draw);
    model.capture_helper.display_in_window(&frame);

    if model.generation_complete && !model.file_written {
        let path = capture_directory(app)
            .join("fronds_2")
            .with_extension("png");
        model.capture_helper.write_to_file(path).unwrap();
    }
}

fn exit(app: &App, mut model: Model) {
    model.capture_helper.close(app).unwrap();
}

fn capture_directory(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("Could not locate project path.")
        .join("outputs")
}
