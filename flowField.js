// Based on https://www.youtube.com/watch?v=BjoM9oKOAKY

const INCR = 0.1;

const SCALE = 10;
let COLS;
let ROWS;

let particles = [];
let flowField = [];

let hues = [];

let fr;

function setup() {
    createCanvas(1080, 720);

    colorMode(HSB, 255);
    background(30);

    COLS = floor(width / SCALE);
    ROWS = floor(height / SCALE);

    for (const i of Array(10000).keys()) {
        particles[i] = new Particle();
    }

    flowField = new Array(COLS*ROWS);
    hues = new Array(width*height);
    for (const i of Array(hues.length).keys()) {
        hues[i] = 255;
    }

    fr = createP('');
}

class Particle {
    constructor() {
        this.pos = createVector(random(width), random(height));
        this.vel = createVector(0, 0);
        this.acc = createVector(0, 0);
        this.maxSpeed = 2;
        this.prevPos = this.pos.copy();
    }

    follow(flowField) {
        const x = floor(this.pos.x / SCALE);
        const y = floor(this.pos.y / SCALE);
        const index = x + y * COLS;

        const force = flowField[index];
        this.applyForce(force);
    }

    update() {
        this.prevPos = this.pos.copy();

        this.vel.add(this.acc);
        this.vel.limit(this.maxSpeed);
        this.pos.add(this.vel);

        this.acc.mult(0);
        this.edges();
    }

    edges() {
        if (this.pos.x > width) {
            this.pos.x = 0;
            this.prevPos = this.pos.copy();
        }
        if (this.pos.x < 0) {
            this.pos.x = width;
            this.prevPos = this.pos.copy();
        }

        if (this.pos.y > height) {
            this.pos.y = 0;
            this.prevPos = this.pos.copy();
        }
        if (this.pos.y < 0) {
            this.pos.y = height;
            this.prevPos = this.pos.copy();
        }
    }

    applyForce(force) {
        this.acc.add(force);
    }

    show() {
        let index = floor(this.pos.x) + floor(this.pos.y) * width;
        if (index >= hues.length) {
            index = hues.length - 1;
        } else if (index < 0) {
            index = 0;
        }

        const h = max(hues[index] - 10, 120);
        hues[index]  = h;
        stroke(h, 255, 255, 5);
        strokeWeight(1);
        line(this.prevPos.x, this.prevPos.y, this.pos.x, this.pos.y);
    }
}

let zOffset = 0;
function draw() {
    let yOffset = 0;
    for (const y of Array(ROWS).keys()) {
        let xOffset = 0;
        for (const x of Array(COLS).keys()) {
            const index = x + y * COLS;

            const angle = noise(xOffset, yOffset, zOffset) * TWO_PI;

            const v = p5.Vector.fromAngle(angle);
            v.setMag(2);
            flowField[index] = v;

            xOffset += INCR;
        }
        yOffset += INCR;
    }

    for (const particle of particles) {
        particle.follow(flowField);
        particle.update();
        particle.show();
    }

    zOffset += INCR/5;

    if (frameCount % 5 == 0) {
        fr.html('FPS: ' + floor(frameRate()));
    }
}
