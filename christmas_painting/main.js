let img;

function preload() {
    img = loadImage(args[0])
}

function setup() {
    createCanvas(img.width, img.height);
}

function draw() {
    image(img, 0, 0);
}