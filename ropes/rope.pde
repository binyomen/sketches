class Rope {
    private PVector _pos;
    private PVector _prev;
    private PVector _direction;

    private float _r;

    private color _c;

    private ArrayList<PVector> _points;
    private ArrayList<Strand> _strands;

    private float _id;

    Rope(final PVector pos) {
        _pos = pos;
        _prev = _pos.copy();
        _direction = generateDirection();
        _r = 10;

        _c = color(random(70, 255), random(70, 255), random(70, 255), 40);

        _points = new ArrayList<PVector>();

        _id = random(10000);

        _strands = new ArrayList<Strand>();
        for (int i = 0; i < 100; ++i) {
            _strands.add(new Strand(this));
        }
    }

    PVector getPos() {
        return _pos;
    }

    color getColor() {
        return _c;
    }

    ArrayList<PVector> getPoints() {
        return _points;
    }

    float getRadius() {
        return _r;
    }

    void show() {
        // push();
        // stroke(255);
        // strokeWeight(1);

        // line(_prev.x, _prev.y, _pos.x, _pos.y);

        for (final Strand strand : _strands) {
            strand.show();
        }

        // pop();
    }

    void update() {
        _prev = _pos.copy();
        _points.add(_prev);

        final PVector actualDirection = _direction.copy();

        // Perlin noise tends towards 0.5, which means the ropes will move left.
        // Rotate the direction so it moves up instead.
        actualDirection.rotate(HALF_PI);

        _pos.add(actualDirection);

        _direction = generateDirection();

        for (final Strand strand : _strands) {
            strand.update();
        }
    }

    PVector generateDirection() {
        final float angle = noise(_pos.x*NOISE_FACTOR, _pos.y*NOISE_FACTOR, _id) * TWO_PI;
        return PVector.fromAngle(angle);
    }
}
