// Based on https://www.youtube.com/watch?v=QHEQuoIKgNE

class Circle {
    constructor(x, y) {
        this.x = x;
        this.y = y;
        this.r = 0;

        this.growing = true;
    }

    show() {
        push();

        const sb = 150;

        const hu = map(this.r, 0, 40, 0, 255, true);
        stroke(hu, sb, sb);
        fill(hu, sb, sb);
        strokeWeight(1);
        ellipse(this.x, this.y, this.r*2);

        pop();
    }

    grow() {
        if (this.growing && !this.edges() && !this.collide()) {
            ++this.r;
        }
    }

    edges() {
        const result = this.x + this.r > width ||
            this.x - this.r < 0 ||
            this.y + this.r > height ||
            this.y - this.r < 0;
        if (result) {
            this.growing = false;
        }

        return result;
    }

    collide() {
        for (const other of circles) {
            if (other != this) {
                const d = dist(this.x, this.y, other.x, other.y);
                if (d - 1 < this.r + other.r) {
                    this.growing = false;
                    return true;
                }
            }
        }

        return false;
    }
}

let circles = [];
let spots = [];
let image;

let fr;

function preload() {
    image = loadImage('data/2017.png', function(){}, function(e) {console.log(e);});
}

function setup() {
    createCanvas(900, 400);

    colorMode(HSB, 255);

    circles.push(new Circle(200, 200));

    image.loadPixels();
    for (const x of Array(image.width).keys()) {
        for (const y of Array(image.height).keys()) {
            const index = (x + y * image.width) * 4;
            if (image.pixels[index+0] > 1 &&
                image.pixels[index+1] > 1 &&
                image.pixels[index+2] > 1) {
                    spots.push(createVector(x, y));
            }
        }
    }

    fr = createP('');
}

function draw() {
    background(30);

    newCircle();

    for (const c of circles) {
        c.show();
        c.grow();
    }

    if (frameCount % 5 == 0) {
        fr.html('FPS: ' + floor(frameRate()));
    }
}

function newCircle() {
    let attempts = 0;
    let n = 0;
    while (n < 100) {
        const pos = random(spots);

        let valid = true;
        for (const c of circles) {
            const d = dist(pos.x, pos.y, c.x, c.y);
            if (d < c.r) {
                // we are inside a circle
                valid = false;
                break;
            }
        }

        if (valid) {
            const c = new Circle(pos.x, pos.y);
            circles.push(c);
            ++n;
        }

        ++attempts;

        if (attempts > 1000) {
            noLoop();
            break;
        }
    }
}
