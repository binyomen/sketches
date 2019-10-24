function setup() {
    createCanvas(500, 500);

    background(30);

    noLoop();
}

class Feather {
    constructor() {
        this.pos = createVector(random(width), random(height));
        this.angle = random(TWO_PI);
        this.len = random(100);

        this.controlFactor = this.len / 1;
        this.controlLen1 = random(-this.controlFactor, this.controlFactor);
        this.controlLen2 = random(-this.controlFactor, this.controlFactor);
    }

    show() {
        push();

        noFill();
        stroke(255);

        const dir = p5.Vector.fromAngle(this.angle);

        const start = this.pos.copy();
        const end = p5.Vector.add(start, p5.Vector.mult(dir, this.len));

        const normal = p5.Vector.sub(end, start);
        normal.normalize();
        normal.rotate(HALF_PI);

        bezier(
            start.x, start.y, // anchor1
            start.x + normal.x*this.controlLen1, start.y + normal.y*this.controlLen1, // control1
            end.x, end.y, // anchor2
            end.x + normal.x*this.controlLen2, end.y + normal.y*this.controlLen2, // control2
        );
        // curve(
        //     start.x + normal.x*this.controlLen1, start.y + normal.y*this.controlLen1,
        //     start.x, start.y,
        //     end.x, end.y,
        //     end.x + normal.x*this.controlLen2, end.y + normal.y*this.controlLen2,
        // );

        pop();
    }
}

function draw() {
    const feather = new Feather();
    feather.show();
}
