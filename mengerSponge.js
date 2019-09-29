// Based on https://www.youtube.com/watch?v=LG8ZK-rRkXo

let sponge = [];

function setup() {
    createCanvas(400, 400, WEBGL);

    noStroke();
    noFill();

    const b = new Box(0, 0, 0, 200);
    sponge.push(b);
}

class Box {
    constructor(x, y, z, r) {
        this.pos = createVector(x, y, z);
        this.r = r;
    }

    show() {
        push();
        translate(this.pos.x, this.pos.y, this.pos.z);
        fill(255);
        box(this.r);
        pop();
    }

    generate() {
        let boxes = [];
        for (let i = -1; i < 2; ++i) {
            for (let j = -1; j < 2; ++j) {
                for (let k = -1; k < 2; ++k) {
                    // Determine if we should add the box.
                    const sum = abs(i) + abs(j) + abs(k);

                    if (sum > 1) {
                        const newR = this.r / 3;
                        const b = new Box(
                            this.pos.x + i*newR,
                            this.pos.y + j*newR,
                            this.pos.z + k*newR,
                            newR);

                        boxes.push(b);
                    }
                }
            }
        }

        return boxes;
    }
}

let a = 0;
function draw() {
    background(51);
    lights();

    rotateX(a);
    rotateY(a);
    rotateZ(a);

    for (const b of sponge) {
        b.show();
    }

    a += 0.01;
}

function mousePressed() {
    let newSponges = [];
    for (const b of sponge) {
        newSponges = newSponges.concat(b.generate());
    }
    sponge = newSponges;
}
