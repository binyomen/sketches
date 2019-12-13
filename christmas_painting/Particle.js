'use strict';

class Particle {
    constructor(x, y) {
        this.pos = createVector(x, y);
        this.vel = createVector(0, 0);
        this.acc = createVector(0, 0);

        this.prev = this.pos.copy();
    }

    applyForce(force) {
        this.acc.add(force);
    }

    updatePhysics(maxSpeed) {
        this.updatePrev();

        this.vel.add(this.acc);
        this.vel.limit(maxSpeed);
        this.pos.add(this.vel);

        this.acc.mult(0);
    }

    updatePrev() {
        this.prev = this.pos.copy();
    }
}
