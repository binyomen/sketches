// Based on https://www.youtube.com/watch?v=6z7GQewK-Ks

final int MAX_ITERATIONS = 100;

void setup() {
    size(9000, 9000);
    pixelDensity(1);
    colorMode(HSB, 255);

    noLoop();
}

void draw() {
    loadPixels();

    for (int x = 0; x < width; ++x) {
        for (int y = 0; y < height; ++y) {
            float a = map(x, 0, width, -2.5, 1.5);
            float b = map(y, 0, height, -2, 2);

            final float ca = a;
            final float cb = b;

            int n = 0;
            while (n < MAX_ITERATIONS) {
                final float aa = a*a - b*b;
                final float bb = 2 * a * b;

                a = aa + ca;
                b = bb + cb;

                if (abs(a + b) > 16) {
                    break;
                }

                ++n;
            }

            final int index = x + y * width;

            if (n == MAX_ITERATIONS) {
                pixels[index] = color(0, 0, 0, 255);
            } else {
                final float hue = map(n, 0, MAX_ITERATIONS, 0, 255);
                pixels[index] = color(hue, 255, 255, 255);
            }
        }
    }

    updatePixels();

    save("mandelbrotSet.png");
    exit();
}
