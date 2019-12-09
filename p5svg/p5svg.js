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

function createCanvasSvg(width, height, maxElements) {
    removeP5Canvas();

    const svgElt = document.createElementNS(svgns, 'svg');
    svgElt.setAttribute('viewBox', '0 0 ' + width + ' ' + height);
    svgElt.setAttribute('height', height);
    svgElt.setAttribute('data-max-elements', maxElements);
    svgElt.id = 'canvasSvg';

    document.body.appendChild(svgElt);

    window.width = width;
    window.height = height;
}

// from https://stackoverflow.com/a/46403589
function saveSvg() {
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

function circleSvg(x, y, r, c) {
    const circleElt = document.createElementNS(svgns, 'circle');
    circleElt.setAttribute('cx', x);
    circleElt.setAttribute('cy', y);
    circleElt.setAttribute('r', r);

    circleElt.setAttribute('fill',
        'rgba(' +
        red(c) + ',' +
        green(c) + ',' +
        blue(c) + ',' +
        alpha(c) + ')');

    const svgElt = document.getElementById('canvasSvg');
    svgElt.appendChild(circleElt);

    limitSvgElements();
}
