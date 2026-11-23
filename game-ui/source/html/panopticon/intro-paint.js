(function (window) {
  'use strict';

  const SPRAY_CROP = [158, 154, 821, 821];
  const TRACE_CROP = [3, 7, 417, 92];
  const INK_COLOR = '#0b176b';
  const RIM_COLOR = '#c4c6d3';
  const SPRAY_MASK_SIZE = 192;
  const SPRAY_SPAN = [98.4, 156];
  const TRACE_SPAN = [469.8, 664.8];

  // Номер кадра при 60 кадрах/с и глубина напыления через каждые 10% ширины.
  // Паузы совпадают с концами отдельных проходов. Координаты заданы без рисунка внутри маски.
  const sprayFronts = [
    [99,  -12, -12, -12, -12, -12, -12,  -8,  -7, -10, -12, -12],
    [100, -12, -12, -12, -12, -10,  -6,   1,   5,  -2, -10, -12],
    [102, -12, -12, -10,  -7,  10,  16,  13,   9,   5,  -1,  -6],
    [104, -10,   3,  19,  21,  20,  17,  14,   9,   5,  -1,  -6],
    [108,  42,  37,  32,  27,  23,  18,  15,  10,   5,  -1,  -6],
    [112,  42,  37,  32,  27,  23,  18,  15,  10,   5,  -1,  -6],
    [114,  74,  48,  32,  27,  23,  18,  15,  10,   5,  -1,  -6],
    [117,  76,  72,  68,  57,  33,  19,  15,  10,   5,  -1,  -6],
    [120,  76,  72,  68,  64,  60,  55,  45,  32,  11,  -1,  -6],
    [123,  76,  72,  68,  64,  60,  56,  51,  47,  41,  36,  21],
    [126,  76,  72,  68,  64,  60,  56,  51,  47,  42,  38,  33],
    [129,  76,  72,  68,  64,  60,  56,  51,  47,  42,  38,  33],
    [132,  76,  72,  68,  64,  60,  56,  51,  47,  43,  46,  56],
    [135,  76,  72,  68,  64,  60,  56,  54,  57,  66,  65,  62],
    [138,  76,  72,  68,  64,  79,  88,  83,  82,  71,  67,  62],
    [141,  77,  74,  81,  96,  93,  88,  83,  79,  72,  67,  62],
    [144,  95, 106, 105, 100,  94,  88,  83,  78,  72,  67,  62],
    [147,  95, 106, 105, 100,  94,  88,  83,  78,  72,  67,  62],
    [150,  95, 107, 110, 111, 108, 101,  96,  85,  74,  67,  62],
    [153,  99, 108, 111, 111, 110, 110, 109, 105,  97,  81,  66],
    [155, 103, 110, 111, 111, 111, 111, 111, 110, 108, 102,  95],
    [156, 110, 110, 111, 111, 111, 111, 111, 111, 111, 110, 108]
  ];

  // Границы мазков проведены по диагоналям заливки, без внутренних линий референса.
  const inkMarks = [
    { start: 469, end: 480, from: -0.03, to: 1.08, slant: -0.54,
      points: [[-0.05, -0.05], [1.05, -0.05], [1.05, 0.17], [0.77, 0.19], [0.52, 0.16], [0.34, 0.23], [-0.05, 0.23]] },
    { start: 485, end: 498, from: 1.03, to: -0.14, slant: 0.42,
      points: [[-0.05, 0.12], [1.05, 0.13], [1.05, 0.27], [0.58, 0.34], [-0.05, 0.44]] },
    { start: 505, end: 516, from: -0.11, to: 1.12, slant: -0.40,
      points: [[-0.05, 0.31], [1.05, 0.53], [1.05, 0.79], [-0.05, 0.50]] },
    { start: 526, end: 540, from: 1.14, to: -0.18, slant: 0.50,
      points: [[-0.05, 0.70], [1.05, 0.51], [1.05, 0.78], [-0.05, 0.95]] },
    { start: 541, end: 552, from: -0.08, to: 1.18, slant: -0.36,
      points: [[-0.05, 0.89], [0.37, 0.86], [0.58, 0.93], [0.78, 0.81], [1.05, 0.75], [1.05, 1.05], [-0.05, 1.05]] },
    { start: 574, end: 583, from: -0.24, to: 1.20, slant: -0.78,
      points: [[-0.05, -0.05], [1.05, -0.05], [1.05, 1.05], [-0.05, 1.05]] }
  ];

  function fraction(value, start, end) {
    return Math.min(1, Math.max(0, (value - start) / (end - start)));
  }

  function fade(value, start, end) {
    const part = fraction(value, start, end);
    return part * part * (3 - 2 * part);
  }

  function makeLayer(width, height) {
    const canvas = document.createElement('canvas');
    canvas.width = width;
    canvas.height = height;
    return canvas.getContext('2d');
  }

  // Верхняя полоса проходит по вершинам букв и высокой эмблеме над ними.
  function inkRow(row) {
    return row <= 0.17 ? row * 0.70 / 0.17 : 0.70 + (row - 0.17) * 0.30 / 0.83;
  }

  function drawSprayMask(context, frame) {
    let index = 1;
    while (index < sprayFronts.length - 1 && frame > sprayFronts[index][0]) index++;
    const before = sprayFronts[index - 1];
    const after = sprayFronts[index];
    const part = fraction(frame, before[0], after[0]);
    const size = SPRAY_MASK_SIZE;
    const front = [];
    for (let column = 1; column < before.length; column++) {
      front.push((before[column] + (after[column] - before[column]) * part) * size / 100);
    }

    context.clearRect(0, 0, size, size);
    context.beginPath();
    context.moveTo(-16, -30);
    context.lineTo(-16, front[0]);
    context.lineTo(0, front[0]);
    for (let column = 1; column < front.length; column++) {
      const x = size * column / (front.length - 1);
      const previousX = size * (column - 1) / (front.length - 1);
      context.quadraticCurveTo(previousX, front[column - 1], (previousX + x) / 2, (front[column - 1] + front[column]) / 2);
    }
    context.lineTo(size, front[front.length - 1]);
    context.lineTo(size + 16, front[front.length - 1]);
    context.lineTo(size + 16, -30);
    context.closePath();
    context.fill();
  }

  function drawInkMark(context, mark, frame, width, height) {
    if (frame <= mark.start) return;

    context.save();
    if (frame < mark.end) {
      const part = fraction(frame, mark.start, mark.end);
      const front = mark.from + (mark.to - mark.from) * part;
      const farEdge = mark.to > mark.from ? -1 : 2;
      context.beginPath();
      context.moveTo(farEdge * width, 0);
      context.lineTo((front - mark.slant / 2) * width, 0);
      context.lineTo((front + mark.slant / 2) * width, height);
      context.lineTo(farEdge * width, height);
      context.closePath();
      context.clip();
    }

    context.beginPath();
    mark.points.forEach(function (point, index) {
      const x = point[0] * width;
      const y = inkRow(point[1]) * height;
      if (index === 0) context.moveTo(x, y);
      else context.lineTo(x, y);
    });
    context.closePath();
    context.fill();
    context.restore();
  }

  window.panopticonIntroPaint = function (canvas, image, mode) {
    const context = canvas.getContext('2d');
    if (!context) return false;

    const spray = mode === 'spray';
    const crop = spray ? SPRAY_CROP : TRACE_CROP;
    const span = spray ? SPRAY_SPAN : TRACE_SPAN;
    let width = 0;
    let height = 0;
    let artwork;
    let mask;
    let stage;
    let rim;
    let crest;

    function drawOriginal(target, x, y, drawWidth = width, drawHeight = height) {
      target.drawImage(image, crop[0], crop[1], crop[2], crop[3], x, y, drawWidth, drawHeight);
    }

    function resize(nextWidth, nextHeight) {
      width = nextWidth;
      height = nextHeight;
      artwork = makeLayer(width, height);
      stage = makeLayer(width, height);
      drawOriginal(artwork, 0, 0);
      if (spray) {
        mask = makeLayer(SPRAY_MASK_SIZE, SPRAY_MASK_SIZE);
        mask.filter = 'blur(3.8px)';
        return;
      }
      mask = makeLayer(width, height);
      artwork.globalCompositeOperation = 'source-in';
      artwork.fillStyle = INK_COLOR;
      artwork.fillRect(0, 0, width, height);

      // Контур — кромка альфа-канала того же PNG, включая отверстия в буквах.
      rim = makeLayer(width, height);
      drawOriginal(rim, 0, 0);
      drawOriginal(stage, 0, 0);
      const edge = Math.max(1, Math.round(height * 0.014));
      stage.globalCompositeOperation = 'destination-in';
      drawOriginal(stage, -edge, 0);
      drawOriginal(stage, edge, 0);
      drawOriginal(stage, 0, -edge);
      drawOriginal(stage, 0, edge);
      stage.globalCompositeOperation = 'source-over';

      rim.globalCompositeOperation = 'destination-out';
      rim.drawImage(stage.canvas, 0, 0);
      rim.globalCompositeOperation = 'source-in';
      rim.fillStyle = RIM_COLOR;
      rim.fillRect(0, 0, width, height);

      // Верхняя эмблема светлеет отдельно перед проявлением остального знака.
      crest = makeLayer(width, height);
      drawOriginal(crest, 0, 0);
      const crestMask = crest.createLinearGradient(0, height * 0.47, 0, height * 0.66);
      crestMask.addColorStop(0, '#000');
      crestMask.addColorStop(1, 'transparent');
      crest.globalCompositeOperation = 'destination-in';
      crest.fillStyle = crestMask;
      crest.fillRect(0, 0, width, height);
    }

    function drawMasked(x, y) {
      stage.clearRect(0, 0, width, height);
      stage.drawImage(artwork.canvas, 0, 0);
      stage.globalCompositeOperation = 'destination-in';
      stage.drawImage(mask.canvas, 0, 0, width, height);
      stage.globalCompositeOperation = 'source-over';
      context.drawImage(stage.canvas, x, y);
    }

    function paintSpray(frame, x, y) {
      drawSprayMask(mask, frame);
      drawMasked(x, y);
    }

    function paintTrace(frame, x, y) {
      if (frame < 583) {
        mask.clearRect(0, 0, width, height);
        inkMarks.forEach(function (mark) {
          drawInkMark(mask, mark, frame, width, height);
        });
        drawMasked(x, y);
      } else {
        context.drawImage(artwork.canvas, x, y);
      }

      const rimPart = fraction(frame, 622, 648);
      if (rimPart > 0) {
        context.save();
        context.beginPath();
        context.moveTo(x, y);
        context.lineTo(x + width * (rimPart * 1.15), y);
        context.lineTo(x + width * (rimPart * 1.15 - 0.15), y + height);
        context.lineTo(x, y + height);
        context.closePath();
        context.clip();
        context.globalAlpha = fade(frame, 622, 648) * 0.75;
        context.drawImage(rim.canvas, x, y);
        context.restore();
      }

      if (frame > 611) {
        context.globalAlpha = fade(frame, 611, 664);
        context.drawImage(crest.canvas, x, y);
      }
      if (frame > 648) {
        context.globalAlpha = fade(frame, 648, TRACE_SPAN[1]);
        drawOriginal(context, x, y);
      }
    }

    return function draw(progress) {
      const bounds = canvas.getBoundingClientRect();
      const ratio = window.devicePixelRatio || 1;
      const pixelWidth = Math.round(bounds.width * ratio);
      const pixelHeight = Math.round(bounds.height * ratio);
      if (!pixelWidth || !pixelHeight) return;

      if (canvas.width !== pixelWidth) canvas.width = pixelWidth;
      if (canvas.height !== pixelHeight) canvas.height = pixelHeight;
      const scale = Math.min(pixelWidth / crop[2], pixelHeight / crop[3]);
      const nextWidth = Math.max(1, Math.round(crop[2] * scale));
      const nextHeight = Math.max(1, Math.round(crop[3] * scale));
      const x = Math.floor((pixelWidth - nextWidth) / 2);
      const y = Math.floor((pixelHeight - nextHeight) / 2);

      context.globalAlpha = 1;
      context.clearRect(0, 0, pixelWidth, pixelHeight);
      if (progress <= 0) return;
      if (progress >= 1 && !spray) {
        drawOriginal(context, x, y, nextWidth, nextHeight);
        return;
      }

      if (!artwork || artwork.canvas.width !== nextWidth || artwork.canvas.height !== nextHeight) {
        resize(nextWidth, nextHeight);
      }
      const frame = span[0] + (span[1] - span[0]) * progress;
      if (progress >= 1) context.drawImage(artwork.canvas, x, y);
      else if (spray) paintSpray(frame, x, y);
      else paintTrace(frame, x, y);
      context.globalAlpha = 1;
    };
  };
})(window);
