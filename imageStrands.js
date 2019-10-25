const OFFSET_INCR = 0.1;
const NUM_PARTICLES = 10000;
const TRANSPARENCY = 50;

const SCALE = 10;
let COLS;
let ROWS;

let particles;
let flowField;

let image;

let fr;

function preload() {
    image = loadImage('data/painting.jpg', function(){}, function(e) { console.log(e); });
}

function setup() {
    createCanvas(image.width, image.height);

    background(30);

    image.loadPixels();

    COLS = floor(width / SCALE);
    ROWS = floor(height / SCALE);

    particles = new Array(NUM_PARTICLES)
    for (const i of Array(particles.length).keys()) {
        particles[i] = new Particle();
    }

    flowField = new Array(COLS*ROWS);

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

    follow() {
        const x = floor(this.pos.x / SCALE);
        const y = floor(this.pos.y / SCALE);
        const index = x + y * COLS;

        const force = flowField[index];
        this.applyForce(force);
    }

    update() {
        this.follow();

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

    draw() {
        push();

        const index = (floor(this.pos.x) + floor(this.pos.y) * width) * 4;

        stroke(
            image.pixels[index+0],
            image.pixels[index+1],
            image.pixels[index+2],
            TRANSPARENCY);
        strokeWeight(1);

        line(this.prevPos.x, this.prevPos.y, this.pos.x, this.pos.y);

        pop();
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

            xOffset += OFFSET_INCR;
        }
        yOffset += OFFSET_INCR;
    }

    for (const particle of particles) {
        particle.update();
        particle.draw();
    }

    zOffset += OFFSET_INCR / 5;

    if (frameCount % 5 == 0) {
        fr.html('FPS: ' + floor(frameRate()));
    }
}
