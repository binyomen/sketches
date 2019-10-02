const NUM_ROPES = 7;
const NOISE_FACTOR = 0.01;
const STRAND_POS_OFFSET_RANGE = 20;
let STRAND_ANGLE_OFFSET_RANGE;

let ropes = [];

let fr;

function setup() {
    createCanvas(800, 500);

    background(30);

    STRAND_ANGLE_OFFSET_RANGE = PI / 12;

    for (const i of Array(NUM_ROPES).keys()) {
        // evenly space the ropes.
        const x = (i + 0.5) * (width / NUM_ROPES);
        ropes.push(new Rope(createVector(x, height)));
    }

    fr = createP('');
}

class Rope {
    constructor(pos) {
        this.pos = pos;
        this.prev = this.pos.copy();
        this.direction = this.generateDirection();
        this.r = 10;

        this.color = color(random(70, 255), random(70, 255), random(70, 255), 40);

        this.points = [];

        this.id = random(10000);

        this.strands = [];
        for (const i of Array(100).keys()) {
            this.strands.push(new Strand(this));
        }
    }

    show() {
        // push();
        // stroke(255);
        // strokeWeight(1);

        // line(this.prev.x, this.prev.y, this.pos.x, this.pos.y);

        for (const strand of this.strands) {
            strand.show();
        }

        // pop();
    }

    update() {
        this.prev = this.pos.copy();
        this.points.push(this.prev);

        const actualDirection = this.direction.copy();

        // Perlin noise tends towards 0.5, which means the ropes will move left.
        // Rotate the direction so it moves up instead.
        actualDirection.rotate(HALF_PI);

        this.pos.add(actualDirection);

        this.direction = this.generateDirection();

        for (const strand of this.strands) {
            strand.update();
        }
    }

    generateDirection() {
        const angle = noise(this.pos.x*NOISE_FACTOR, this.pos.y*NOISE_FACTOR, this.id) * TWO_PI;
        return p5.Vector.fromAngle(angle);
    }
}

class Strand {
    constructor(rope) {
        this.rope = rope;

        this.pos = this.rope.pos.copy();
        this.pos.x += random(-STRAND_POS_OFFSET_RANGE, STRAND_POS_OFFSET_RANGE);

        this.vel = createVector(0, -1);
        this.vel.rotate(random(-STRAND_ANGLE_OFFSET_RANGE, STRAND_ANGLE_OFFSET_RANGE));

        this.acc = createVector(0, 0);

        this.maxSpeed = 1;
        this.maxForce = 0.03;

        this.prev = this.pos.copy();
    }

    show() {
        push();
        stroke(this.rope.color);
        strokeWeight(1);

        line(this.prev.x, this.prev.y, this.pos.x, this.pos.y);
        pop();
    }

    update() {
        this.prev = this.pos.copy();

        this.follow();

        this.vel.add(this.acc);
        this.vel.limit(this.maxSpeed);
        this.pos.add(this.vel);

        this.acc.mult(0);
    }

    follow() {
        if (this.rope.points.length < 2) {
            return;
        }

        const prediction = this.vel.copy();
        prediction.setMag(this.maxSpeed);
        const predictedLocation = p5.Vector.add(this.pos, prediction);

        let minDistance = null;
        let minTarget = null;
        for (let i = 0; i < this.rope.points.length-1; ++i) {
            const a = this.rope.points[i];
            const b = this.rope.points[i+1];

            const normalPoint = a;

            const dir = p5.Vector.sub(b, a);
            dir.normalize();
            dir.mult(10);
            const target = p5.Vector.add(normalPoint, dir);

            const distance = p5.Vector.dist(predictedLocation, normalPoint);
            if (minDistance == null || distance < minDistance) {
                minDistance = distance;
                minTarget = target;
            }
        }

        if (minDistance > this.rope.r) {
            this.seek(minTarget);
        }
    }

    seek(target) {
        const desired = p5.Vector.sub(target, this.pos);

        if (desired.mag() == 0) {
            return;
        }

        desired.normalize();
        desired.mult(this.maxSpeed);

        const steer = p5.Vector.sub(desired, this.vel);
        steer.limit(this.maxForce);

        this.applyForce(steer);
    }

    applyForce(force) {
        this.acc.add(force);
    }
}

function draw() {
    for (const rope of ropes) {
        rope.update();
        rope.show();
    }

    if (frameCount % 5 == 0) {
        fr.html('FPS: ' + floor(frameRate()));
    }
}
