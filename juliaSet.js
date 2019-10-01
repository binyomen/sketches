// Based on https://www.youtube.com/watch?v=fAsaSkmbF5s


const MAX_ITERATIONS = 50;

const PHI = 1.6180339887498948482;
const KNOWN_SETS = [
    [1 - PHI, 0],
    [PHI - 2, PHI - 1],
    [0.285, 0],
    [0.285, 0.01],
    [0.45, 0.1428],
    [-0.70176, -0.3842],
    [-0.835, -0.2321],
    [-0.8, 0.156],
    [-0.7269, 0.1889],
    [0, -0.8],
];

function setup() {
    createCanvas(200, 200);
    pixelDensity(1);
}

function draw() {
    loadPixels();

    // const knownSet = random(KNOWN_SETS);
    // const ca = knownSet[0];
    // const cb = knownSet[1];
    const ca = map(mouseX, 0, width, -2, 2);
    const cb = map(mouseY, 0, height, -2, 2);

    for (const x of Array(width).keys()) {
        for (const y of Array(height).keys()) {
            let a = map(x, 0, width, -2, 2);
            let b = map(y, 0, height, -2, 2);

            let n = 0;
            while (n < MAX_ITERATIONS) {
                const aa = a*a - b*b;
                const bb = 2 * a * b;

                if (a*a + b*b > 4) {
                    break;
                }

                a = aa + ca;
                b = bb + cb;

                ++n;
            }

            const index = (x + y * width) * 4;

            if (n == MAX_ITERATIONS) {
                pixels[index+0] = 0;
                pixels[index+1] = 0;
                pixels[index+2] = 0;
                pixels[index+3] = 255;
            } else {
                pixels[index+0] = map(n, 0, MAX_ITERATIONS, 0, 255);
                pixels[index+1] = map(n, 0, MAX_ITERATIONS, 0, 255);
                pixels[index+2] = map(n, 0, MAX_ITERATIONS, 0, 255);
                pixels[index+3] = 255;
            }
        }
    }

    updatePixels();
}
