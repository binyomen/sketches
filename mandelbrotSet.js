// Based on https://www.youtube.com/watch?v=6z7GQewK-Ks

const MAX_ITERATIONS = 100;

function setup() {
    createCanvas(5590, 5590);
    pixelDensity(1);

    noLoop();
}

function draw() {
    loadPixels();

    for (const x of Array(width).keys()) {
        for (const y of Array(height).keys()) {
            let a = map(x, 0, width, -2.5, 1.5);
            let b = map(y, 0, height, -2, 2);

            const ca = a;
            const cb = b;

            let n = 0;
            while (n < MAX_ITERATIONS) {
                const aa = a*a - b*b;
                const bb = 2 * a * b;

                a = aa + ca;
                b = bb + cb;

                if (abs(a + b) > 16) {
                    break;
                }

                ++n;
            }

            const index = (x + y * width) * 4;

            if (n == MAX_ITERATIONS) {
                pixels[index+0] = 0;
                pixels[index+1] = 0;
                pixels[index+2] = 0;
                pixels[index+3] = 255;
            } else {
                pixels[index+0] = map(n, 0, MAX_ITERATIONS, 40, 80);
                pixels[index+1] = map(n, 0, MAX_ITERATIONS, 43, 150);
                pixels[index+2] = map(n, 0, MAX_ITERATIONS, 20, 30);
                pixels[index+3] = 255;
            }
        }
    }

    updatePixels();
}
