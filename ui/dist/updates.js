(function () {
    'use strict';

    const copy = {
        ru: {
            title: 'Обновления',
            mode: 'Режим обновлений',
            ask: 'Спрашивать',
            auto: 'Устанавливать автоматически',
            manual: 'Проверять вручную',
            installed: 'Установленная версия',
            check: 'Проверить обновления',
            checking: 'Проверяем обновления…',
            current: 'Установлена актуальная версия.',
            available: 'Доступно обновление',
            checkError: 'Не удалось проверить обновления. Повторите попытку.',
            loadError: 'Не удалось загрузить режим обновлений.',
            saveError: 'Не удалось сохранить режим обновлений.',
            install: 'Установить обновление',
            installing: 'Скачиваем и проверяем обновление…',
            started: 'Установщик запущен.',
            installError: 'Не удалось установить обновление. Повторите попытку.',
            gameRunning: 'Закройте Garry’s Mod, чтобы установить обновление.',
            unmanaged: 'Для обновлений установите Panopticon Hub через официальный установщик.',
            unavailable: 'Это обновление больше недоступно. Проверьте обновления ещё раз.',
            later: 'Позже',
            dontAsk: 'Больше не спрашивать',
        },
        en: {
            title: 'Updates',
            mode: 'Update mode',
            ask: 'Ask',
            auto: 'Install automatically',
            manual: 'Check manually',
            installed: 'Installed version',
            check: 'Check for updates',
            checking: 'Checking for updates…',
            current: 'You have the latest version.',
            available: 'Update available',
            checkError: 'Could not check for updates. Try again.',
            loadError: 'Could not load the update mode.',
            saveError: 'Could not save the update mode.',
            install: 'Install update',
            installing: 'Downloading and verifying the update…',
            started: 'The installer has started.',
            installError: 'Could not install the update. Try again.',
            gameRunning: 'Close Garry’s Mod to install the update.',
            unmanaged: 'To get updates, install Panopticon Hub using the official installer.',
            unavailable: 'This update is no longer available. Check for updates again.',
            later: 'Later',
            dontAsk: 'Don’t ask again',
        },
        fr: {
            title: 'Mises à jour',
            mode: 'Mode de mise à jour',
            ask: 'Demander',
            auto: 'Installer automatiquement',
            manual: 'Vérifier manuellement',
            installed: 'Version installée',
            check: 'Rechercher des mises à jour',
            checking: 'Recherche de mises à jour…',
            current: 'Vous avez la dernière version.',
            available: 'Mise à jour disponible',
            checkError: 'Impossible de rechercher des mises à jour. Réessayez.',
            loadError: 'Impossible de charger le mode de mise à jour.',
            saveError: 'Impossible d’enregistrer le mode de mise à jour.',
            install: 'Installer la mise à jour',
            installing: 'Téléchargement et vérification de la mise à jour…',
            started: 'Le programme d’installation a démarré.',
            installError: 'Impossible d’installer la mise à jour. Réessayez.',
            gameRunning: 'Fermez Garry’s Mod pour installer la mise à jour.',
            unmanaged: 'Pour recevoir les mises à jour, installez Panopticon Hub avec le programme d’installation officiel.',
            unavailable: 'Cette mise à jour n’est plus disponible. Relancez la recherche.',
            later: 'Plus tard',
            dontAsk: 'Ne plus demander',
        },
    };

    const native = window.electron;
    if (!native) return;

    const modes = ['ask', 'auto', 'manual'];
    const otherModals = '.modal-dim.open, .st-dialog-dim, dialog[open]';
    const errors = ['checkError', 'installError', 'unavailable', 'unmanaged', 'gameRunning', 'loadError', 'saveError'];
    const installErrors = new Map([
        ['UPDATE_GAME_RUNNING', 'gameRunning'],
        ['UPDATE_UNMANAGED', 'unmanaged'],
        ['UPDATE_NOT_AVAILABLE', 'unavailable'],
    ]);
    let mode, modeError, version, release, settings, prompt;
    let phase, savingMode = false, startupReady = false, startupChecked = false, backgroundOffer = false;
    let visible = false, running, launching = false;
    let gameRevision = 0, retryAfterGameRevision, gameSettleTimer;

    function element(tag, className) {
        const node = document.createElement(tag);
        if (className) node.className = className;
        return node;
    }

    function button(className, handler) {
        const node = element('button', className);
        node.type = 'button';
        node.addEventListener('click', handler);
        return node;
    }

    function setText(node, value) {
        if (node.textContent !== value) node.textContent = value;
    }

    function busy() {
        return phase === 'checking' || phase === 'installing' || phase === 'started';
    }

    function canInstall() {
        return release && !busy() && !savingMode && running === false && !launching
            && gameSettleTimer === undefined && phase !== 'unmanaged';
    }

    function showStatus(node, label, key) {
        setText(node, label || '');
        node.classList.toggle('is-error', errors.includes(key));
    }

    function render() {
        const lang = document.documentElement.lang.toLowerCase().split('-')[0];
        const labels = copy[lang] || copy.en;
        const releaseTitle = release ? labels.available + ' · ' + release.version : '';
        const gameBlocked = release && !busy() && (running || launching);
        const statusKey = gameBlocked ? 'gameRunning' : phase === 'available' ? undefined : phase;
        if (settings) {
            setText(settings.title, labels.title);
            setText(settings.label, labels.mode);
            for (const option of settings.select.options) setText(option, labels[option.value]);
            settings.select.value = mode || '';
            settings.select.disabled = !mode || savingMode || phase === 'installing' || phase === 'started';
            setText(settings.version, version ? labels.installed + ' ' + version : '');
            setText(settings.check, labels.check);
            settings.check.disabled = busy();
            setText(settings.releaseTitle, releaseTitle);
            setText(settings.notes, release?.body || '');
            settings.notes.hidden = !release?.body;
            setText(settings.install, labels.install);
            settings.install.hidden = !release;
            settings.install.disabled = !canInstall();
            showStatus(settings.modeStatus, labels[modeError], modeError);
            showStatus(settings.status, labels[statusKey], statusKey);
        }
        if (prompt) {
            setText(prompt.title, releaseTitle || labels.title);
            setText(prompt.notes, release?.body || '');
            prompt.notes.hidden = !release?.body;
            setText(prompt.label, labels.dontAsk);
            setText(prompt.later, labels.later);
            setText(prompt.install, labels.install);
            prompt.checkbox.disabled = savingMode || busy();
            prompt.later.disabled = savingMode || busy();
            prompt.install.disabled = !canInstall();
            const promptStatus = modeError || statusKey;
            showStatus(prompt.status, labels[promptStatus], promptStatus);
        }
    }

    async function saveMode(value) {
        if (value === mode) return true;
        savingMode = true;
        modeError = undefined;
        render();
        try {
            await native.invoke('updateSetMode', value);
            mode = value;
            if (mode === 'manual') backgroundOffer = false;
            return true;
        } catch (_) {
            modeError = 'saveError';
            return false;
        } finally {
            savingMode = false;
            render();
            tryBackground();
        }
    }

    async function check(background) {
        if (busy()) return;
        backgroundOffer = false;
        retryAfterGameRevision = undefined;
        release = undefined;
        phase = 'checking';
        render();
        try {
            const result = await native.invoke('updateCheck');
            if (result != null && (typeof result.version !== 'string' || !result.version)) throw new Error('Invalid update.');
            if (result) {
                release = {version: result.version, body: typeof result.body === 'string' ? result.body : ''};
                backgroundOffer = background && mode !== 'manual';
            }
            phase = release ? 'available' : 'current';
        } catch (_) {
            phase = 'checkError';
        }
        render();
        tryBackground();
    }

    async function install(automatic = false) {
        if (!canInstall()) return;
        const beforeInstall = gameRevision;
        backgroundOffer = false;
        phase = 'installing';
        render();
        try {
            await native.invoke('updateInstall');
            phase = 'started';
        } catch (error) {
            phase = installErrors.get(error?.code) || installErrors.get(error?.message) || 'installError';
            if (phase === 'unavailable') release = undefined;
            if (automatic && phase === 'gameRunning') {
                backgroundOffer = true;
                // The native scan may see a game before WebView receives a process event.
                if (!running && !launching && gameRevision === beforeInstall) retryAfterGameRevision = gameRevision;
            }
        }
        render();
        tryBackground();
    }

    function buildSettings(viewport) {
        const section = element('section', 'st-block pt-update-settings');
        const title = element('h3', 'st-block__head');
        const body = element('div', 'st-block__body');
        const row = element('div', 'st-set-row');
        const label = element('label', 'st-set-name');
        const select = element('select', 'st-dd__btn pt-update-mode');
        label.htmlFor = select.id = 'pt-update-mode';
        for (const value of modes) select.add(new Option('', value));
        select.addEventListener('change', () => { void saveMode(select.value); });
        row.append(label, select);

        const actionRow = element('div', 'st-set-row');
        const version = element('span', 'st-set-desc');
        const checkButton = button('a-btn ghost pt-update-button', () => check(false));
        actionRow.append(version, checkButton);
        const modeStatus = element('p', 'pt-update-status');
        const status = element('p', 'pt-update-status');
        modeStatus.setAttribute('role', 'status');
        status.setAttribute('role', 'status');
        const releaseTitle = element('p', 'st-set-name pt-update-release-title');
        releaseTitle.setAttribute('aria-live', 'polite');
        const notes = element('p', 'pt-update-notes');
        const installButton = button('a-btn pt-update-button', () => install());
        body.append(row, modeStatus, actionRow, releaseTitle, notes, status, installButton);
        section.append(title, body);
        viewport.append(section);
        return {viewport, section, title, label, select, version, check: checkButton, releaseTitle, notes, status, modeStatus, install: installButton};
    }

    function showPrompt() {
        const dialog = element('dialog', 'modal pt-update-dialog no-drag-window');
        const head = element('header', 'modal-head');
        const title = element('h2', 'modal-title');
        title.id = 'pt-update-title';
        title.tabIndex = -1;
        dialog.setAttribute('aria-labelledby', title.id);
        head.append(title);

        const body = element('div', 'modal-body');
        const notes = element('p', 'pt-update-notes');
        const choice = element('label', 'pt-update-choice');
        const checkbox = element('input');
        checkbox.type = 'checkbox';
        const label = element('span');
        choice.append(checkbox, label);
        const status = element('p', 'pt-update-status');
        status.setAttribute('role', 'status');
        body.append(notes, choice, status);

        async function choose(accept) {
            if (busy() || savingMode) return;
            if (checkbox.checked && !(await saveMode('manual'))) return;
            if (accept) await install();
            else dialog.close();
        }

        const actions = element('footer', 'pt-update-actions');
        const later = button('a-btn ghost pt-update-button', () => choose(false));
        const installButton = button('a-btn pt-update-button', () => choose(true));
        actions.append(later, installButton);
        dialog.append(head, body, actions);
        dialog.addEventListener('cancel', event => {
            event.preventDefault();
            void choose(false);
        });
        dialog.addEventListener('close', () => {
            prompt = undefined;
            modeError = undefined;
            dialog.remove();
            render();
        }, {once: true});
        prompt = {title, notes, label, checkbox, status, later, install: installButton};
        render();
        document.body.append(dialog);
        dialog.showModal();
        title.focus({preventScroll: true});
    }

    function tryBackground() {
        if (!startupReady || !mode || savingMode || !visible || document.hidden || running !== false || launching
            || gameSettleTimer !== undefined || retryAfterGameRevision === gameRevision
            || !document.querySelector('.hub') || document.querySelector(otherModals)) return;
        if (!startupChecked) {
            startupChecked = true;
            if (mode !== 'manual' && !phase) void check(true);
            return;
        }
        if (!backgroundOffer || !canInstall()) return;
        backgroundOffer = false;
        if (mode === 'ask') showPrompt();
        if (mode === 'auto') void install(true);
    }

    function gameStateChanged() {
        gameRevision++;
        clearTimeout(gameSettleTimer);
        gameSettleTimer = undefined;
        if (running !== false || launching) return;
        // Launch progress and the game PID arrive as separate events.
        gameSettleTimer = setTimeout(() => {
            gameSettleTimer = undefined;
            if (phase === 'gameRunning') phase = 'available';
            render();
            tryBackground();
        }, 120);
    }

    window.panopticonUpdatesReady = async function () {
        if (startupReady) return;
        try {
            const consent = await native.invoke('store', 'get', 'legalConsent');
            if (!consent?.acceptedAt) return;
            startupReady = true;
            tryBackground();
        } catch (_) {
            // A later first-run callback may succeed.
        }
    };

    function syncDom() {
        const viewport = document.querySelector('.modal.m-settings > .modal-body');
        if (!viewport) {
            settings = undefined;
        } else if (settings?.viewport !== viewport || !viewport.contains(settings.section)) {
            settings = buildSettings(viewport);
            render();
        }
        tryBackground();
    }

    function bindSnapshot(channel, event, apply, fallback) {
        let received = false;
        function update(value) {
            received = true;
            apply(value);
            render();
            tryBackground();
        }
        native.on(event, update);
        native.invoke(channel).then(value => {
            if (!received) update(value);
        }).catch(() => {
            if (!received) update(fallback);
        });
    }

    function start() {
        new MutationObserver(syncDom).observe(document.body, {
            childList: true, subtree: true, attributes: true, attributeFilter: ['open'],
        });
        new MutationObserver(render).observe(document.documentElement, {attributes: true, attributeFilter: ['lang']});
        document.addEventListener('visibilitychange', tryBackground);
        bindSnapshot('getGmodPid', 'gmodUpdatePid', value => {
            running = Boolean(value);
            gameStateChanged();
        }, undefined);
        bindSnapshot('isHubWindowVisible', 'hubWindowVisible', value => { visible = value === true; }, false);
        native.on('gmodLaunchState', state => {
            launching = state != null;
            gameStateChanged();
            render();
            tryBackground();
        });
        window.addEventListener('focus', () => {
            if (retryAfterGameRevision === gameRevision) gameStateChanged();
        });
        native.invoke('updateCurrentVersion').then(value => {
            if (typeof value === 'string') version = value;
            render();
        }).catch(() => {});
        native.invoke('updateGetMode').then(value => {
            if (!modes.includes(value)) throw new Error('Invalid update mode.');
            mode = value;
            render();
            tryBackground();
        }).catch(() => {
            modeError = 'loadError';
            render();
        });
        syncDom();
    }

    if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', start, {once: true});
    else start();
})();
