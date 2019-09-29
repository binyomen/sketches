let w = 700;
let h = 700;

const CODE_START = h/20;
const CODE_END = h - h/20;
const LINES_OF_CODE = 30;
const LINE_SEP = (CODE_END - CODE_START) / LINES_OF_CODE;

function setup() {
    createCanvas(w, h);
    background(30);

    strokeWeight(10);
    strokeCap(ROUND);

    noLoop();
}

function randomColor() {
    // Palette is from https://github.com/taniarascia/new-moon/.
    const PALETTE = [
        color(229, 115, 118),
        color(235, 167, 114),
        color(114, 178, 241),
        color(211, 173, 223),
        color(170, 198, 166),
    ];

    return PALETTE[parseInt(random(PALETTE.length))];
}

function draw() {
    let lineY = CODE_START;
    let indent = 0;

    for (const i of Array(LINES_OF_CODE).keys()) {
        if (indent > 0 || random(1) < 0.9) {
            let lineX = 50 + indent*50;
            const lineSegments = parseInt(random(2, 8));

            stroke(randomColor());

            for (const j of Array(lineSegments).keys()) {
                if (random(1) < 0.7) {
                    stroke(randomColor());
                }

                const segmentLength = random(10, 80);
                line(lineX, lineY, lineX + segmentLength, lineY);

                lineX += segmentLength + 20;
            }

            if (indent < 5 && random(1) < 0.2) {
                ++indent;
            } else if (indent > 0 && random(1) < 0.5) {
                --indent;
            }

        }

        lineY += LINE_SEP;
    }
}
