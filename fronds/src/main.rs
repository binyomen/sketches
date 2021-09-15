use {
    fronds::{Plant, BACKGROUND_COLOR, HEIGHT, WIDTH, WIDTH_1, WIDTH_2, WIDTH_3},
    nannou::{app::App, draw::Draw, event::Update, frame::Frame},
    nannou_imageutil::capture::CaptureHelper,
    rand::{thread_rng, Rng},
    std::{env, fs, ops::Range, path::PathBuf},
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

const MIN_NUM_PLANTS: usize = 12;
const MAX_NUM_PLANTS: usize = 22;

const SECONDS_PER_PLANT: f32 = if WIDTH == WIDTH_1 {
    10.0
} else if WIDTH == WIDTH_2 || WIDTH == WIDTH_3 {
    15.0
} else {
    0.0
};

const WIDTH_F32: f32 = WIDTH as f32;
const QUADRANTS: [Range<f32>; 4] = [
    -WIDTH_F32 / 2.0..-WIDTH_F32 / 4.0,
    -WIDTH_F32 / 4.0..0.0,
    0.0..WIDTH_F32 / 4.0,
    WIDTH_F32 / 4.0..WIDTH_F32 / 2.0,
];

const FILE_NAME: &str = "fronds_4";

struct Model {
    num_updates: u64,
    plants: Vec<(f32, Plant)>,
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

    let num_plants = rng.gen_range(MIN_NUM_PLANTS..MAX_NUM_PLANTS);
    println!("Drawing {} plants.", num_plants);

    let mut quadrant_index = 0;

    let mut plants = Vec::with_capacity(num_plants);
    for _ in 0..num_plants {
        let plant_position = rng.gen_range(QUADRANTS[quadrant_index].clone());
        let plant_closeness = rng.gen();
        plants.push((
            plant_closeness,
            Plant::new(plant_position, plant_closeness, &mut rng),
        ));

        quadrant_index = (quadrant_index + 1) % QUADRANTS.len();
    }

    plants.sort_by(|(d1, _), (d2, _)| d2.partial_cmp(d1).unwrap());

    fs::create_dir_all(&capture_directory(app)).unwrap();

    Model {
        num_updates: 0,
        plants,
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
    if model.t > SECONDS_PER_PLANT {
        model.t = 0.0;
        model.plants.pop();
        println!("{} plants left.", model.plants.len());
    } else if let Some((_, plant)) = model.plants.last_mut() {
        plant.event(model.t);
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
        let path = capture_directory(app).join(FILE_NAME).with_extension("png");
        model.capture_helper.write_to_file(path).unwrap();

        println!("File written.");
    }
}

fn process_view(model: &Model, draw: &Draw) {
    if model.num_updates <= 1 {
        draw.background()
            .rgb(BACKGROUND_COLOR, BACKGROUND_COLOR, BACKGROUND_COLOR);
    }

    if let Some((_, plant)) = model.plants.last() {
        plant.view(draw);
    }
}

fn exit(app: &App, mut model: Model) {
    model.capture_helper.close(app).unwrap();
}

fn capture_directory(app: &App) -> PathBuf {
    app.project_path()
        .expect("Could not locate project path.")
        .join("outputs")
}
