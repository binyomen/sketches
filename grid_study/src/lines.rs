use crate::Square;
use nannou::{prelude::*, Draw};
use rand::Rng;

const LINE_DIST: f32 = 12.0;

struct Line {
    p: Point2<f32>,
    v1: Vector2,
    v2: Vector2,
}

impl Line {
    fn draw(&self, draw: &Draw) {
        Line::draw_vec(self.p, self.v1, draw);
        Line::draw_vec(self.p, self.v2, draw);

        // for debugging
        // draw.ellipse()
        //     .x_y(self.p.x, self.p.y)
        //     .radius(3.0)
        //     .rgb(1.0, 0.0, 0.0);
    }

    fn draw_vec(p: Point2<f32>, v: Vector2, draw: &Draw) {
        draw.line()
            .start(pt2(p.x, p.y))
            .end(pt2(p.x + v.x, p.y + v.y));
    }
}

pub fn draw(sq: Square, draw: &Draw) {
    let angle = rand::thread_rng().gen_range(0.0, PI);

    let lines = gen_lines(angle, &sq, draw);

    for line in lines {
        line.draw(draw);
    }

    // for debugging
    // draw.rect()
    //     .x_y(sq.x, sq.y)
    //     .w_h(sq.side, sq.side)
    //     .stroke(rgb(0.0, 1.0, 0.0))
    //     .no_fill();
}

fn vec_from_angle(angle: f32) -> Vector2 {
    vec2(angle.cos(), angle.sin()).normalize()
}

fn inside_square(p: Point2<f32>, sq: &Square) -> bool {
    p.y < sq.get_top_y()
        && p.x < sq.get_right_x()
        && p.y > sq.get_bottom_y()
        && p.x > sq.get_left_x()
}

fn gen_lines(angle: f32, sq: &Square, draw: &Draw) -> Vec<Line> {
    let mut lines = vec![];

    let perpendicular_angle = angle + PI / 2.0;
    let perp = vec_from_angle(perpendicular_angle);

    lines.append(&mut gen_lines_in_direction(angle, sq, perp, draw));
    lines.append(&mut gen_lines_in_direction(angle, sq, -perp, draw));

    lines
}

fn gen_lines_in_direction(angle: f32, sq: &Square, direction: Vector2, _draw: &Draw) -> Vec<Line> {
    let mut lines = vec![];

    let mut p = pt2(sq.x, sq.y);
    while inside_square(p, sq) {
        // for debugging
        // draw.ellipse().x_y(p.x, p.y).radius(3.0).rgb(1.0, 0.0, 1.0);

        let line = gen_line(p, angle, sq);

        match line {
            None => (),
            Some(line) => lines.push(line),
        }

        p = pt2(
            p.x + (direction.x * LINE_DIST),
            p.y + (direction.y * LINE_DIST),
        );
    }

    lines
}

fn gen_line(p: Point2<f32>, angle: f32, sq: &Square) -> Option<Line> {
    // Generate a vector that
    let v1 = vec_from_angle(angle);
    let v1 = confine_to_square(p, v1, sq);
    let v1 = match v1 {
        None => return None,
        Some(v) => v,
    };

    let reverse_angle = angle + PI;
    let v2 = vec_from_angle(reverse_angle);
    let v2 = confine_to_square(p, v2, sq);
    let v2 = match v2 {
        None => return None,
        Some(v) => v,
    };

    Some(Line {
        p: p,
        v1: v1,
        v2: v2,
    })
}

fn confine_to_square(p: Point2<f32>, v: Vector2, sq: &Square) -> Option<Vector2> {
    // Using equation defined at http://paulbourke.net/geometry/pointlineplane/

    let mut mag = None;

    for side in sq.get_sides().iter() {
        let x1 = p.x;
        let y1 = p.y;
        let x2 = p.x + v.x;
        let y2 = p.y + v.y;

        let x3 = side.p1.x;
        let y3 = side.p1.y;
        let x4 = side.p2.x;
        let y4 = side.p2.y;

        let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);

        // If the lines aren't equivalent.
        if denominator != 0.0 {
            let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;
            let ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denominator;

            // If the intersection occurs in front of the ray.
            if ua > 0.0 && ub > 0.0 {
                let x = x1 + ua * (x2 - x1);
                let y = y1 + ua * (y2 - y1);

                let a = x - x1;
                let b = y - y1;
                let curr_mag = (a * a + b * b).sqrt();

                match mag {
                    None => mag = Some(curr_mag),
                    Some(m) => {
                        if curr_mag < m {
                            mag = Some(curr_mag)
                        }
                    }
                }
            }
        }
    }

    match mag {
        None => None,
        Some(m) => Some(v.with_magnitude(m)),
    }
}
