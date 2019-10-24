const SQUARE_SIZE = 312;
const DRAW_RATE = 3;

const squaresToFill = [];

function setup() {
    createCanvas(1536, 864);

    background(30);

    const w = round(width / SQUARE_SIZE);
    const h = round(height / SQUARE_SIZE);
    for (const x of Array(w).keys()) {
        for (const y of Array(h).keys()) {
            squaresToFill.push([x, y]);
        }
    }
}

class Square {
    constructor(x, y) {
        this.x = x;
        this.y = y;

        this.color = color(random(70, 255), random(70, 255), random(70, 255), 200);
    }

    draw() {
        push();

        fill(this.color);
        square(this.x*SQUARE_SIZE, this.y*SQUARE_SIZE, SQUARE_SIZE);

        pop();
    }
}

function draw() {
    if (frameCount % DRAW_RATE == 0 && squaresToFill.length > 0) {
        const index = round(random(0, squaresToFill.length-1));
        const x = squaresToFill[index][0];
        const y = squaresToFill[index][1];

        const square = new Square(x, y);
        square.draw();

        squaresToFill.splice(index, 1);
    }
}
