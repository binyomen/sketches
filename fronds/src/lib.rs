use nannou::draw::Draw;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 500;

pub struct Frond {
    x: f32,
}

impl Frond {
    pub fn new(x: f32) -> Self {
        Frond { x }
    }

    pub fn view(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.x, -((HEIGHT as f32) / 2.0))
            .radius(100.0)
            .rgb(0.3, 0.3, 0.3);
    }
}
