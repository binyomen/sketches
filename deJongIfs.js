// Based on https://www.youtube.com/watch?v=vLlbEZt-3j0

function setup() {
    createCanvas(4*1000, 4*1000);
    background(30);
    colorMode(HSB, 255, 255, 255);
    noLoop();
}

class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }

    draw() {
        point(this.x*1000, this.y*1000);
    }
}

let currentPoint = new Point(0, 0);

function draw() {
    translate(width/2, height/2);

    for (let i = 0; i < 9999999; ++i) {
        stroke((i % 100) + 80, 100, 150);

        currentPoint.draw();
        currentPoint = ifs(currentPoint);
    }
}

let a = 0.97;
let b = -1.9;
let c = 1.38;
let d = -1.5;

function ifs(p) {
    let x = sin(a * p.y) - cos(b * p.x);
    let y = sin(c * p.x) - cos(d * p.y);
    return new Point(x, y);
}
