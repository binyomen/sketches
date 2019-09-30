// Based on https://www.youtube.com/watch?v=O_0fRV4MTZo

let yOffset = 0;

function setup() {
    createCanvas(200, 200);

    stroke(255);
    strokeWeight(1);
    fill(255, 50);
}

function drawWingPoint(a, xOffset, isRight) {
    const n = noise(xOffset, yOffset);
    const r = sin(2*a) * map(n, 0, 1, 50, 125); // use perlin noise plus a rose

    const x =  r * cos(a);
    // Multiplying by sine of a factor of the frame count makes it flap.
    const y = sin(frameCount / 0.00001) * r * sin(a);

    vertex(x, y);

    return xOffset + (isRight ? 0.1 : -0.1);
}

function draw() {
    background(51);

    translate(width/2, height/2);
    rotate(PI / 2); // hack to make the moth right side up

    const incr = PI / 100;

    beginShape();

    // right wing
    let xOffset = 0;
    for (let a = -PI/2; a < PI/2; a += incr) {
        xOffset = drawWingPoint(a, xOffset, true);
    }

    // left wing
    for (let a = PI/2; a < 3*PI/2; a += incr) {
        xOffset = drawWingPoint(a, xOffset, false);
    }
    endShape();

    yOffset += 0.1;
}
