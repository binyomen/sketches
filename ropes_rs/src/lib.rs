use nannou::{
    math::{cgmath, Basis2, Rad, Rotation2},
    noise::NoiseFn,
    prelude::*,
    Draw,
};
use rand::{rngs::ThreadRng, Rng};

use std::cell::RefCell;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 500;

pub const NUM_ROPES: u32 = 7;
const NOISE_FACTOR: f32 = 0.01;
const STRAND_POS_OFFSET_RANGE: f32 = 20.0;
const STRAND_ANGLE_OFFSET_RANGE: f32 = PI / 12.0;
const NUM_STRANDS: usize = 100;
const ROPE_RADIUS: f32 = 10.0;
const STRAND_MAX_SPEED: f32 = 1.0;
const STRAND_MAX_FORCE: f32 = 0.03;

fn generate_direction(noise: &mut Box<dyn NoiseFn<[f64; 3]>>, x: f32, y: f32, id: u32) -> Vector2 {
    let angle = noise.get([
        (x * NOISE_FACTOR) as f64,
        (y * NOISE_FACTOR) as f64,
        id as f64,
    ]) as f32
        * PI
        * 2.0;

    vec2(angle.cos(), angle.sin()).normalize()
}

pub struct Rope {
    id: u32,

    pos: Point2,
    prev: Point2,
    direction: Vector2,

    color: Rgba,

    points: Vec<Point2>,
    strands: RefCell<Vec<Strand>>,
}

impl Rope {
    pub fn new(rng: &mut ThreadRng, noise: &mut Box<dyn NoiseFn<[f64; 3]>>, pos: Point2) -> Self {
        let id = rng.gen_range(0, 10000);

        let red = rng.gen_range(0.27, 1.0);
        let green = rng.gen_range(0.27, 1.0);
        let blue = rng.gen_range(0.27, 1.0);

        let mut strands = Vec::with_capacity(NUM_STRANDS);
        strands.resize_with(NUM_STRANDS, || Strand::new(rng, pos));

        Rope {
            id: id,
            pos: pos,
            prev: pos,
            direction: generate_direction(noise, pos.x, pos.y, id),
            color: rgba(red, green, blue, 0.157),
            points: vec![],
            strands: RefCell::new(strands),
        }
    }

    pub fn update(&mut self, noise: &mut Box<dyn NoiseFn<[f64; 3]>>) {
        self.prev = self.pos;
        self.points.push(self.prev);

        let actual_direction = self.direction;

        // Perlin noise tends towards 0.5, which means the ropes will move left.
        // Rotate the direction so it moves up instead.
        let rot: Basis2<f32> = Rotation2::from_angle(Rad(PI / 2.0));
        let actual_direction =
            rot.rotate_vector(cgmath::Vector2::new(actual_direction.x, actual_direction.y));

        self.pos = pt2(
            self.pos.x + actual_direction.x,
            self.pos.y + actual_direction.y,
        );

        self.direction = generate_direction(noise, self.pos.x, self.pos.y, self.id);

        for strand in &mut *self.strands.borrow_mut() {
            strand.update(self);
        }
    }

    pub fn view(&self, draw: &Draw) {
        // draw.line()
        //     .color(rgb(1.0, 1.0, 1.0))
        //     .weight(1.0)
        //     .start(self.prev)
        //     .end(self.pos);

        for strand in &*self.strands.borrow() {
            strand.view(draw, self);
        }
    }
}

struct Strand {
    pos: Point2,
    prev: Point2,
    vel: Vector2,
    acc: Vector2,
}

impl Strand {
    fn new(rng: &mut ThreadRng, pos: Vector2) -> Self {
        let pos = pt2(
            pos.x + rng.gen_range(-STRAND_POS_OFFSET_RANGE, STRAND_POS_OFFSET_RANGE),
            pos.y,
        );

        let angle = rng.gen_range(-STRAND_ANGLE_OFFSET_RANGE, STRAND_ANGLE_OFFSET_RANGE);
        let rot: Basis2<f32> = Rotation2::from_angle(Rad(angle));
        let vel = rot.rotate_vector(cgmath::Vector2::new(0.0, 1.0));

        Strand {
            pos: pos,
            prev: pos,
            vel: vec2(vel.x, vel.y),
            acc: vec2(0.0, 0.0),
        }
    }

    fn update(&mut self, rope: &Rope) {
        self.prev = self.pos;

        self.follow(rope);

        self.vel += self.acc;
        self.vel = self.vel.limit_magnitude(STRAND_MAX_SPEED);
        self.pos += self.vel;

        self.acc *= 0.0;
    }

    fn follow(&mut self, rope: &Rope) {
        if rope.points.len() < 2 {
            return;
        }

        let prediction = self.vel.with_magnitude(STRAND_MAX_SPEED);
        let predicted_location = self.pos + prediction;

        let mut min_distance = None;
        let mut min_target = None;
        for i in 0..rope.points.len() - 1 {
            let a = rope.points[i];
            let b = rope.points[i + 1];

            let normal_point = a;

            let dir = b - a;
            let dir = dir.normalize();
            let dir = dir * 10.0;
            let target = normal_point + dir;

            let distance = predicted_location.distance2(normal_point);
            if min_distance.is_none() || distance < min_distance.unwrap() {
                min_distance = Some(distance);
                min_target = Some(target);
            }
        }

        if min_distance.is_some() && min_distance.unwrap() > ROPE_RADIUS {
            self.seek(min_target.unwrap());
        }
    }

    fn seek(&mut self, target: Point2) {
        let desired = target - self.pos;

        if desired.x == 0.0 && desired.y == 0.0 {
            return;
        }

        let desired = desired.normalize() * STRAND_MAX_SPEED;

        let steer = (desired - self.vel).limit_magnitude(STRAND_MAX_FORCE);

        self.apply_force(steer);
    }

    fn apply_force(&mut self, force: Vector2) {
        self.acc += force;
    }

    fn view(&self, draw: &Draw, rope: &Rope) {
        draw.line()
            .color(rope.color)
            .weight(1.0)
            .start(self.prev)
            .end(self.pos);
    }
}
