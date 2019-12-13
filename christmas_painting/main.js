let img;
let system;

loadScript('/christmas_painting/Particle.js');

switch (args[1]) {
    case 'circlemovement': {
        loadScript('/christmas_painting/CircleMovementSystem.js');
        break;
    }
    case 'impressionist': {
        loadScript('/christmas_painting/ImpressionistSystem.js');
        break;
    }
    default: throw new Error('invalid system name: ' + args[1]);
}

function loadScript(path) {
    const scriptTag = document.createElement('script');
    scriptTag.setAttribute('src', path);
    document.head.appendChild(scriptTag);
}

function preload() {
    img = loadImage(args[0]);
}

function setup() {
    createCanvas(img.width, img.height);

    img.loadPixels();

    system = new System(img);

    background(255);
}

function draw() {
    system.update();
    system.draw();
}
