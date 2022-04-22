// Based on https://www.youtube.com/watch?v=sJ6pMLp_IaI and https://www.youtube.com/watch?v=lobJ9gzbLo8

let delta;
let angle;

const SIDES = 4;

let polys;

function setup() {
    createCanvas(400, 400);

    polys = [];

    const incr = 100;
    for (let x = 0; x < width; x += incr) {
        for (let y = 0; y < height; y += incr) {
            const poly = new Polygon();
            poly.addVertex(x, y);
            poly.addVertex(x + incr, y);
            poly.addVertex(x + incr, y + incr);
            poly.addVertex(x, y + incr);
            poly.close();

            polys.push(poly);
        }
    }
}

class Edge {
    constructor(a, b) {
        this.a = a;
        this.b = b;
    }

    show() {
        this.h1.show();
        this.h2.show();
    }

    hankin() {
        const mid = p5.Vector.add(this.a, this.b);
        mid.mult(0.5);

        const v1 = p5.Vector.sub(this.a, mid);
        const v2 = p5.Vector.sub(this.b, mid);

        const edgeLen = v1.mag() + delta;

        let offset1 = mid;
        let offset2 = mid;

        if (delta > 0) {
            v1.setMag(delta);
            v2.setMag(delta);

            offset1 = p5.Vector.add(mid, v2);
            offset2 = p5.Vector.add(mid, v1);
        }

        v1.normalize();
        v2.normalize();

        v1.rotate(-angle);
        v2.rotate(angle);

        // law of sines
        const interior = (SIDES - 2) * PI / SIDES;
        const alpha = interior / 2;
        const beta = PI - angle - alpha;
        const hankinLen = edgeLen * sin(alpha) / sin(beta);

        v1.setMag(hankinLen);
        v2.setMag(hankinLen);

        this.h1 = new Hankin(offset1, v1);
        this.h2 = new Hankin(offset2, v2);
    }
}

class Polygon {
    constructor() {
        this.edges = [];
        this.vertices = [];
    }

    addVertex(x, y) {
        const a = createVector(x, y);

        const numVertices = this.vertices.length;
        if (numVertices > 0) {
            const prev = this.vertices[numVertices - 1];
            const edge = new Edge(prev, a);
            this.edges.push(edge);
        }

        this.vertices.push(a);
    }

    close() {
        const first = this.vertices[0];
        const last = this.vertices[this.vertices.length - 1];

        const edge = new Edge(last, first);
        this.edges.push(edge);
    }

    hankin() {
        for (const edge of this.edges) {
            edge.hankin();
        }
    }

    show() {
        for (const edge of this.edges) {
            edge.show();
        }
    }
}

class Hankin {
    constructor(a, v) {
        this.a = a;
        this.v = v;
        this.b = p5.Vector.add(this.a, this.v);
    }

    show() {
        stroke(255, 0, 255);
        line(this.a.x, this.a.y, this.b.x, this.b.y);
    }
}

let n1 = 0;
let n2 = 0;
function draw() {
    background(51);

    delta = map(sin(n1), -1, 1, 0, 25);
    angle = map(sin(n2), -1, 1, 0, HALF_PI);

    for (const poly of polys) {
        poly.hankin();
        poly.show();
    }

    n1 += 0.02;
    n2 += 0.034;
}
