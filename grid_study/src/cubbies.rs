use crate::Square;
use nannou::{prelude::*, Draw};

struct InnerCubby<'s> {
    sq: &'s Square,
}

impl<'s> InnerCubby<'s> {
    fn new(sq: &'s Square) -> Self {
        InnerCubby { sq: sq }
    }

    fn draw(&self, draw: &Draw) {
        let [p1, p2] = self.get_points();
        let color = rgba(1.0, 1.0, 1.0, 0.3);

        draw.line()
            .start(p1)
            .end(pt2(self.sq.get_left_x(), self.sq.get_bottom_y()))
            .color(color);
        draw.line()
            .start(p2)
            .end(pt2(self.sq.get_right_x(), self.sq.get_bottom_y()))
            .color(color);

        draw.line()
            .start(p1)
            .end(pt2(p1.x, self.sq.get_top_y()))
            .color(color);
        draw.line()
            .start(p2)
            .end(pt2(p2.x, self.sq.get_top_y()))
            .color(color);

        draw.line().start(p1).end(p2).color(color);
    }

    fn get_points(&self) -> [Point2<f32>; 2] {
        let y = self.sq.y - 9.0;
        let x1 = self.sq.x - 15.0;
        let x2 = self.sq.x + 15.0;

        [pt2(x1, y), pt2(x2, y)]
    }
}

pub fn draw(sq: Square, draw: &Draw) {
    draw.rect()
        .x_y(sq.x, sq.y)
        .w_h(sq.side, sq.side)
        .stroke(rgb(1.0, 1.0, 1.0))
        .stroke_weight(2.0)
        .no_fill();

    let inner = InnerCubby::new(&sq);
    inner.draw(draw);
}
