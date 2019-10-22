// Based on https://www.youtube.com/watch?v=S1TQCi9axzg

const SYMBOL_SIZE = 26;

let streams = [];

function setup() {
    createCanvas(window.innerWidth, window.innerHeight);

    textSize(SYMBOL_SIZE);

    let x = 0;
    for (const i of Array(round(width / SYMBOL_SIZE)).keys()) {
        const y = random(0, -1000);
        const stream = new Stream(x, y);
        streams.push(stream);

        x += SYMBOL_SIZE;
    }
}

class Symbol {
    constructor(x, y, speed, first) {
        this.x = x;
        this.y = y;
        this.speed = speed;
        this.bright = first && (random(0, 1) < 0.5);

        this.switchInterval = round(random(2, 20));
    }

    setToRandomSymbol() {
        this.value = String.fromCharCode(0x30a0 + round(random(0, 96)));
    }

    update() {
        this.y += this.speed;

        if (this.y > height) {
            this.y = 0;
        }

        if ((frameCount - 1) % this.switchInterval == 0) {
            this.setToRandomSymbol();
        }
    }

    draw() {
        if (this.bright) {
            fill(180, 255, 180);
        } else {
            fill(0, 255, 70);
        }

        text(this.value, this.x, this.y);
    }
}

class Stream {
    constructor(x, y) {
        this.symbols = [];
        this.speed = random(5, 10);

        this.generateSymbols(x, y);
    }

    generateSymbols(x, y) {
        const SIZE = round(random(5, 30));
        let first = true;
        for (let i of Array(SIZE).keys()) {
            const symbol = new Symbol(x, y, this.speed, first);
            this.symbols.push(symbol);

            y -= SYMBOL_SIZE;
            first = false;
        }
    }

    update() {
        for (const symbol of this.symbols) {
            symbol.update();
        }
    }

    draw() {
        for (const symbol of this.symbols) {
            symbol.draw();
        }
    }
}

function draw() {
    background(0);

    for (const stream of streams) {
        stream.update();
        stream.draw();
    }
}
