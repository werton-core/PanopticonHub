(function () {
    'use strict';

    var mode = 'idle';
    var gain = null;
    var panel, meter, cover, peakLine, reading, slider, value, note;
    var heldPeak = 0;
    var holdUntil = 0;
    var showLevel = false;
    var statuses = {
        active: 'Прослушивание микрофона',
        connected: 'Проверка доступна в главном меню после отключения от сервера.',
        unavailable: 'Движок Garry’s Mod не смог запустить проверку микрофона.'
    };

    function run(command, ...args) {
        if (window.lua && typeof window.lua.Run === 'function') window.lua.Run(command, ...args);
    }

    function createPanel() {
        var style = document.createElement('style');
        style.textContent = [
            '.pt-voice{min-width:0;border:1px solid var(--pt-accent-border,#655b47);border-radius:12px;padding:16px;background:#100e0ee8;color:#fffaef}',
            '.pt-voice[hidden],.pt-voice [hidden]{display:none!important}',
            '.pt-voice__head,.pt-voice__monitor{display:flex;align-items:center;justify-content:space-between;gap:10px;font-size:17px;font-weight:600}',
            '.pt-voice__note{margin:0;font-size:17px;line-height:1.4;color:var(--pt-accent-text,#e4c69b)}',
            '.pt-voice__number{white-space:nowrap;color:var(--pt-accent-secondary,#d9cbb2);font-variant-numeric:tabular-nums}',
            '.pt-voice__meter{height:12px;position:relative;overflow:hidden;margin-top:10px;border-radius:3px;background:linear-gradient(90deg,#339a64 0%,#4fb67b 72%,#e5b963 88%,#e27568 100%)}',
            '.pt-voice__cover{position:absolute;top:0;right:0;bottom:0;background:#322b28;width:100%}',
            '.pt-voice__peak{position:absolute;top:0;bottom:0;width:2px;background:rgb(var(--pt-accent-pale-rgb,255,238,210));left:0}',
            '.pt-voice__scale{position:relative;height:20px;margin:5px 0 9px;color:var(--pt-accent-secondary,#b5a995);font-size:12px;font-variant-numeric:tabular-nums}',
            '.pt-voice__scale span{position:absolute;transform:translateX(-50%)}',
            '.pt-voice__scale span:first-child{transform:none}.pt-voice__scale span:last-child{transform:translateX(-100%)}',
            '.pt-voice__slider{display:block;width:100%;margin:10px 0 1px;appearance:none;background:var(--pt-accent-wash,#44382d);height:5px;border-radius:4px;cursor:pointer}',
            '.pt-voice__slider::-webkit-slider-thumb{appearance:none;width:14px;height:14px;border-radius:50%;background:var(--pt-accent-text,#ebcb7c)}',
            '.pt-voice__slider:focus-visible{outline:2px solid var(--pt-accent-text,#ebcb7c);outline-offset:5px}'
        ].join('\n');
        document.head.appendChild(style);

        panel = document.createElement('section');
        panel.className = 'pt-voice';
        panel.setAttribute('aria-label', 'Проверка микрофона');
        panel.innerHTML = [
            '<div class="pt-voice__levels">',
            '<div class="pt-voice__head"><span>Уровень микрофона</span><span class="pt-voice__number" aria-hidden="true"></span></div>',
            '<div class="pt-voice__meter" role="meter" aria-label="Уровень микрофона" aria-valuemin="0" aria-valuemax="100">',
            '<div class="pt-voice__cover"></div><div class="pt-voice__peak"></div></div>',
            '<div class="pt-voice__scale" aria-hidden="true"></div>',
            '<label class="pt-voice__monitor" for="pt-voice-monitor">Громкость передачи <span class="pt-voice__number"></span></label>',
            '<input id="pt-voice-monitor" class="pt-voice__slider" type="range" min="0" max="100" step="1">',
            '</div><div class="pt-voice__note" role="status"></div>'
        ].join('');
        meter = panel.querySelector('.pt-voice__meter');
        cover = panel.querySelector('.pt-voice__cover');
        peakLine = panel.querySelector('.pt-voice__peak');
        reading = panel.querySelector('.pt-voice__head .pt-voice__number');
        slider = panel.querySelector('.pt-voice__slider');
        value = panel.querySelector('.pt-voice__monitor .pt-voice__number');
        note = panel.querySelector('.pt-voice__note');
        [0, 25, 50, 75, 100].forEach(function (percent) {
            var mark = document.createElement('span');
            mark.textContent = String(percent);
            mark.style.left = percent + '%';
            panel.querySelector('.pt-voice__scale').appendChild(mark);
        });
        slider.addEventListener('input', function () { setVolume(false); });
        slider.addEventListener('change', function () { setVolume(true); });
        showMeter(0);
    }

    function setVolume(commit) {
        gain = Number(slider.value);
        value.textContent = gain + '%';
        run('stMenu.SetMicrophoneVolume(%d,%d)', gain, commit ? 1 : 0);
    }

    function showMeter(level) {
        if (!meter) return;
        var percent = Math.min(100, Math.max(0, (Number(level) || 0) * 100));
        var now = performance.now();
        if (percent >= heldPeak) {
            heldPeak = percent;
            holdUntil = now + 350;
        } else if (now > holdUntil) {
            heldPeak = Math.max(percent, heldPeak - 3);
        }
        var rounded = Math.round(percent);
        if (reading.textContent !== rounded + '%') reading.textContent = rounded + '%';
        cover.style.width = (100 - percent) + '%';
        peakLine.style.left = 'calc(' + heldPeak + '% - 1px)';
        meter.setAttribute('aria-valuenow', rounded);
    }

    function syncPanel() {
        var details = document.querySelector('.settings-page__details');
        if (!details) {
            if (mode === 'active' && showLevel) {
                showLevel = false;
                run('stMenu.SetMicrophoneTest(false)');
            }
            return;
        }
        if (mode === 'idle' && !panel) return;
        if (!panel) createPanel();
        if (panel.parentNode !== details) details.appendChild(panel);
        showLevel = mode === 'active';
        panel.hidden = mode === 'idle';
        panel.querySelector('.pt-voice__levels').hidden = !showLevel;
        note.hidden = showLevel;
        if (!showLevel && note.textContent !== (statuses[mode] || '')) note.textContent = statuses[mode] || '';
    }

    window.PTVoiceTestState = function (next, percentValue) {
        mode = Object.prototype.hasOwnProperty.call(statuses, next) ? next : 'idle';
        syncPanel();
        window.PTVoiceTestVolume(percentValue);
        heldPeak = 0;
        showMeter(0);
    };
    window.PTVoiceTestVolume = function (percentValue) {
        gain = typeof percentValue === 'number' ? Math.min(100, Math.max(0, percentValue)) : null;
        if (slider) {
            slider.disabled = gain === null;
            if (gain !== null) slider.value = gain;
            value.textContent = gain === null ? 'Недоступно' : gain + '%';
        }
    };
    window.PTVoiceTestFrame = function (level) {
        if (showLevel) showMeter(level);
    };

    new MutationObserver(syncPanel).observe(document.documentElement, { childList: true, subtree: true });
})();
