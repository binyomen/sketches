'use strict';

// based on https://github.com/nishtahir/generative-impressionism/

const SCALE_FACTOR = 200;
const OFFSET_INC = 0.1;

const MIN_LIFE_SPAN = 1;
const MAX_LIFE_SPAN = 12;

const PARTICLE_ALPHA_MAX = 30;
const PARTICLE_ALPHA_MIN = 0;

const WEIGHT_DEC_FACTOR = 0.01;
const STARTING_WEIGHT = 340;
const MIN_WEIGHT = 30;

const RECT_WIDTH_FACTOR = 1.75;
const RECT_HEIGHT_FACTOR = 1;
const RECT_CORNER = 200;

const FLOW_FIELD_MAG = 6;
const MAX_SPEED = 6;

const NUM_PARTICLES = 100;
const MARGIN = 150;

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
            const newPos = createVector(random(MARGIN, width-MARGIN), random(MARGIN, height-MARGIN));
            this.resetParticle(p, newPos, this.getLifespan());
        }

        if (p.color == null) {
            const index = (floor(p.pos.x) + floor(p.pos.y) * width) * 4;
            const r = this.img.pixels[index+0];
            const g = this.img.pixels[index+1];
            const b = this.img.pixels[index+2];

            p.color = color(r, g, b);
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
        c.setAlpha(this.getParticleAlpha(p));

        noStroke();
        fill(c);

        rectMode(CENTER);
        translate(p.pos.x, p.pos.y);
        rotate(p.vel.heading());

        let w = this.weight*RECT_WIDTH_FACTOR;
        let h = this.weight*RECT_HEIGHT_FACTOR;
        rect(0, 0, w, h, RECT_CORNER);

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

    getParticleAlpha(p) {
        return map(p.life, MAX_LIFE_SPAN, MIN_LIFE_SPAN, PARTICLE_ALPHA_MIN, PARTICLE_ALPHA_MAX);
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
        if (p.pos.x > w-MARGIN) {
            p.pos.x = MARGIN;
            p.updatePrev();
        }
        if (p.pos.x < MARGIN) {
            p.pos.x = w-MARGIN;
            p.updatePrev();
        }

        if (p.pos.y > h-MARGIN) {
            p.pos.y = MARGIN;
            p.updatePrev();
        }
        if (p.pos.y < MARGIN) {
            p.pos.y = h-MARGIN;
            p.updatePrev();
        }
    }

    generateParticles() {
        const particles = [];

        for (const i of Array(NUM_PARTICLES).keys()) {
            const p = new Particle(random(MARGIN,width-MARGIN), random(MARGIN,height-MARGIN));
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
                v.setMag(FLOW_FIELD_MAG);
                flowField.push(v);
            }
        }

        return flowField;
    }

    getLifespan() {
        return random(MIN_LIFE_SPAN, MAX_LIFE_SPAN);
    }
}
