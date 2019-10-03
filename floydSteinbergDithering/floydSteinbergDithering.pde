// Based on https://www.youtube.com/watch?v=0L2n8Tg2FwI

PImage kitten;

void setup() {
    size(1024, 512);

    kitten = loadImage("data/kitten.jpg");
    kitten.filter(GRAY);
    image(kitten, 0, 0);
}

class Error {
    public final float r;
    public final float g;
    public final float b;

    Error(final float r, final float g, final float b) {
        this.r = r;
        this.g = g;
        this.b = b;
    }
}

void draw() {
    kitten.loadPixels();

    for (int y = 0; y < kitten.height; ++y) {
        for (int x = 0; x < kitten.width; ++x) {
            final int index = x + y * kitten.width;
            final color c = kitten.pixels[index];

            // Produce 0 or 255 for each color.
            final int factor = 1;
            final int r = round(factor * red(c) / 255) * (255 / factor);
            final int g = round(factor * green(c) / 255) * (255 / factor);
            final int b = round(factor * blue(c) / 255) * (255 / factor);
            kitten.pixels[index] = color(r, g, b);

            final Error error = new Error(red(c) - r, green(c) - g, blue(c) - b);
            distributeError(x+1, y, error, 7/16.0);
            distributeError(x-1, y+1, error, 3/16.0);
            distributeError(x, y+1, error, 5/16.0);
            distributeError(x+1, y+1, error, 1/16.0);
        }
    }

    kitten.updatePixels();

    image(kitten, 512, 0);
}

void distributeError(final int x, final int y, final Error error, final float factor) {
    if (x > 0 && x < kitten.width && y > 0 && y < kitten.height) {
        final int index = x + y * kitten.width;
        final color c = kitten.pixels[index];

        final float r = red(c) + error.r * factor;
        final float g = green(c) + error.g * factor;
        final float b = blue(c) + error.b * factor;

        kitten.pixels[index] = color(r, g, b);
    }
}
