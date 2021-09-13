use {
    fronds::{Frond, BACKGROUND_COLOR, HEIGHT, WIDTH, WIDTH_1, WIDTH_2, WIDTH_3},
    nannou::{app::App, draw::Draw, event::Update, frame::Frame},
    nannou_imageutil::capture::CaptureHelper,
    rand::{thread_rng, Rng},
    std::{env, fs},
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

const MIN_NUM_FRONDS: usize = 12;
const MAX_NUM_FRONDS: usize = 22;

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
    nannou::app(model)
        .update(update)
        .view(view)
        .exit(exit)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Fronds")
        .size(WIDTH / SIZE_DIVIDEND, HEIGHT / SIZE_DIVIDEND)
        .build()
        .unwrap();

    let mut rng = thread_rng();

    let num_fronds = rng.gen_range(MIN_NUM_FRONDS..MAX_NUM_FRONDS);
    println!("Drawing {} fronds.", num_fronds);

    let mut fronds = Vec::with_capacity(num_fronds);
    for _ in 0..num_fronds {
        let frond_position = rng.gen_range(-(WIDTH as f32) / 2.0..(WIDTH as f32) / 2.0);
        let frond_closeness = rng.gen();
        fronds.push((
            frond_closeness,
            Frond::new(frond_position, frond_closeness, &mut rng),
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

fn update(_app: &App, model: &mut Model, _update: Update) {
    process_update(model);
}

fn process_update(model: &mut Model) {
    // If we've already written out the file, we're done here.
    if model.file_written {
        return;
    }

    model.num_updates += 1;

    let t = 1.0 / 60.0;
    model.t += t;
    if model.t > SECONDS_PER_FROND {
        model.t = 0.0;
        model.fronds.pop();
        println!("{} fronds left.", model.fronds.len());
    } else if let Some((_, frond)) = model.fronds.last_mut() {
        frond.event(model.t);
    } else if model.generation_complete {
        model.file_written = true;
    } else {
        model.generation_complete = true;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // If we've already written out the file, we're done here.
    if model.file_written {
        return;
    }

    let draw = app.draw();
    process_view(model, &draw);

    if env::args().nth(1) == Some("fast".to_owned()) {
        let mut_model = unsafe { (model as *const Model as *mut Model).as_mut() }.unwrap();
        for _ in 0..100 {
            if model.generation_complete {
                break;
            }
            process_update(mut_model);
            process_view(model, &draw);
        }
    }

    model.capture_helper.render_image(app, &draw);
    model.capture_helper.display_in_window(&frame);

    if model.generation_complete && !model.file_written {
        let path = capture_directory(app)
            .join("fronds_3")
            .with_extension("png");
        model.capture_helper.write_to_file(path).unwrap();

        println!("File written.");
    }
}

fn process_view(model: &Model, draw: &Draw) {
    if model.num_updates <= 1 {
        draw.background()
            .rgb(BACKGROUND_COLOR, BACKGROUND_COLOR, BACKGROUND_COLOR);
    }

    if let Some((_, frond)) = model.fronds.last() {
        frond.view(draw);
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
