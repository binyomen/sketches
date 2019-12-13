'use strict';

const svgns = 'http://www.w3.org/2000/svg';

function removeP5Canvas() {
    for (const elt of document.getElementsByTagName('canvas')) {
        elt.parentNode.removeChild(elt);
    }
}

function limitSvgElements() {
    const svgElt = document.getElementById('canvasSvg');
    const maxElements = svgElt.getAttribute('data-max-elements');

    while (svgElt.childElementCount > maxElements) {
        svgElt.removeChild(svgElt.firstChild);
    }
}

function createCanvasSvg(width, height, renderer) {
    if (SVG_MODE) {
        removeP5Canvas();

        const svgElt = document.createElementNS(svgns, 'svg');
        svgElt.setAttribute('viewBox', '0 0 ' + width + ' ' + height);
        svgElt.setAttribute('height', height);
        svgElt.id = 'canvasSvg';

        document.body.appendChild(svgElt);

        window.width = width;
        window.height = height;
    } else {
        createCanvas(width, height, renderer);
    }
}

function setMaxElementsSvg(maxElements) {
    if (SVG_MODE) {
        const svgElt = document.getElementById('canvasSvg');
        svgElt.setAttribute('data-max-elements', maxElements);
    }
}

// from https://stackoverflow.com/a/46403589
function saveSvg() {
    if (SVG_MODE) {
        const svgElt = document.getElementById('canvasSvg');
        svgElt.setAttribute('xmlns', svgns);
        const svgData = svgElt.outerHTML;

        const preface = '<?xml version="1.0" standalone="no"?>\n';
        const svgBlob = new Blob([preface, svgData], { type: 'image/svg+xml;charset=utf-8' });
        const svgUrl = URL.createObjectURL(svgBlob);

        const downloadLink = document.createElement("a");
        downloadLink.href = svgUrl;
        downloadLink.download = 'sketch.svg';

        document.body.appendChild(downloadLink);
        downloadLink.click();
        document.body.removeChild(downloadLink);
    }
}

function circleSvg(x, y, r, c) {
    if (SVG_MODE) {
        const circleElt = document.createElementNS(svgns, 'circle');
        circleElt.setAttribute('cx', x);
        circleElt.setAttribute('cy', y);
        circleElt.setAttribute('r', r);

        circleElt.setAttribute('fill',
            'rgba(' +
            red(c) + ',' +
            green(c) + ',' +
            blue(c) + ',' +
            alpha(c)/255 + ')');

        const svgElt = document.getElementById('canvasSvg');
        svgElt.appendChild(circleElt);

        limitSvgElements();
    } else {
        push();

        noStroke();
        fill(c);
        circle(x, y, r);

        pop();
    }
}

function lineSvg(x1, y1, x2, y2, weight, c) {
    if (SVG_MODE) {
        const lineElt = document.createElementNS(svgns, 'line');
        lineElt.setAttribute('x1', x1);
        lineElt.setAttribute('y1', y1);
        lineElt.setAttribute('x2', x2);
        lineElt.setAttribute('y2', y2);

        lineElt.setAttribute('stroke',
            'rgba(' +
            red(c) + ',' +
            green(c) + ',' +
            blue(c) + ',' +
            alpha(c)/255 + ')');
        lineElt.setAttribute('stroke-width', weight);
        lineElt.setAttribute('stroke-linecap', 'round');

        const svgElt = document.getElementById('canvasSvg');
        svgElt.appendChild(lineElt);

        limitSvgElements();
    } else {
        push();

        strokeWeight(weight);
        stroke(c);
        line(x1, y1, x2, y2);

        pop();
    }
}
