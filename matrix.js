// Emulating falling code from https://www.youtube.com/watch?v=E6kcwwIQsN4

const TEXT_SIZE = 30;
const TEXT_OVERFILL = 5;

const SPEED_MIN = 10;
const SPEED_MAX = 100;

const TRAIL_MIN = 8;
const TRAIL_MAX = 14;

const DECAY_MAX = 50;

let fr;
let grid;

function setup() {
    createCanvas(windowWidth, windowHeight);

    textSize(TEXT_SIZE + TEXT_OVERFILL);
    textAlign(LEFT, TOP);

    const w = round(width/TEXT_SIZE);
    const h = round(height/TEXT_SIZE);
    grid = new Grid(w, h);

    for (const x of Array(w).keys()) {
        grid.spawnDropper(x, round(random(-30, 0)));
    }

    fr = createP('');
}

class Dropper {
    constructor(x, y, grid) {
        this.x = x;
        this.y = y;
        this.grid = grid;

        this.symbols = [];
        this.time = 0;

        this.speed = random(SPEED_MIN, SPEED_MAX);
        this.trailLength = round(random(TRAIL_MIN, TRAIL_MAX));
    }

    update() {
        this.time += deltaTime;

        if (this.time > this.speed) {
            this.time = this.time - this.speed;

            const prevSymbol = this.grid.getSymbol(this.x, this.y);
            if (this.y >= 0 && prevSymbol) {
                prevSymbol.setBright(false);
            }

            this.y += 1;
            if (this.y >= this.grid.getHeight()) {
                this.y = 0;
            }

            if (this.y >= 0) {
                const newSymbol = this.grid.createSymbol(this.x, this.y);
                newSymbol.setBright(true);
                this.symbols.push(newSymbol);

                if (this.symbols.length > this.trailLength) {
                    this.grid.removeSymbol(this.symbols[0].x, this.symbols[0].y);
                    this.symbols.shift();
                }
            }
        }
    }
}

class Symbol {
    constructor(x, y) {
        this.x = x;
        this.y = y;
        this.bright = false;

        this.setRandomChar();
    }

    setRandomChar() {
        this.char = String.fromCharCode(0x30a0 + round(random(0, 95)));
    }

    setBright(bright) {
        this.bright = bright;
    }

    getCoordinates() {
        return [this.x*TEXT_SIZE, this.y*TEXT_SIZE];
    }

    update() {
        if (random(0, 1) < 0.001) {
            this.setRandomChar();
        }
    }

    draw() {
        push();

        // const alpha = map(this.decay, 0, DECAY_MAX, 0, 255);
        const alpha = 255;
        if (this.bright) {
            fill(180, 255, 180, alpha);
        } else {
            fill(0, 255, 70, alpha);
        }

        const [x, y] = this.getCoordinates();
        text(this.char, x, y);

        pop();
    }
}

class Grid {
    constructor(w, h) {
        this.w = w;
        this.h = h;

        this.dict = {};
        this.droppers = [];
    }

    getSymbol(x, y) {
        if (x in this.dict && y in this.dict[x]) {
            return this.dict[x][y];
        } else {
            return null;
        }
    }

    createSymbol(x, y) {
        if (!(x in this.dict)) {
            this.dict[x] = {};
        }

        this.dict[x][y] = new Symbol(x, y);
        return this.dict[x][y];
    }

    removeSymbol(x, y) {
        delete this.dict[x][y];
    }

    getHeight() {
        return this.h;
    }

    spawnDropper(x, y) {
        this.droppers.push(new Dropper(x, y, this));
    }

    update() {
        for (const x in this.dict) {
            for (const y in this.dict[x]) {
                this.dict[x][y].update();
            }
        }

        for (const dropper of this.droppers) {
            dropper.update();
        }
    }

    draw() {
        for (const x in this.dict) {
            for (const y in this.dict[x]) {
                this.dict[x][y].draw();
            }
        }
    }
}

function draw() {
    background(0);

    grid.update();
    grid.draw();

    if (frameCount % 5 == 0) {
        fr.html('FPS: ' + floor(frameRate()));
    }
}
