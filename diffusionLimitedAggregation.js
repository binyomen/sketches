// Based on https://www.youtube.com/watch?v=Cl_Gjj80gPE

const NUM_WALKERS = 10;
const NUM_ITERS = 1000;
const STICKINESS = 1;
const RADIUS_DECR = 0.01;
const HUE_INCR = 0.5;

let tree = [];
let walkers = [];

let r = 8;
let hu = 0;

let fr;
let monitor;

function setup() {
    createCanvas(500, 500);
    colorMode(HSB, 255);

    tree.push(new Walker(width/2, height/2, true));

    for (const i of Array(NUM_WALKERS).keys()) {
        walkers.push(new Walker());
    }

    fr = createP('');
    monitor = createDiv('');
}

class Walker {
    constructor(x = null, y = null, stuck = false) {
        if (x !== null && y !== null) {
            this.pos = createVector(x, y);
        } else {
            this.pos = randomWalkerPoint();
        }

        this.stuck = stuck;

        this.r = r;
        r -= RADIUS_DECR;

        this.hu = null;
    }

    walk() {
        const vel = p5.Vector.random2D();
        this.pos.add(vel);
        this.pos.x = constrain(this.pos.x, 0, width);
        this.pos.y = constrain(this.pos.y, 0, height);
    }

    checkStuck(others) {
        for (const other of others) {
            const d = distSq(this.pos, other.pos);
            // If we are touching the point and we pass a "stickiness test".
            if (d < this.r*other.r*4 &&
                random(1) <= STICKINESS) {
                    this.hu = hu;

                    hu += HUE_INCR;
                    constrain(hu, 0, 255);

                    this.stuck = true;
                    return true;
            }
        }

        return false;
    }

    show() {
        push();

        noStroke();

        if (this.stuck) {
            fill(this.hu, 255, 255);
        } else {
            fill(hu, 255, 255);
        }

        ellipse(this.pos.x, this.pos.y, this.r*2);

        pop();
    }
}

function draw() {
    background(0);

    for (const w of tree) {
        w.show();
    }
    for (const w of walkers) {
        w.show();
    }

    for (const n of Array(NUM_ITERS).keys()) {
        for (let i = 0; i < walkers.length; ++i) {
            walkers[i].walk();

            if (walkers[i].checkStuck(tree)) {
                const walkerToBeRemoved = walkers[i];
                tree.push(walkerToBeRemoved);

                // Eventually stop adding walkers.
                if (r > 2) {
                    const newWalker = new Walker();
                    walkers[i] = newWalker;
                } else {
                    walkers.splice(i, 1);
                    // Decrement i so that we don't skip the worker at i+1.
                    --i;
                }
            }
        }
    }

    if (frameCount % 5 == 0) {
        fr.html('FPS: ' + floor(frameRate() + 0.5));

        monitor.html(
            '<p>Walkers: ' + walkers.length + '</p>' +
            '<p>Tree Size: ' + tree.length + '</p>' +
            '<p>Radius: ' + r + '</p>');
    }
}

function randomWalkerPoint() {
    const i = random([0, 1, 2, 3]);

    if (i == 0) {
        return createVector(random(width), 0);
    } else if (i == 1) {
        return createVector(random(width), height);
    } else if (i == 2) {
        return createVector(0, random(height));
    } else if (i == 3) {
        return createVector(width, random(height));
    }
}

// An optimization so we don't need to use square roots, since we don't care
// about the exact distance.
function distSq(a, b) {
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    return dx*dx + dy*dy;
}
