use {
    nannou::{draw::Draw, event::Event},
    std::f32::consts::{E, PI},
};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 500;

pub struct Frond {
    branches: Vec<Branch>,
}

impl Frond {
    pub fn new(x: f32) -> Self {
        Frond {
            branches: vec![
                Branch::new(x, x - 5.0),
                Branch::new(x, x),
                Branch::new(x, x + 5.0),
            ],
        }
    }

    pub fn event(&mut self, event: &Event) {
        for branch in &mut self.branches {
            branch.event(event);
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
    radius: f32,
    x: f32,
    y: f32,
}

impl Branch {
    fn new(frond_center: f32, original_offset: f32) -> Self {
        Branch {
            frond_center,
            original_offset,
            radius: 3.0,
            x: 0.0,
            y: 0.0,
        }
    }

    fn event(&mut self, event: &Event) {
        match event {
            Event::Update(update) => {
                self.radius *= 0.99;
                self.update_position(update.since_start.as_secs_f32());
            }
            _ => (),
        }
    }

    fn view(&self, draw: &Draw) {
        const WINDOW_BOTTOM: f32 = -((HEIGHT as f32) / 2.0);
        draw.ellipse()
            .x_y(
                self.frond_center + self.original_offset + self.x,
                WINDOW_BOTTOM + self.y,
            )
            .radius(self.radius)
            .rgb(0.3, 0.3, 0.3);
    }

    fn update_position(&mut self, t: f32) {
        let direction_multiplier = if self.original_offset < 0.0 {
            -1.0
        } else if self.original_offset > 0.0 {
            1.0
        } else {
            0.0
        };

        // See https://en.wikipedia.org/wiki/Damping#Damped_sine_wave and
        // https://mathworld.wolfram.com/LogarithmicSpiral.html
        let amplitude = self.original_offset.abs() * 10.0;
        const DECAY_RATE: f32 = 0.7;
        const ANGULAR_FREQUENCY: f32 = PI;
        const PHASE_ANGLE: f32 = 0.0;
        let function_output =
            damped_function_cos(t, amplitude, DECAY_RATE, ANGULAR_FREQUENCY, PHASE_ANGLE);
        self.x = direction_multiplier * -(function_output - amplitude);

        self.y = damped_function_sin(t, amplitude, DECAY_RATE, ANGULAR_FREQUENCY, PHASE_ANGLE);
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
