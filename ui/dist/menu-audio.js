(() => {
    'use strict';

    const storageKey = 'pt-hub-menu-audio-v1';
    const audioKey = /^[0-9a-f]{64}\.(?:mp3|wav|ogg)$/;
    const labels = {
        ru: ['Музыка в хабе', 'Громкость музыки', 'Включить музыку', 'Выключить музыку'],
        en: ['Hub music', 'Music volume', 'Unmute music', 'Mute music'],
        fr: ['Musique du hub', 'Volume de la musique', 'Activer la musique', 'Couper la musique'],
    };
    const speaker = 'M11 5 6 9H3v6h3l5 4V5z';
    const waves = 'M15 9a4 4 0 0 1 0 6m3-9a8 8 0 0 1 0 12';
    const cross = 'm16 9 6 6m0-6-6 6';
    const preferences = loadPreferences();
    let asset = null;
    let track;
    let control;
    let toggle;
    let icon;
    let range;
    let percent;
    let nativeVisible = false;
    let gotVisibility = false;
    let gotPid = false;
    let gamePid;
    let launching = false;
    let previewing = false;
    let pageHidden = false;
    let resume;

    function loadPreferences() {
        try {
            const saved = JSON.parse(localStorage.getItem(storageKey));
            if (saved && Number.isFinite(saved.volume) && saved.volume >= 0 && saved.volume <= 1) {
                return {volume: saved.volume, muted: saved.muted === true};
            }
        } catch (_) {
            // The current window can still remember the controls when storage is unavailable.
        }
        return {volume: .3, muted: false};
    }

    function savePreferences() {
        try { localStorage.setItem(storageKey, JSON.stringify(preferences)); } catch (_) {}
    }

    function element(name, className) {
        const node = document.createElement(name);
        if (className) node.className = className;
        return node;
    }

    function buildControl() {
        control = element('div', 'pt-hub-menu-audio no-drag-window');
        control.id = 'pt-hub-menu-audio';
        toggle = element('button', 'icon-btn pt-hub-menu-audio__toggle no-drag-window');
        toggle.type = 'button';
        toggle.innerHTML = '<svg class="ico" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path/></svg>';
        icon = toggle.querySelector('path');
        toggle.addEventListener('click', () => {
            preferences.muted = !preferences.muted;
            savePreferences();
            renderControl();
            updatePlayback();
        });

        const popover = element('div', 'pt-hub-menu-audio__popover');
        const panel = element('div', 'pt-hub-menu-audio__panel');
        const slider = element('div', 'pt-hub-menu-audio__slider');
        range = element('input', 'st-volume__range pt-hub-menu-audio__range');
        range.type = 'range';
        range.min = '0';
        range.max = '100';
        range.step = '1';
        range.setAttribute('aria-orientation', 'vertical');
        range.addEventListener('input', () => {
            preferences.volume = range.valueAsNumber / 100;
            savePreferences();
            renderControl();
            updatePlayback();
        });
        percent = element('span', 'pt-hub-menu-audio__percent');
        percent.setAttribute('aria-hidden', 'true');
        slider.append(range);
        panel.append(slider, percent);
        popover.append(panel);
        control.append(toggle, popover);
        renderControl();
    }

    function renderControl() {
        if (!control) return;
        const [music, volume, unmute, mute] = labels[document.documentElement.lang] || labels.en;
        const value = Math.round(preferences.volume * 100);
        toggle.setAttribute('aria-label', music);
        toggle.setAttribute('aria-pressed', String(!preferences.muted));
        toggle.title = preferences.muted ? unmute : mute;
        icon.setAttribute('d', speaker + (preferences.muted ? cross : waves));
        control.classList.toggle('is-muted', preferences.muted);
        range.disabled = preferences.muted;
        range.value = String(value);
        range.setAttribute('aria-label', volume);
        range.setAttribute('aria-valuetext', value + '%');
        range.style.setProperty('--st-volume', value + '%');
        percent.textContent = value + '%';
    }

    function syncControl() {
        const settings = document.querySelector('.topbar .settings');
        if (!asset || !settings) {
            control?.remove();
            return;
        }
        if (!control) buildControl();
        if (control.nextElementSibling !== settings) settings.before(control);
    }

    function canPlay() {
        return nativeVisible && gotPid && !gamePid && !launching && !document.hidden && !pageHidden &&
            !previewing && !preferences.muted && preferences.volume > 0;
    }

    function stopResume() {
        clearTimeout(resume);
        resume = undefined;
    }

    function updatePlayback() {
        if (!track) return;
        track.volume = preferences.volume;
        if (!canPlay()) {
            stopResume();
            track.pause();
            return;
        }
        if (!track.paused || resume !== undefined) return;
        // The host clears launch progress just before sending the new game PID.
        resume = setTimeout(() => {
            resume = undefined;
            if (track && canPlay()) track.play().catch(() => {});
        }, 100);
    }

    function apply(key) {
        const next = typeof key === 'string' && audioKey.test(key) ? key : null;
        if (asset === next) return;
        stopResume();
        if (track) {
            track.pause();
            track.removeAttribute('src');
            track.load();
            track = undefined;
        }
        asset = next;
        if (asset) {
            track = new Audio('/design-media/' + asset);
            track.loop = true;
            track.preload = 'metadata';
        }
        syncControl();
        updatePlayback();
    }

    function setPreviewing(active) {
        previewing = active === true;
        updatePlayback();
    }

    function start() {
        const root = document.getElementById('root');
        if (!root || !window.electron) return;
        new MutationObserver(syncControl).observe(root, {subtree: true, childList: true});
        new MutationObserver(renderControl).observe(document.documentElement, {attributeFilter: ['lang']});
        syncControl();

        function receiveVisibility(visible) {
            if (typeof visible !== 'boolean') return;
            gotVisibility = true;
            nativeVisible = visible;
            updatePlayback();
        }

        function receivePid(pid) {
            gotPid = true;
            gamePid = pid;
            updatePlayback();
        }

        window.electron.on('hubWindowVisible', receiveVisibility);
        window.electron.on('gmodUpdatePid', receivePid);
        window.electron.on('gmodLaunchState', (state) => {
            launching = state != null;
            updatePlayback();
        });
        window.electron.invoke('isHubWindowVisible').then((visible) => {
            if (!gotVisibility) receiveVisibility(visible);
        }).catch(() => {});
        window.electron.invoke('getGmodPid').then((pid) => {
            if (!gotPid) receivePid(pid);
        }).catch(() => {});
        document.addEventListener('visibilitychange', updatePlayback);
        window.addEventListener('pagehide', () => {
            pageHidden = true;
            updatePlayback();
        });
        window.addEventListener('pageshow', () => {
            pageHidden = false;
            updatePlayback();
        });
    }

    window.PTHubMenuAudio = Object.freeze({apply, setPreviewing});
    if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', start, {once: true});
    else start();
})();
