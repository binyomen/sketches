final int NUM_ROPES = 7;
final float NOISE_FACTOR = 0.01;
final int STRAND_POS_OFFSET_RANGE = 20;
final float STRAND_ANGLE_OFFSET_RANGE = PI / 12;

ArrayList<Rope> ropes;

void setup() {
    size(800, 500);

    background(30);

    ropes = new ArrayList<Rope>();
    for (int i = 0; i < NUM_ROPES; ++i) {
        // evenly space the ropes.
        final float x = (i + 0.5) * (width / NUM_ROPES);
        ropes.add(new Rope(new PVector(x, height)));
    }
}

void draw() {
    for (final Rope rope : ropes) {
        rope.update();
        rope.show();
    }
}
