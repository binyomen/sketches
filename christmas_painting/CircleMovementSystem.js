const NUM_PARTICLES = 50;
const RADIUS = 7;
const SPEED = 10;

class System {
    constructor(img) {
        this.img = img;

        this.particles = [];
        for (const i of Array(NUM_PARTICLES).keys()) {
            this.particles.push(new Particle(random(width), random(height)));
        }

        setMaxElementsSvg(20000);
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
        p.pos.x += dir.x * SPEED;
        p.pos.y += dir.y * SPEED;

        if (p.pos.x < 0) {
            p.pos.x = 0;
        }
        if (p.pos.x > width) {
            p.pos.x = width;
        }
        if (p.pos.y < 0) {
            p.pos.y = 0;
        }
        if (p.pos.y > height) {
            p.pos.y = height;
        }
    }

    p_draw(p) {
        const index = (floor(p.pos.x) + floor(p.pos.y) * width) * 4;
        const r = this.img.pixels[index+0];
        const g = this.img.pixels[index+1];
        const b = this.img.pixels[index+2];
        const a = this.img.pixels[index+3];

        circleSvg(p.pos.x, p.pos.y, RADIUS, color(r, g, b, a));
    }
}
