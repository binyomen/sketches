// Based on https://www.youtube.com/watch?v=0dwJ-bkJwDI

const NUM_ORBITS = 10;
const RESOLUTION = 1000;
const K = -4;

const FAST_MODE = false;

let path = [];

let root;

function setup() {
    createCanvas(600, 600);

    root = new Orbit(300, 300, 150, null, 0)

    let current = root;
    for (let i of Array(NUM_ORBITS-1).keys()) {
        current = current.addChild();
    }
}

class Orbit {
    constructor(x, y, r, parent, level) {
        this.x = x;
        this.y = y;
        this.r = r;
        this.parent = parent;
        this.child = null;
        this.angle = -HALF_PI;
        this.level = level;
        this.speed = pow(K, this.level-1) / RESOLUTION;
    }

    show() {
        stroke(255);
        strokeWeight(2);
        noFill();
        ellipse(this.x, this.y, this.r*2);

        if (this.child) {
            this.child.show();
        }
    }

    update() {
        if (this.parent) {
            this.angle += this.speed;

            const rSum = this.r + this.parent.r;
            this.x = this.parent.x + rSum * cos(this.angle);
            this.y = this.parent.y + rSum * sin(this.angle);
        }

        if (this.child) {
            this.child.update();
        } else {
            path.push(createVector(this.x, this.y));
        }
    }

    addChild() {
        const newR = this.r / 3;
        const newX = this.x + this.r + newR;
        const newY = this.y;

        this.child = new Orbit(newX, newY, newR, this, this.level + 1);
        return this.child;
    }
}

function draw() {
    background(51);

    if (FAST_MODE) {
        for (const i of Array(RESOLUTION).keys()) {
            root.update();
        }
    } else {
        root.update();
    }

    root.show();

    stroke(255, 0, 255);
    beginShape();
    for (const vec of path) {
        vertex(vec.x, vec.y);
    }
    endShape();
}
