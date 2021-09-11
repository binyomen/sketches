use nannou::{draw::Draw, event::Event};

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
    center: f32,
    offset: f32,
    radius: f32,
    height: f32,
}

impl Branch {
    fn new(center: f32, offset: f32) -> Self {
        Branch {
            center,
            offset,
            radius: 10.0,
            height: 0.0,
        }
    }

    fn event(&mut self, event: &Event) {
        match event {
            Event::Update(_) => {
                self.height += 0.5;
                self.radius *= 0.999;
                self.update_offset();
            }
            _ => (),
        }
    }

    fn view(&self, draw: &Draw) {
        const WINDOW_BOTTOM: f32 = -((HEIGHT as f32) / 2.0);
        draw.ellipse()
            .x_y(self.center + self.offset, WINDOW_BOTTOM + self.height)
            .radius(self.radius)
            .rgb(0.3, 0.3, 0.3);
    }

    fn update_offset(&mut self) {
        let direction_multiplier = if self.offset < 0.0 {
            -1.0
        } else if self.offset > 0.0 {
            1.0
        } else {
            0.0
        };

        let shifted_height = (self.height / 100.0) - 4.493;
        let new_offset = (5.0 / shifted_height) * (shifted_height).sin();
        self.offset = new_offset * direction_multiplier;
    }
}
