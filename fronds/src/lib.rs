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
    height: f32,
    relative_offset: f32,
}

impl Branch {
    fn new(frond_center: f32, original_offset: f32) -> Self {
        Branch {
            frond_center,
            original_offset,
            radius: 3.0,
            height: 0.0,
            relative_offset: 0.0,
        }
    }

    fn event(&mut self, event: &Event) {
        match event {
            Event::Update(_) => {
                self.height += 0.5;
                // self.radius *= 0.99;
                self.update_relative_offset();
            }
            _ => (),
        }
    }

    fn view(&self, draw: &Draw) {
        const WINDOW_BOTTOM: f32 = -((HEIGHT as f32) / 2.0);
        draw.ellipse()
            .x_y(
                self.frond_center + self.original_offset + self.relative_offset,
                WINDOW_BOTTOM + self.height,
            )
            .radius(self.radius)
            .rgb(0.3, 0.3, 0.3);
    }

    fn update_relative_offset(&mut self) {
        let direction_multiplier = if self.original_offset < 0.0 {
            -1.0
        } else if self.original_offset > 0.0 {
            1.0
        } else {
            0.0
        };

        // See https://en.wikipedia.org/wiki/Damping#Damped_sine_wave
        let amplitude = self.original_offset.abs() * 10.0;
        const DECAY_RATE: f32 = 0.7;
        let x = self.height * 0.01;
        let function_output = amplitude * E.powf(-DECAY_RATE * x) * (PI * x).cos();
        self.relative_offset = direction_multiplier * -(function_output - amplitude);
    }
}
