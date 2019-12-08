const NUM_PARTICLES = 50;
const RADIUS = 7;
const SPEED = 10;

class CircleMovementSystem {
    constructor(img) {
        this.img = img;

        this.particles = [];
        for (const i of Array(NUM_PARTICLES).keys()) {
            this.particles.push(new Particle(width/2, height/2));
        }
    }

    update() {
        for (const p of this.particles) {
            this.p_update(p);
        }
    }

    draw() {
        for (const p of this.particles) {
            this.p_draw(p);
        }
    }

    p_update(p) {
        const dir = createVector(random(-1, 1), random(-1, 1));
        p.x += dir.x * SPEED;
        p.y += dir.y * SPEED;

        if (p.x < 0) {
            p.x = 0;
        }
        if (p.x > width) {
            p.x = width;
        }
        if (p.y < 0) {
            p.y = 0;
        }
        if (p.y > height) {
            p.y = height;
        }
    }

    p_draw(p) {
        push();

        noStroke();

        const index = (floor(p.x) + floor(p.y) * width) * 4;
        const r = this.img.pixels[index+0];
        const g = this.img.pixels[index+1];
        const b = this.img.pixels[index+2];
        const a = this.img.pixels[index+3];
        fill(color(r, g, b, a));

        circle(p.x, p.y, RADIUS);

        pop();
    }
}
