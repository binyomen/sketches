use crate::Square;
use nannou::{prelude::*, Draw};

pub fn draw(sq: Square, draw: &Draw) {
    draw.rect()
        .x_y(sq.x, sq.y)
        .w_h(sq.side, sq.side)
        .stroke(rgb(1.0, 1.0, 1.0))
        .no_fill();
}
