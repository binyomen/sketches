class Strand {
    private Rope _rope;
    private PVector _pos;
    private PVector _vel;
    private PVector _acc;

    private PVector _prev;

    private static final int MAX_SPEED = 1;
    private static final float MAX_FORCE = 0.03;

    Strand(final Rope rope) {
        _rope = rope;

        _pos = _rope.getPos().copy();
        _pos.x += random(-STRAND_POS_OFFSET_RANGE, STRAND_POS_OFFSET_RANGE);

        _vel = new PVector(0, -1);
        _vel.rotate(random(-STRAND_ANGLE_OFFSET_RANGE, STRAND_ANGLE_OFFSET_RANGE));

        _acc = new PVector(0, 0);

        _prev = _pos.copy();
    }

    void show() {
        push();
        stroke(_rope.getColor());
        strokeWeight(1);

        line(_prev.x, _prev.y, _pos.x, _pos.y);
        pop();
    }

    void update() {
        _prev = _pos.copy();

        follow();

        _vel.add(_acc);
        _vel.limit(MAX_SPEED);
        _pos.add(_vel);

        _acc.mult(0);
    }

    void follow() {
        if (_rope.getPoints().size() < 2) {
            return;
        }

        final PVector prediction = _vel.copy();
        prediction.setMag(MAX_SPEED);
        final PVector predictedLocation = PVector.add(_pos, prediction);

        Float minDistance = null;
        PVector minTarget = null;
        for (int i = 0; i < _rope.getPoints().size()-1; ++i) {
            final PVector a = _rope.getPoints().get(i);
            final PVector b = _rope.getPoints().get(i+1);

            final PVector normalPoint = a;

            final PVector dir = PVector.sub(b, a);
            dir.normalize();
            dir.mult(10);
            final PVector target = PVector.add(normalPoint, dir);

            final float distance = PVector.dist(predictedLocation, normalPoint);
            if (minDistance == null || distance < minDistance) {
                minDistance = distance;
                minTarget = target;
            }
        }

        if (minDistance > _rope.getRadius()) {
            seek(minTarget);
        }
    }

    void seek(final PVector target) {
        final PVector desired = PVector.sub(target, _pos);

        if (desired.mag() == 0) {
            return;
        }

        desired.normalize();
        desired.mult(MAX_SPEED);

        final PVector steer = PVector.sub(desired, _vel);
        steer.limit(MAX_FORCE);

        applyForce(steer);
    }

    void applyForce(final PVector force) {
        _acc.add(force);
    }
}
