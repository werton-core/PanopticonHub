"""Сборка вступления из логотипов проекта, кадровой анимации и исходной дорожки."""

import argparse
import base64
from pathlib import Path
import subprocess
import tempfile

from playwright.sync_api import sync_playwright


ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / 'game-ui/source/html/panopticon'
FONT = ROOT / 'game-ui/dist/html/assets/montserrat-v18-latin_cyrillic-700.woff2'
FPS = 60
FRAME_COUNT = 972
MAX_BYTES = 8 * 1024 * 1024

RENDER = r'''async ({werton, mark, font, fps}) => {
    const introFont = await new FontFace('Intro', `url(${font})`).load();
    document.fonts.add(introFont);

    function canvas(width, height) {
        const layer = document.createElement('canvas');
        layer.width = width;
        layer.height = height;
        layer.style.width = `${width}px`;
        layer.style.height = `${height}px`;
        document.body.appendChild(layer);
        return layer;
    }

    async function artwork(source, width, height, mode) {
        const image = new Image();
        image.src = source;
        await image.decode();
        const layer = canvas(width, height);
        return {layer, draw: window.panopticonIntroPaint(layer, image, mode)};
    }

    const frame = canvas(1024, 1200);
    const ctx = frame.getContext('2d', {alpha: false});
    const author = await artwork(werton, 1024, 1024, 'spray');
    const warning = await artwork(mark, 1024, 226, 'trace');
    ctx.font = '700 64px Intro';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    ctx.letterSpacing = '2.24px';

    function progress(time, first, last) {
        return Math.min(1, Math.max(0, (time - first) / (last - first)));
    }

    // Яркость логотипа и подписи в исходном видеоряде, начиная с кадра 318.
    const logoFade = [
        255, 255, 255, 255, 254, 254, 252, 252, 250, 250, 246, 246, 240, 240,
        237, 237, 231, 230, 225, 225, 215, 215, 192, 192, 192, 192, 182, 182,
        171, 171, 158, 158, 146, 146, 129, 129, 115, 115, 99, 99, 84, 84,
        67, 67, 47, 47, 26, 26, 10, 10,
    ];
    const captionFade = [
        254, 254, 251, 251, 238, 238, 224, 223, 216, 216, 208, 208, 200, 200,
        191, 191, 183, 183, 174, 174, 161, 161, 135, 135, 135, 135, 118, 118,
        112, 112, 101, 101, 92, 92, 83, 83, 71, 71, 59, 59, 46, 46,
        32, 32, 19, 19, 5, 5,
    ];
    function opacity(curve, number) {
        return number < 318 ? 1 : (curve[number - 318] || 0) / 255;
    }

    let previous;
    window.renderFrame = number => {
        const t = number / fps;
        const spray = progress(t, 1.64, 2.60);
        const gold = opacity(logoFade, number);
        const caption = number >= 162 ? opacity(captionFade, number) : 0;
        const trace = progress(t, 7.83, 11.08);
        const key = JSON.stringify([spray, gold, caption, trace]);
        if (key === previous) return null;
        previous = key;

        ctx.globalAlpha = 1;
        ctx.fillStyle = '#000';
        ctx.fillRect(0, 0, frame.width, frame.height);
        if (spray > 0 && gold > 0) {
            author.draw(spray);
            ctx.globalAlpha = gold;
            ctx.drawImage(author.layer, 0, 0);
            ctx.globalAlpha = caption;
            ctx.fillStyle = '#f5f3f3';
            ctx.fillText('werton', 512, 1040);
        }
        if (trace > 0) {
            warning.draw(trace);
            ctx.drawImage(warning.layer, 0, 0);
        }
        return frame.toDataURL('image/png').split(',')[1];
    };
}'''


def data_url(path, mime):
    encoded = base64.b64encode(path.read_bytes()).decode('ascii')
    return f'data:{mime};base64,{encoded}'


def render(encoded, chrome):
    with sync_playwright() as playwright:
        launch = {'executable_path': chrome} if chrome else {'channel': 'chrome'}
        with playwright.chromium.launch(**launch) as browser:
            page = browser.new_page(device_scale_factor=1)
            page.add_script_tag(path=ASSETS / 'intro-paint.js')
            page.evaluate(RENDER, {
                'werton': data_url(ASSETS / 'intro-werton.png', 'image/png'),
                'mark': data_url(ASSETS / 'intro-panopticon.png', 'image/png'),
                'font': data_url(FONT, 'font/woff2'),
                'fps': FPS,
            })
            for number in range(FRAME_COUNT):
                png = page.evaluate('renderFrame', number)
                if png is not None:
                    frame = base64.b64decode(png)
                encoded.write(frame)
                if (number + 1) % (FPS * 4) == 0:
                    print(f'Кадры {number + 1}/{FRAME_COUNT}', flush=True)


def main():
    options = argparse.ArgumentParser(description=__doc__)
    options.add_argument('output', type=Path, help='Итоговый файл WebM')
    options.add_argument('--ffmpeg', default='ffmpeg', help='Путь к FFmpeg или команда из PATH')
    options.add_argument('--chrome', help='Путь к Chrome, если он установлен в нестандартном месте')
    args = options.parse_args()
    output = args.output.resolve()
    if output.suffix.lower() != '.webm':
        options.error('Итоговый файл должен иметь расширение .webm')
    output.parent.mkdir(parents=True, exist_ok=True)

    with tempfile.NamedTemporaryFile(dir=output.parent, suffix='.webm', delete=False) as temporary:
        staged = Path(temporary.name)
    try:
        command = [
            args.ffmpeg, '-hide_banner', '-loglevel', 'error', '-nostdin', '-y',
            '-f', 'image2pipe', '-framerate', str(FPS), '-vcodec', 'png', '-threads', '2', '-i', 'pipe:0',
            '-i', str(ASSETS / 'intro-audio.wav'), '-map', '0:v:0', '-map', '1:a:0',
            '-c:v', 'libvpx-vp9', '-deadline', 'good', '-cpu-used', '5', '-row-mt', '1', '-threads', '4',
            '-crf', '19', '-b:v', '0', '-pix_fmt', 'yuv420p', '-g', str(FPS),
            '-c:a', 'libopus', '-b:a', '256k', '-map_metadata', '-1', str(staged),
        ]
        with subprocess.Popen(command, stdin=subprocess.PIPE) as ffmpeg:
            with ffmpeg.stdin:
                render(ffmpeg.stdin, args.chrome)
            if ffmpeg.wait() != 0:
                raise RuntimeError('Не удалось собрать WebM')
        if staged.stat().st_size > MAX_BYTES:
            raise RuntimeError('Вступление превышает лимит игрового пакета в 8 МиБ')
        staged.replace(output)
        print(f'{output} · {FRAME_COUNT} кадров · {output.stat().st_size} байт', flush=True)
    finally:
        staged.unlink(missing_ok=True)


if __name__ == '__main__':
    main()
