let img;
let system;

loadScript('/christmas_painting/Particle.js');
loadScript('/christmas_painting/CircleMovementSystem.js');

function loadScript(path) {
    const scriptTag = document.createElement('script');
    scriptTag.setAttribute('src', path);
    document.head.appendChild(scriptTag);
}

function preload() {
    img = loadImage(args[0]);
}

function setup() {
    createCanvasSvg(img.width, img.height);

    img.loadPixels();

    switch (args[1]) {
        case 'circlemovement': {
            system = new CircleMovementSystem(img);
            break;
        }
        default: throw new Error('invalid system name');
    }
}

function draw() {
    system.update();
    system.draw();
}