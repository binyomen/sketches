// based on https://github.com/nishtahir/generative-impressionism/

const SCALE_FACTOR = 10;
const OFFSET_INC = 0.1;

const MIN_LIFE_SPAN = 1;
const MAX_LIFE_SPAN = 5;

const WEIGHT_DEC_FACTOR = 0.001;
const STARTING_WEIGHT = 50;
const MIN_WEIGHT = 5;

const NUM_PARTICLES = 100;
const VECTOR_MAG = 6;
const PARTICLE_ALPHA = 100;
const MAX_SPEED = 3;

class System {
    constructor(img) {
        this.img = img;

        this.rows = floor(height / SCALE_FACTOR);
        this.cols = floor(width / SCALE_FACTOR);

        this.particles = this.generateParticles();
        this.flowField = this.generateFlowField();

        this.weight = STARTING_WEIGHT;
    }

    update() {
        for (const p of this.particles) {
            this.update_p(p);
        }
    }

    draw() {
        // this.renderFlowField();
        for (const p of this.particles) {
            this.draw_p(p);
        }
    }

    update_p(p) {
        if (p.life < 0) {
            const newPos = createVector(random(width), random(height));
            this.resetParticle(p, newPos, this.getLifespan());
        }

        if (p.color == null) {
            const index = (floor(p.pos.x) + floor(p.pos.y) * width) * 4;
            const r = this.img.pixels[index+0];
            const g = this.img.pixels[index+1];
            const b = this.img.pixels[index+2];

            p.color = color(r, g, b, PARTICLE_ALPHA);
        }

        this.applyFlowFieldToParticle(p, this.flowField);

        p.updatePhysics(MAX_SPEED);
        p.life -= 1;

        this.wrapEdgesToBounds(p, width, height);

        if (this.weight > MIN_WEIGHT) {
            this.weight -= WEIGHT_DEC_FACTOR;
        }
    }

    draw_p(p) {
        push();

        const c = p.color == null ? color(0, 0, 0) : p.color;

        noStroke();
        fill(c);
        circle(p.pos.x, p.pos.y, this.weight);

        pop();
    }

    // ONLY FOR DEBUGGING
    renderFlowField() {
        for (const x of Array(this.cols).keys()) {
            for (const y of Array(this.rows).keys()) {
                push();

                const index = x + y * this.cols;
                stroke(0, 50);
                translate(x * SCALE_FACTOR, y * SCALE_FACTOR);
                rotate(this.flowField[index].heading());
                line(0, 0, SCALE_FACTOR, 0);

                pop();
            }
        }
    }

    applyFlowFieldToParticle(p, flowField) {
        const x = floor(p.pos.x / SCALE_FACTOR);
        const y = floor(p.pos.y / SCALE_FACTOR);

        const index = x + y * this.cols;
        const force = flowField[index];
        p.applyForce(force);
    }

    resetParticle(p, pos, life) {
        p.pos = pos.copy();
        p.prev = pos.copy();
        p.color = null;
        p.life = life;
    }

    wrapEdgesToBounds(p, w, h) {
        if (p.pos.x > w) {
            p.pos.x = 0;
            p.updatePrev();
        }
        if (p.pos.x < 0) {
            p.pos.x = w;
            p.updatePrev();
        }

        if (p.pos.y > h) {
            p.pos.y = 0;
            p.updatePrev();
        }
        if (p.pos.y < 0) {
            p.pos.y = h;
            p.updatePrev();
        }
    }

    generateParticles() {
        const particles = [];

        for (const i of Array(NUM_PARTICLES).keys()) {
            const p = new Particle(random(width), random(height));
            p.vel = p5.Vector.random2D();
            p.life = this.getLifespan();

            particles.push(p);
        }

        return particles;
    }

    generateFlowField() {
        const flowField = [];

        for (const x of Array(this.cols).keys()) {
            for (const y of Array(this.rows).keys()) {
                const noiseValue = noise(x * OFFSET_INC, y * OFFSET_INC);
                const angle = map(noiseValue, 0, 1, 0, TWO_PI);

                const v = p5.Vector.fromAngle(angle);
                v.setMag(VECTOR_MAG);
                flowField.push(v);
            }
        }

        return flowField;
    }

    getLifespan() {
        return random(MIN_LIFE_SPAN, MAX_LIFE_SPAN);
    }
}
