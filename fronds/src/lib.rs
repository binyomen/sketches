use {
    nannou::{draw::Draw, glam::Vec2, prelude::Point2},
    rand::Rng,
    std::f32::consts::{E, PI},
};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 500;
const WINDOW_BOTTOM: f32 = -((HEIGHT as f32) / 2.0);

pub const BACKGROUND_COLOR: f32 = 0.116;

pub struct Frond {
    branches: Vec<Branch>,
}

impl Frond {
    pub fn new<R: Rng>(x: f32, distance: f32, rng: &mut R) -> Self {
        let max_height = rng.gen_range(100.0..300.0);
        Frond {
            branches: vec![
                Branch::new(x, -5.0, max_height, distance),
                Branch::new(x, -2.5, max_height, distance),
                Branch::new(x, -1.0, max_height, distance),
                Branch::new(x, 1.0, max_height, distance),
                Branch::new(x, 2.5, max_height, distance),
                Branch::new(x, 5.0, max_height, distance),
            ],
        }
    }

    pub fn event(&mut self, t: f32) {
        for branch in &mut self.branches {
            branch.event(t);
        }
    }

    pub fn view(&self, draw: &Draw) {
        for branch in &self.branches {
            branch.view(draw);
        }
    }
}

struct Branch {
    frond_center: f32,
    original_offset: f32,
    max_height: f32,
    weight: f32,
    color: f32,
    height: f32,
    relative_offset: f32,
    prev_point: Option<Point2>,
    t_before_curl: f32,
    curl: Option<Curl>,
}

impl Branch {
    fn new(frond_center: f32, original_offset: f32, max_height: f32, distance: f32) -> Self {
        Branch {
            frond_center,
            original_offset,
            max_height,
            weight: 10.0 * distance,
            color: (distance / 2.0) + BACKGROUND_COLOR,
            height: 0.0,
            relative_offset: 0.0,
            prev_point: None,
            t_before_curl: 0.0,
            curl: None,
        }
    }

    fn event(&mut self, t: f32) {
        self.weight *= 0.999;
        match &mut self.curl {
            Some(curl) => curl.event(t - self.t_before_curl),
            None => self.update_position(t),
        }
    }

    fn view(&self, draw: &Draw) {
        match &self.curl {
            Some(curl) => curl.view(draw),
            None => {
                if let Some(prev_point) = self.prev_point {
                    draw.line()
                        .start(prev_point)
                        .end(Point2::new(
                            self.frond_center + self.original_offset + self.relative_offset,
                            WINDOW_BOTTOM + self.height,
                        ))
                        .weight(self.weight)
                        .rgb(self.color, self.color, self.color);
                }
            }
        }
    }

    fn update_position(&mut self, t: f32) {
        self.prev_point = Some(Point2::new(
            self.frond_center + self.original_offset + self.relative_offset,
            WINDOW_BOTTOM + self.height,
        ));

        self.height += t;
        self.relative_offset = (1.0 / 50.0) * self.original_offset * t.powi(4);

        if self.height > self.max_height {
            self.t_before_curl = t;

            let direction_multiplier = if self.original_offset < 0.0 {
                -1.0
            } else if self.original_offset > 0.0 {
                1.0
            } else {
                0.0
            };
            self.curl = Some(Curl {
                amplitude: self.original_offset.abs() * 10.0,
                weight: self.weight,
                color: self.color,
                direction_multiplier,
                starting_point: Vec2::new(
                    self.frond_center + self.original_offset + self.relative_offset,
                    WINDOW_BOTTOM + self.height,
                ),
                relative_position: Vec2::new(0.0, 0.0),
                prev_point: self.prev_point,
            });
        }
    }
}

struct Curl {
    amplitude: f32,
    weight: f32,
    color: f32,
    direction_multiplier: f32,
    starting_point: Vec2,
    relative_position: Vec2,
    prev_point: Option<Point2>,
}

impl Curl {
    fn event(&mut self, t: f32) {
        self.prev_point = Some(Point2::new(
            self.starting_point.x + self.relative_position.x,
            self.starting_point.y + self.relative_position.y,
        ));

        self.weight *= 0.99;

        // See https://en.wikipedia.org/wiki/Damping#Damped_sine_wave and
        // https://mathworld.wolfram.com/LogarithmicSpiral.html
        const DECAY_RATE: f32 = 0.7;
        const ANGULAR_FREQUENCY: f32 = PI;
        const PHASE_ANGLE: f32 = 0.0;
        let function_output = damped_function_cos(
            t,
            self.amplitude,
            DECAY_RATE,
            ANGULAR_FREQUENCY,
            PHASE_ANGLE,
        );

        let x = self.direction_multiplier * -(function_output - self.amplitude);
        let y = damped_function_sin(
            t,
            self.amplitude,
            DECAY_RATE,
            ANGULAR_FREQUENCY,
            PHASE_ANGLE,
        );

        self.relative_position = Vec2::new(x, y);
    }

    fn view(&self, draw: &Draw) {
        if let Some(prev_point) = self.prev_point {
            draw.line()
                .start(prev_point)
                .end(Point2::new(
                    self.starting_point.x + self.relative_position.x,
                    self.starting_point.y + self.relative_position.y,
                ))
                .weight(self.weight)
                .rgb(self.color, self.color, self.color);
        }
    }
}

fn damped_function_cos(
    t: f32,
    amplitude: f32,
    decay_rate: f32,
    angular_frequency: f32,
    phase_angle: f32,
) -> f32 {
    damped_function(
        t,
        amplitude,
        decay_rate,
        angular_frequency,
        phase_angle,
        f32::cos,
    )
}

fn damped_function_sin(
    t: f32,
    amplitude: f32,
    decay_rate: f32,
    angular_frequency: f32,
    phase_angle: f32,
) -> f32 {
    damped_function(
        t,
        amplitude,
        decay_rate,
        angular_frequency,
        phase_angle,
        f32::sin,
    )
}

fn damped_function<F: FnOnce(f32) -> f32>(
    t: f32,
    amplitude: f32,
    decay_rate: f32,
    angular_frequency: f32,
    phase_angle: f32,
    f: F,
) -> f32 {
    amplitude * E.powf(-decay_rate * t) * f(angular_frequency * t - phase_angle)
}
