(function () {
    'use strict';

    const copy = {
        ru: {
            title: 'Оформление', live: 'Предпросмотр', appearance: 'Стиль', media: 'Медиа', sound: 'Звук', layout: 'Раскладка', decor: 'Декор',
            hubMusic: 'Музыка хаба', gameMenuMusic: 'Музыка меню игры', gameLoadingMusic: 'Музыка загрузки',
            musicVolume: 'Громкость', silence: 'Без музыки',
            stockMusic: 'Стандартная музыка', customMusic: 'Свой файл', listen: 'Прослушать', stop: 'Остановить', removeMusic: 'Убрать',
            audioError: 'Не удалось воспроизвести звук. Выберите другой файл или сбросьте замену.',
            afterLaunchMusic: 'Музыка меню игры меняется при следующем запуске из хаба.',
            afterLaunchLoadingMusic: 'Музыка загрузки применяется при следующем запуске игры из хаба.',
            builtinStyles: 'Темы',
            hub: 'Хаб', menu: 'Меню игры', browser: 'Серверы', loading: 'Загрузка',
            accent: 'Цвет акцента', identity: 'Названия окон', hubName: 'Название хаба', gameName: 'Название игры',
            chooseColor: 'Выбрать цвет', hue: 'Оттенок', saturation: 'Насыщенность', brightness: 'Яркость',
            colorPlane: 'Насыщенность и яркость. Стрелки влево и вправо меняют насыщенность, вверх и вниз — яркость.', correction: 'Цветокоррекция',
            editorTabs: 'Разделы оформления', previewTargets: 'Экраны предпросмотра',
            afterLaunch: 'Оформление игры применяется при следующем запуске из хаба.',
            background: 'Фон', logo: 'Логотип', windowIcon: 'Значок окна',
            choose: 'Выбрать…', restore: 'Вернуть', position: 'Положение',
            layoutHint: 'Перетащите элементы в предпросмотре или задайте положение в процентах.',
            shown: 'Показать', play: 'Играть', settings: 'Настройки', social: 'Соцсети',
            onlinePanopticon: 'Онлайн Panopticon', onlineShinri: 'Онлайн Shinri', promo: 'Афиша',
            minimize: 'Свернуть в трей', close: 'Закрыть', x: 'По горизонтали, %', y: 'По вертикали, %', width: 'Ширина, %',
            add: 'Добавить файл…', remove: 'Удалить', noDecor: 'Добавьте картинку, GIF или видео поверх фона хаба.',
            decoration: 'Декор', import: 'Импорт…', export: 'Экспорт…', reset: 'Сбросить', cancel: 'Отмена', apply: 'Применить',
            imported: 'Профиль загружен. Нажмите «Применить», чтобы сохранить.', exported: 'Профиль экспортирован.',
            resetDraft: 'Восстановлен стандартный вид. Нажмите «Применить», чтобы сохранить.',
            wait: 'Загрузка настроек…', loadError: 'Не удалось загрузить оформление. Откройте редактор снова.',
            saveError: 'Не удалось сохранить оформление.', fileError: 'Не удалось открыть файл.',
            importError: 'Не удалось импортировать профиль.', exportError: 'Не удалось экспортировать профиль.',
            decodeError: 'Файл не воспроизводится. Выберите другой файл или верните стандартный.',
            serverOne: 'Лобби', serverTwo: 'Быстрая игра', connecting: 'Подключение к серверу…',
            activeServer: 'Выбранный сервер', noServer: 'Сервер не выбран', serverList: 'Список серверов', localRegion: 'СНГ',
        },
        en: {
            title: 'Appearance', live: 'Live preview', appearance: 'Style', media: 'Media', sound: 'Sound', layout: 'Layout', decor: 'Decorations',
            hubMusic: 'Hub music', gameMenuMusic: 'Game menu music', gameLoadingMusic: 'Loading music',
            musicVolume: 'Volume', silence: 'No music',
            stockMusic: 'Default music', customMusic: 'Custom file', listen: 'Listen', stop: 'Stop', removeMusic: 'Remove',
            audioError: 'Could not play this audio. Choose another file or reset the replacement.',
            afterLaunchMusic: 'Game menu music changes the next time you launch the game from the Hub.',
            afterLaunchLoadingMusic: 'Loading music takes effect the next time you launch the game from the Hub.',
            builtinStyles: 'Themes',
            hub: 'Hub', menu: 'Game menu', browser: 'Servers', loading: 'Loading',
            accent: 'Accent color', identity: 'Window titles', hubName: 'Hub title', gameName: 'Game title',
            chooseColor: 'Choose color', hue: 'Hue', saturation: 'Saturation', brightness: 'Brightness',
            colorPlane: 'Saturation and brightness. Use left and right to change saturation, and up and down to change brightness.', correction: 'Color correction',
            editorTabs: 'Appearance sections', previewTargets: 'Preview screens',
            afterLaunch: 'Game appearance takes effect the next time you launch it from the Hub.',
            background: 'Background', logo: 'Logo', windowIcon: 'Window icon',
            choose: 'Choose…', restore: 'Restore', position: 'Position',
            layoutHint: 'Drag items in the preview or set their position as percentages.',
            shown: 'Show', play: 'Play', settings: 'Settings', social: 'Social links',
            onlinePanopticon: 'Panopticon online', onlineShinri: 'Shinri online', promo: 'Events',
            minimize: 'Minimize to tray', close: 'Close', x: 'Horizontal, %', y: 'Vertical, %', width: 'Width, %',
            add: 'Add file…', remove: 'Remove', noDecor: 'Add an image, GIF or video over the Hub background.',
            decoration: 'Decoration', import: 'Import…', export: 'Export…', reset: 'Reset', cancel: 'Cancel', apply: 'Apply',
            imported: 'Profile loaded. Select Apply to save it.', exported: 'Profile exported.',
            resetDraft: 'Defaults restored. Select Apply to save them.',
            wait: 'Loading appearance…', loadError: 'Could not load the appearance. Reopen the editor to try again.',
            saveError: 'Could not save the appearance.', fileError: 'Could not open the file.',
            importError: 'Could not import the profile.', exportError: 'Could not export the profile.',
            decodeError: 'This file could not be displayed. Choose another file or restore the default.',
            serverOne: 'Lobby', serverTwo: 'Quick game', connecting: 'Connecting to server…',
            activeServer: 'Selected server', noServer: 'No server selected', serverList: 'Server list', localRegion: 'CIS',
        },
        fr: {
            title: 'Apparence', live: 'Aperçu en direct', appearance: 'Style', media: 'Médias', sound: 'Son', layout: 'Disposition', decor: 'Décorations',
            hubMusic: 'Musique du hub', gameMenuMusic: 'Musique du menu du jeu', gameLoadingMusic: 'Musique du chargement',
            musicVolume: 'Volume', silence: 'Sans musique',
            stockMusic: 'Musique par défaut', customMusic: 'Fichier personnalisé', listen: 'Écouter', stop: 'Arrêter', removeMusic: 'Supprimer',
            audioError: 'Impossible de lire ce son. Choisissez un autre fichier ou rétablissez le réglage.',
            afterLaunchMusic: 'La musique du menu change au prochain lancement du jeu depuis le hub.',
            afterLaunchLoadingMusic: 'La musique du chargement s’applique au prochain lancement du jeu depuis le hub.',
            builtinStyles: 'Thèmes',
            hub: 'Hub', menu: 'Menu du jeu', browser: 'Serveurs', loading: 'Chargement',
            accent: 'Couleur d’accent', identity: 'Titres des fenêtres', hubName: 'Titre du hub', gameName: 'Titre du jeu',
            chooseColor: 'Choisir une couleur', hue: 'Teinte', saturation: 'Saturation', brightness: 'Luminosité',
            colorPlane: 'Saturation et luminosité. Utilisez les flèches gauche et droite pour régler la saturation, et haut et bas pour la luminosité.', correction: 'Correction des couleurs',
            editorTabs: 'Sections de l’apparence', previewTargets: 'Écrans d’aperçu',
            afterLaunch: 'L’apparence du jeu s’appliquera au prochain lancement depuis le hub.',
            background: 'Arrière-plan', logo: 'Logo', windowIcon: 'Icône de fenêtre',
            choose: 'Choisir…', restore: 'Rétablir', position: 'Position',
            layoutHint: 'Déplacez les éléments dans l’aperçu ou indiquez leur position en pourcentage.',
            shown: 'Afficher', play: 'Jouer', settings: 'Paramètres', social: 'Réseaux sociaux',
            onlinePanopticon: 'Panopticon en ligne', onlineShinri: 'Shinri en ligne', promo: 'Événements',
            minimize: 'Réduire dans la zone de notification', close: 'Fermer', x: 'Horizontale, %', y: 'Verticale, %', width: 'Largeur, %',
            add: 'Ajouter un fichier…', remove: 'Supprimer', noDecor: 'Ajoutez une image, un GIF ou une vidéo sur le fond du hub.',
            decoration: 'Décoration', import: 'Importer…', export: 'Exporter…', reset: 'Réinitialiser', cancel: 'Annuler', apply: 'Appliquer',
            imported: 'Profil chargé. Choisissez Appliquer pour l’enregistrer.', exported: 'Profil exporté.',
            resetDraft: 'Apparence par défaut rétablie. Choisissez Appliquer pour l’enregistrer.',
            wait: 'Chargement de l’apparence…', loadError: 'Impossible de charger l’apparence. Rouvrez l’éditeur pour réessayer.',
            saveError: 'Impossible d’enregistrer l’apparence.', fileError: 'Impossible d’ouvrir le fichier.',
            importError: 'Impossible d’importer le profil.', exportError: 'Impossible d’exporter le profil.',
            decodeError: 'Ce fichier ne peut pas être affiché. Choisissez un autre fichier ou rétablissez le réglage par défaut.',
            serverOne: 'Salon', serverTwo: 'Partie rapide', connecting: 'Connexion au serveur…',
            activeServer: 'Serveur sélectionné', noServer: 'Aucun serveur sélectionné', serverList: 'Liste des serveurs', localRegion: 'CEI',
        },
    };
    const brushIcon = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m11 13 7.7-8.7a1.4 1.4 0 0 1 2 2L12 14M9.5 13c2.1 0 3.4 1.5 3.4 3.4 0 3.1-3.4 4.6-8.5 4.6 1.7-1.7 1.9-2.8 1.9-4.5A3.4 3.4 0 0 1 9.5 13Z"/></svg>';
    const icons = {settings: '⚙', minimize: '−', close: '×', social: '•••'};
    const defaults = {
        hubBackground: './background-q0xJoMR6.webm',
        hubLogo: './panopticon-logo.png',
        gameMenuBackground: '/design-defaults/gameMenuBackground',
        gameBrowserBackground: '/design-defaults/gameBrowserBackground',
        gameLoadingBackground: '/design-defaults/gameLoadingBackground',
        gameLogo: '/design-defaults/gameLogo',
        gameLoadingLogo: '/design-defaults/gameLoadingLogo',
        hubIcon: '/design-defaults/hubIcon',
        gameIcon: '/design-defaults/gameIcon',
        gameMenuMusic: '/design-defaults/gameMenuMusic',
    };
    const screens = {
        hub: ['hubBackground', 'hubLogo', 'hubIcon'],
        menu: ['gameMenuBackground', 'gameLogo', 'gameIcon'],
        browser: ['gameBrowserBackground', 'gameLogo', 'gameIcon'],
        loading: ['gameLoadingBackground', 'gameLogo', 'gameIcon'],
    };
    const formats = {hubBackground: 'PNG · JPG · WebP · GIF · WebM · MP4', gameBackground: 'PNG · JPG · WebP · GIF · WebM', logo: 'PNG · JPG · WebP · GIF', icon: 'ICO'};
    const parts = {
        play: {selector: '.action .bm-wrap', x: 90, y: 91, required: true},
        settings: {selector: '.topbar .settings', x: 83, y: 6, required: true},
        logo: {selector: '.brand', x: 11, y: 8},
        onlinePanopticon: {selector: '.topbar .pt-online', x: 63, y: 6},
        onlineShinri: {selector: '.topbar .online-chip:not(.pt-online)', x: 74, y: 6},
        social: {selector: '.social', x: 96, y: 50},
        promo: {selector: '.promo', x: 20, y: 82},
        minimize: {selector: '.topbar [data-pt-design-window="minimize"]', x: 93, y: 6},
        close: {selector: '.topbar .icon-btn.close', x: 97, y: 6},
    };
    const assetKey = /^[0-9a-f]{64}\.(?:png|jpe?g|webp|gif|webm|mp4|ico)$/;
    const audioKey = /^[0-9a-f]{64}\.(?:mp3|ogg|wav)$/;
    const soundScreens = {hub: 'hubMusic', menu: 'gameMenuMusic', loading: 'gameLoadingMusic'};
    const defaultLoadingVolume = 30;
    const themedBackgrounds = ['hubBackground', 'gameMenuBackground'];
    const stroke = /^#[0-9a-f]{6}$/i;
    const colors = ['#ebcb7c', '#e88987', '#bba2eb', '#81c8ec', '#84d0ac'];
    const defaultAccent = colors[0];
    const panopticonVideo = '/design-defaults/panopticon-background.webm';
    const panopticonPoster = '/design-defaults/panopticon-background.jpg';
    const filterControls = {
        hue: {min: -180, max: 180, initial: 0, unit: '°'},
        saturation: {min: 0, max: 200, initial: 100, unit: '%'},
        brightness: {min: 0, max: 200, initial: 100, unit: '%'},
    };
    const colorTokens = ['--pt-accent', '--pt-accent-text', '--pt-accent-mark', '--pt-accent-soft', '--pt-accent-outline', '--pt-accent-hover', '--pt-accent-active', '--pt-accent-ink', '--pt-accent-fill'];
    const video = /\.(webm|mp4)$/;
    const validated = new Set();
    let saved = {version: 1};
    let draft;
    let loading;
    let loaded = false;
    let busy = false;
    let status;
    let tool = 'appearance';
    let screen = 'hub';
    let selection = 'play';
    let selectedDecor = 0;
    let root;
    let hub;
    let brush;
    let editor;
    let drag;
    let windowVisible = !document.hidden;
    let nativeVisible = true;
    let frameScheduled = false;
    let locale;
    let coordInputs = {};
    let previewGeometry;
    let audioPreview;
    let themes;
    let initialStyle;
    let initialStyleDone = false;
    const defaultPositions = {};
    const ui = {};

    const text = (key) => (copy[document.documentElement.lang] || copy.en)[key];
    const clone = (profile) => JSON.parse(JSON.stringify(profile));
    const name = (profile, field) => profile[field]?.trim() || (field === 'hubName' ? 'Panopticon Hub' : 'Panopticon Trial');
    const clamp = (number, min, max) => Math.min(Math.max(number, min), max);
    const percent = (value) => Math.round(clamp(value, 0, 100));
    const source = (key) => '/design-media/' + key;
    const el = (tag, className, label) => {
        const node = document.createElement(tag);
        if (className) node.className = className;
        if (label) node.textContent = label;
        return node;
    };

    function button(label, fn, className = 'pt-design-button st-btn st-btn--secondary') {
        const node = el('button', className, label);
        node.type = 'button';
        node.addEventListener('click', fn);
        return node;
    }

    function identifyButton(node, label) {
        node.title = label;
        node.setAttribute('aria-label', label);
    }

    function titleRow(label, content) {
        const row = el('label', 'pt-design-field');
        row.append(el('span', '', label), content);
        return row;
    }

    function readBackgroundFilter(profile, slot) {
        const correction = profile.backgroundFilters?.[slot];
        return Object.fromEntries(Object.entries(filterControls).map(([field, {min, max, initial}]) => {
            const value = correction?.[field];
            return [field, Number.isFinite(value) ? Math.round(clamp(value, min, max)) : initial];
        }));
    }

    function applyBackgroundFilter(layer, profile, slot) {
        const {hue, saturation, brightness} = readBackgroundFilter(profile, slot);
        if (hue === 0 && saturation === 100 && brightness === 100) {
            layer.style.removeProperty('--pt-background-filter');
        } else {
            layer.style.setProperty('--pt-background-filter', `hue-rotate(${hue}deg) saturate(${saturation}%) brightness(${brightness}%)`);
        }
    }

    function setColor(scope, profile) {
        const accent = profile?.accent;
        if (!stroke.test(accent || '')) {
            colorTokens.forEach((key) => scope.style.removeProperty(key));
            scope.classList.remove('pt-design-accent', 'pt-design-website-style');
            return;
        }
        const [r, g, b] = rgbFromHex(accent);
        const luminance = [r, g, b].map((c) => c / 255).map((c) => c <= .04045 ? c / 12.92 : ((c + .055) / 1.055) ** 2.4);
        const lightness = .2126 * luminance[0] + .7152 * luminance[1] + .0722 * luminance[2];
        const darkText = lightness > .179;
        const accentText = 'color-mix(in srgb,' + accent + (darkText ? ' 68%' : ' 34%') + ', white)';
        const websiteColors = profile.builtinStyle === 'panopticon' && accent.toLowerCase() === '#f0504f';
        // Keep thin controls visible against the dark Hub panels.
        const values = [
            accent, websiteColors ? '#ff9c9a' : accentText, lightness >= .13 ? accent : accentText,
            accent + '1f', accent + '8c',
            websiteColors ? '#e64b51' : 'color-mix(in srgb,' + accent + ' 86%, ' + (darkText ? 'white' : 'black') + ')',
            websiteColors ? '#c13038' : 'color-mix(in srgb,' + accent + ' 85%, ' + (darkText ? 'black' : 'white') + ')',
            websiteColors || !darkText ? '#ffffff' : '#100c08',
            websiteColors ? '#d93a41' : accent,
        ];
        colorTokens.forEach((key, index) => scope.style.setProperty(key, values[index]));
        scope.classList.add('pt-design-accent');
        scope.classList.toggle('pt-design-website-style', websiteColors);
    }

    function mediaNode(url, className, onError) {
        const isVideo = video.test(url) || url === defaults.gameMenuBackground;
        const node = el(isVideo ? 'video' : 'img', className);
        if (isVideo) {
            node.muted = true;
            node.loop = true;
            node.playsInline = true;
            node.preload = 'auto';
            node.disablePictureInPicture = true;
            if (url === panopticonVideo) node.poster = panopticonPoster;
            node.addEventListener('loadeddata', () => {
                if (windowVisible && node.isConnected) node.play().catch(() => {});
            });
            node.autoplay = windowVisible;
        } else {
            node.alt = '';
            node.draggable = false;
        }
        if (onError) node.addEventListener('error', onError, {once: true});
        node.src = url;
        return node;
    }

    function showMedia(container, custom, fallback, onError) {
        const key = assetKey.test(custom || '') ? custom : null;
        const url = key ? source(key) : fallback;
        if (container.mediaUrl === url && container.mediaFallback === fallback) return;
        container.mediaUrl = url;
        container.mediaFallback = fallback;
        container.replaceChildren();
        if (!url) return;
        const node = mediaNode(url, '', () => {
            if (node.parentNode !== container) return;
            node.remove();
            if (key && fallback) container.append(mediaNode(fallback));
            else if (url === panopticonVideo) container.append(mediaNode(panopticonPoster));
            if (key) onError?.();
        });
        container.append(node);
    }

    function testMedia(key) {
        const isAudio = audioKey.test(key);
        if (!isAudio && !assetKey.test(key)) return Promise.resolve(false);
        if (validated.has(key)) return Promise.resolve(true);
        return new Promise((resolve) => {
            const isVideo = video.test(key);
            const isPlayback = isVideo || isAudio;
            const node = el(isVideo ? 'video' : isAudio ? 'audio' : 'img');
            const ready = isPlayback ? 'loadeddata' : 'load';
            const timeout = setTimeout(() => done(false), 7000);
            function done(ok) {
                clearTimeout(timeout);
                node.removeEventListener(ready, success);
                node.removeEventListener('error', error);
                node.removeAttribute('src');
                if (isPlayback) node.load();
                if (ok) validated.add(key);
                resolve(ok);
            }
            function success() { done(!isVideo || node.videoWidth > 0); }
            function error() { done(false); }
            if (isPlayback) {
                node.muted = true;
                node.preload = 'auto';
            }
            node.addEventListener(ready, success);
            node.addEventListener('error', error);
            node.src = source(key);
        });
    }

    function geometry(position, field, fallback) {
        return Number.isFinite(position?.[field]) ? position[field] : fallback;
    }

    function positionIn(rect, width, height, point, padding = 0) {
        const x = clamp(rect.width * point.x / 100 - width / 2, padding, Math.max(padding, rect.width - width - padding));
        const y = clamp(rect.height * point.y / 100 - height / 2, padding, Math.max(padding, rect.height - height - padding));
        return {x, y, width, height};
    }

    function intersects(a, b, gap = 4) {
        return a.x < b.x + b.width + gap && a.x + a.width + gap > b.x &&
            a.y < b.y + b.height + gap && a.y + a.height + gap > b.y;
    }

    function keepReachable(wanted, rect, obstacles) {
        if (!obstacles.some((other) => intersects(wanted, other))) return wanted;
        const choices = [];
        const xs = [wanted.x, 8, rect.width - wanted.width - 8];
        const ys = [wanted.y, 8, rect.height - wanted.height - 8];
        for (const bounds of obstacles) {
            xs.push(bounds.x - wanted.width - 5, bounds.x + bounds.width + 5);
            ys.push(bounds.y - wanted.height - 5, bounds.y + bounds.height + 5);
        }
        for (const x of xs) for (const y of ys) {
            if (x >= 0 && y >= 0 && x + wanted.width <= rect.width && y + wanted.height <= rect.height) choices.push({...wanted, x, y});
        }
        choices.sort((a, b) => Math.hypot(a.x - wanted.x, a.y - wanted.y) - Math.hypot(b.x - wanted.x, b.y - wanted.y));
        return choices.find((choice) => !obstacles.some((other) => intersects(choice, other))) || wanted;
    }

    function setCoords(node, x, y) {
        node.style.setProperty('--pt-design-x', x + 'px');
        node.style.setProperty('--pt-design-y', y + 'px');
    }

    function setLayoutClasses(profile) {
        const entries = {};
        for (const [id, part] of Object.entries(parts)) {
            const node = hub.querySelector(part.selector);
            if (!node) continue;
            const position = profile.layout?.[id];
            const hidden = !part.required && position?.hidden === true;
            const moved = Number.isFinite(position?.x) || Number.isFinite(position?.y);
            node.classList.toggle('pt-design-layout-hidden', hidden);
            node.classList.toggle('pt-design-placed', moved && !hidden);
            node.classList.toggle('pt-design-layout-shown', !hidden && (moved || position?.hidden === false));
            entries[id] = {node, hidden, moved};
        }
        return entries;
    }

    function measureLayout(entries) {
        const rect = hub.getBoundingClientRect();
        const relative = (node) => {
            const bounds = node.getBoundingClientRect();
            return {x: bounds.left - rect.left, y: bounds.top - rect.top, width: bounds.width, height: bounds.height};
        };
        const bounds = {};
        for (const [id, item] of Object.entries(entries)) {
            if (item.hidden) continue;
            const box = item.moved ? {width: item.node.offsetWidth, height: item.node.offsetHeight} : relative(item.node);
            if (!box.width || !box.height) continue;
            bounds[id] = {...box, moved: item.moved};
        }
        const progress = hub.querySelector('.action .dlp');
        const audio = hub.querySelector('#pt-hub-menu-audio');
        return {rect, bounds, fixed: relative(brush), progress: progress && relative(progress), audio: audio && relative(audio)};
    }

    function projectLayout(measured, profile) {
        const {rect, bounds, fixed, progress, audio} = measured;
        const placed = {};
        const reserved = [fixed];
        if (progress?.width) reserved.push(progress);
        if (audio?.width) reserved.push(audio);
        const stationary = [];
        for (const [id, box] of Object.entries(bounds)) {
            if (box.moved) continue;
            placed[id] = box;
            if (parts[id].required) reserved.push(box);
            else stationary.push(box);
        }
        // Сохраняем доступ к основным кнопкам при пересечении импортированных координат.
        for (const [id, box] of Object.entries(bounds)) {
            if (!box.moved) continue;
            const position = profile.layout?.[id];
            const anchor = defaultPositions[id] || parts[id];
            const wanted = positionIn(rect, box.width, box.height, {
                x: geometry(position, 'x', anchor.x),
                y: geometry(position, 'y', anchor.y),
            }, 8);
            const safe = keepReachable(wanted, rect, parts[id].required ? [...reserved, ...stationary] : reserved);
            placed[id] = safe;
            if (parts[id].required) reserved.push(safe);
        }
        return placed;
    }

    function updateLayout() {
        const entries = setLayoutClasses(saved);
        const measured = measureLayout(entries);
        const {rect, bounds} = measured;
        for (const [id, box] of Object.entries(bounds)) {
            if (!box.moved) {
                defaultPositions[id] = {
                    x: (box.x + box.width / 2) / rect.width * 100,
                    y: (box.y + box.height / 2) / rect.height * 100,
                };
            }
        }
        for (const [id, box] of Object.entries(projectLayout(measured, saved))) {
            if (entries[id].moved) setCoords(entries[id].node, rect.left + box.x, rect.top + box.y);
        }
        previewGeometry = null;
    }

    function measurePreviewLayout() {
        const key = Object.keys(parts).map((id) => {
            const position = draft.layout?.[id];
            const moved = Number.isFinite(position?.x) || Number.isFinite(position?.y);
            return (moved ? 3 : 0) + (position?.hidden === true ? 2 : position?.hidden === false ? 1 : 0);
        }).join('');
        if (previewGeometry?.key === key) return previewGeometry.measured;
        // Измеряем черновую раскладку в размерах окна Hub и восстанавливаем классы до отрисовки кадра.
        const measured = measureLayout(setLayoutClasses(draft));
        setLayoutClasses(saved);
        previewGeometry = {key, measured};
        return measured;
    }

    function backdrop() {
        const art = hub.querySelector('.art');
        if (!art) return;
        applyBackgroundFilter(art, saved, 'hubBackground');
        const asset = assetKey.test(saved.assets?.hubBackground || '') ? saved.assets.hubBackground : '';
        const original = art.querySelector('.art-hero');
        const defaultMedia = saved.builtinStyle === 'panopticon'
            ? (original?.tagName === 'IMG' ? panopticonPoster : panopticonVideo)
            : '';
        let custom = art.querySelector('.pt-design-actual-background');
        if (!custom && (asset || defaultMedia)) {
            custom = el('div', 'pt-design-actual-background');
            art.querySelector('.art-glow').before(custom);
        }
        if (custom) showMedia(custom, asset, defaultMedia, scheduleLive);
        const media = custom?.firstChild;
        art.classList.toggle('pt-design-replaced', !!media);
        if (original?.tagName === 'VIDEO') {
            if (media && !original.designPaused) {
                original.designPaused = true;
                original.autoplay = false;
                original.addEventListener('play', pauseStockBackground);
                original.pause();
            } else if (!media && original.designPaused) {
                original.designPaused = false;
                original.autoplay = true;
                original.removeEventListener('play', pauseStockBackground);
                if (windowVisible) original.play().catch(() => {});
            }
        }
    }

    function pauseStockBackground(event) {
        event.currentTarget.pause();
    }

    function applyBrand() {
        const brand = saved.assets?.hubLogo;
        const label = name(saved, 'hubName');
        if (document.title !== label) document.title = label;
        for (const node of hub.querySelectorAll('.brand-logo, .dlp-start-logo')) {
            const alt = !saved.hubName && node.classList.contains('dlp-start-logo') ? 'Panopticon Trial' : label;
            if (node.alt !== alt) node.alt = alt;
            const url = brand && assetKey.test(brand) && node.failedDesignLogo !== brand ? source(brand) : defaults.hubLogo;
            if (node.getAttribute('src')?.endsWith(url.replace(/^\.\//, '/'))) continue;
            node.onerror = () => {
                node.failedDesignLogo = brand;
                if (brand) node.src = defaults.hubLogo;
            };
            node.src = url;
        }
        if (saved.assets?.hubIcon || document.querySelector('#pt-design-tab-icon')) {
            let icon = document.querySelector('#pt-design-tab-icon');
            if (!icon) {
                icon = el('link');
                icon.id = 'pt-design-tab-icon';
                icon.rel = 'icon';
                document.head.append(icon);
            }
            icon.href = saved.assets?.hubIcon ? source(saved.assets.hubIcon) : defaults.hubIcon;
        }
    }

    function updateDecor(layer, profile, active) {
        const items = profile.decorations || [];
        while (layer.childElementCount > items.length) layer.lastElementChild.remove();
        items.forEach((item, index) => {
            const node = layer.children[index] || el('div', 'pt-design-decoration');
            if (!node.parentNode) layer.append(node);
            const target = active && tool === 'decor';
            node.classList.toggle('is-editable', target);
            node.classList.toggle('is-selected', target && selectedDecor === index);
            node.dataset.decorationIndex = index;
            if (target) {
                node.tabIndex = 0;
                node.setAttribute('role', 'button');
                node.setAttribute('aria-label', text('decoration') + ' ' + (index + 1));
            } else {
                node.removeAttribute('tabindex');
                node.removeAttribute('role');
                node.removeAttribute('aria-label');
            }
            node.style.width = clamp(item.width, 5, 90) + '%';
            showMedia(node, item.asset, '', active ? () => notice('decodeError', true) : null);
            const fit = () => {
                const point = positionIn(layer.getBoundingClientRect(), node.offsetWidth, node.offsetHeight, item);
                setCoords(node, point.x, point.y);
            };
            if (node.firstChild) {
                const media = node.firstChild;
                media.onloadeddata = fit;
                media.onload = fit;
            }
            fit();
        });
    }

    function refreshLive() {
        frameScheduled = false;
        hub = root.querySelector('.hub');
        if (!hub) return;
        const settings = hub.querySelector('.topbar .settings');
        if (!settings) return;
        if (!brush) {
            brush = button('', openEditor, 'icon-btn no-drag-window');
            brush.id = 'pt-design-open';
            brush.innerHTML = brushIcon;
            brush.setAttribute('aria-haspopup', 'dialog');
        }
        if (settings.nextElementSibling !== brush) settings.after(brush);
        identifyButton(brush, text('title'));
        if (brush.disabled !== settings.disabled) brush.disabled = settings.disabled;
        setColor(document.documentElement, saved);
        window.PTHubMenuAudio?.apply(saved.assets?.hubMusic ?? null);
        updateLayout();
        backdrop();
        applyBrand();
        let decorations = hub.querySelector('.pt-design-actual-decor');
        if (!decorations && saved.decorations?.length) {
            decorations = el('div', 'pt-design-actual-decor');
            decorations.setAttribute('aria-hidden', 'true');
            hub.querySelector('.art').after(decorations);
        }
        if (decorations) updateDecor(decorations, saved, false);
        if (locale !== document.documentElement.lang) {
            locale = document.documentElement.lang;
            if (editor?.open && !busy) renderEditor();
        }
        if (editor?.open && draft) {
            if (screen === 'hub') renderPreviewBackdrop();
            placePreview();
        }
    }

    function scheduleLive() {
        if (frameScheduled) return;
        frameScheduled = true;
        requestAnimationFrame(refreshLive);
    }

    function notice(key, error = false, detail) {
        status = key ? {key, error, detail} : null;
        if (editor?.open) renderNotice();
    }

    function renderNotice() {
        ui.notice.textContent = status ? text(status.key) + (status.detail ? ' ' + status.detail : '') : '';
        ui.notice.classList.toggle('is-error', status?.error === true);
        ui.notice.hidden = !status;
    }

    function load() {
        if (loaded || loading) return loading;
        loading = window.electron.invoke('designLoad').then((profile) => {
            if (profile?.version !== 1) throw Error('Invalid design profile');
            saved = profile;
            loaded = true;
            scheduleLive();
        }).catch(() => {
            if (editor?.open) notice('loadError', true);
        }).finally(() => { loading = null; });
        return loading;
    }

    async function readStyles() {
        const status = await window.electron.invoke('designStyles');
        if (status?.themes) themes = status.themes;
        return status;
    }

    window.panopticonInitialStyleReady = function () {
        if (initialStyleDone || !window.PTHubInitialStyle) return;
        if (!initialStyle) {
            initialStyle = (async () => {
                const consent = await window.electron.invoke('store', 'get', 'legalConsent');
                if (!consent?.acceptedAt) return;
                const status = await readStyles();
                if (!status?.pending) {
                    initialStyleDone = true;
                    return;
                }
                await window.PTHubInitialStyle.open({
                    themes: status.themes,
                    select: async (choice) => {
                        await loading;
                        const profile = await window.electron.invoke('designSelectInitialStyle', choice);
                        if (profile?.version !== 1) throw Error('Invalid design profile');
                        saved = profile;
                        loaded = true;
                        scheduleLive();
                        return profile;
                    },
                });
                initialStyleDone = true;
            })().catch(error => {
                console.error('Ошибка загрузки тем оформления:', error);
            }).finally(() => { initialStyle = null; });
        }
        return initialStyle;
    };

    function buildEditor() {
        editor = el('dialog', 'modal pt-design no-drag-window');
        editor.setAttribute('aria-labelledby', 'pt-design-title');
        const dismiss = () => {
            if (!busy) editor.close();
        };
        editor.addEventListener('cancel', (event) => {
            event.preventDefault();
            dismiss();
        });
        let startedOutside = false;
        function outside(event) {
            if (event.target !== editor) return false;
            const rect = editor.getBoundingClientRect();
            return event.clientX < rect.left || event.clientX >= rect.right || event.clientY < rect.top || event.clientY >= rect.bottom;
        }
        editor.addEventListener('pointerdown', (event) => {
            if (event.isPrimary) startedOutside = !busy && event.button === 0 && outside(event);
        });
        editor.addEventListener('pointercancel', () => { startedOutside = false; });
        editor.addEventListener('click', (event) => {
            if (startedOutside && outside(event)) dismiss();
            startedOutside = false;
        });
        editor.addEventListener('close', () => {
            startedOutside = false;
            stopAudioPreview();
            hub?.classList.remove('pt-design-editing');
            draft = null;
            drag = null;
            ui.backdrop.mediaUrl = undefined;
            ui.backdrop.replaceChildren();
            ui.surface.replaceChildren();
            ui.decorations.replaceChildren();
        });
        ui.header = el('header', 'modal-head pt-design-header');
        ui.heading = el('h1', 'modal-title');
        ui.heading.id = 'pt-design-title';
        ui.close = button('', dismiss, 'modal-x');
        ui.close.innerHTML = '<svg class="ico" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" aria-hidden="true"><path d="m6 6 12 12M18 6 6 18"/></svg>';
        ui.header.append(ui.heading, ui.close);
        ui.tools = el('nav', 'pt-design-tools');
        ui.area = el('div', 'modal-body pt-design-area');
        ui.area.tabIndex = 0;
        ui.preview = el('section', 'pt-design-preview');
        ui.previewHeading = el('h2');
        ui.screens = el('nav', 'pt-design-screens');
        ui.windowBar = el('div', 'pt-design-window-bar');
        ui.windowIcon = el('span', 'pt-design-window-icon');
        ui.windowTitle = el('span');
        ui.windowBar.append(ui.windowIcon, ui.windowTitle);
        ui.stage = el('div', 'pt-design-stage');
        ui.stockBase = el('div', 'art-base');
        ui.backdrop = el('div', 'pt-design-preview-backdrop');
        ui.stockLighting = el('div', 'pt-design-preview-lighting');
        ui.stockLighting.append(el('div', 'art-glow'), el('div', 'art-vig'));
        ui.decorations = el('div', 'pt-design-preview-decor');
        ui.surface = el('div', 'pt-design-surface');
        ui.stage.append(ui.stockBase, ui.backdrop, ui.stockLighting, ui.decorations, ui.surface);
        ui.stage.addEventListener('pointerdown', startMove);
        ui.stage.addEventListener('pointermove', move);
        ui.stage.addEventListener('lostpointercapture', () => { drag = null; });
        ui.stage.addEventListener('keydown', keyMove);
        ui.frame = el('div', 'pt-design-preview-frame');
        ui.frame.append(ui.windowBar, ui.stage);
        ui.tip = el('p', 'pt-design-tip');
        ui.preview.append(ui.previewHeading, ui.screens, ui.frame, ui.tip);
        ui.inspector = el('section', 'pt-design-inspector');
        ui.area.append(ui.preview, ui.inspector);
        ui.notice = el('p', 'pt-design-status');
        ui.notice.setAttribute('role', 'status');
        ui.footer = el('footer', 'pt-design-footer');
        editor.append(ui.header, ui.tools, ui.area, ui.notice, ui.footer);
        document.body.append(editor);
        new ResizeObserver(() => {
            if (editor.open && draft) placePreview();
        }).observe(ui.stage);
    }

    async function openEditor() {
        if (!editor) buildEditor();
        if (editor.open || brush.disabled) return;
        tool = 'appearance';
        screen = 'hub';
        selection = 'play';
        selectedDecor = 0;
        status = null;
        draft = clone(saved);
        busy = !loaded;
        if (!loaded) notice('wait');
        hub.classList.add('pt-design-editing');
        editor.showModal();
        renderEditor();
        ui.area.scrollTop = 0;
        ui.area.focus({preventScroll: true});
        if (!loaded) {
            await load();
            if (!editor.open) return;
            draft = clone(saved);
            if (loaded) notice(null);
            busy = false;
            renderEditor();
        }
        if (!themes) {
            const status = await readStyles().catch(() => null);
            const section = editor.open && tool === 'appearance' && status?.themes && builtinThemeFields();
            if (section && !ui.inspector.querySelector('.pt-design-builtin-styles')) ui.inspector.append(section);
        }
    }

    function renderEditor() {
        setColor(editor, draft);
        ui.heading.textContent = text('title');
        ui.previewHeading.textContent = text('live');
        identifyButton(ui.close, text('close'));
        ui.close.disabled = busy;
        ui.tools.setAttribute('aria-label', text('editorTabs'));
        const tools = screen === 'hub' ? ['appearance', 'media', 'sound', 'layout', 'decor'] : ['appearance', 'media', 'sound'];
        ui.tools.replaceChildren(...tools.map((id) => {
            const node = button(text(id), () => {
                stopAudioPreview();
                tool = id;
                if (tool === 'sound' && !soundScreens[screen]) screen = 'menu';
                renderEditor();
                ui.area.scrollTop = 0;
                ui.tools.querySelector('[aria-pressed="true"]')?.focus();
            }, 'pt-design-tool');
            node.dataset.tool = id;
            node.setAttribute('aria-pressed', String(tool === id));
            node.disabled = busy || !loaded;
            return node;
        }));
        const targets = tool === 'layout' || tool === 'decor' ? ['hub']
            : tool === 'sound' ? Object.keys(soundScreens) : Object.keys(screens);
        ui.screens.setAttribute('aria-label', text('previewTargets'));
        ui.screens.replaceChildren(...targets.map((id) => {
            const node = button(text(id), () => {
                stopAudioPreview();
                screen = id;
                renderEditor();
                ui.screens.querySelector('[aria-pressed="true"]')?.focus();
            }, 'pt-design-screen');
            node.dataset.screen = id;
            node.setAttribute('aria-pressed', String(screen === id));
            node.disabled = busy || !loaded;
            return node;
        }));
        ui.inspector.inert = busy || !loaded;
        renderInspector();
        renderPreview();
        renderFooter();
        renderNotice();
    }

    function renderFooter() {
        const files = el('div');
        const actions = el('div');
        const importButton = button(text('import'), importProfile);
        const exportButton = button(text('export'), exportProfile);
        const resetButton = button(text('reset'), () => {
            stopAudioPreview();
            draft = {version: 1};
            notice('resetDraft');
            renderEditor();
            ui.footer.querySelector('.pt-design-reset')?.focus();
        }, 'pt-design-button pt-design-reset st-btn st-btn--secondary');
        const apply = button(text('apply'), saveProfile, 'pt-design-button pt-design-apply st-btn st-btn--gold');
        const cancel = button(text('cancel'), () => editor.close());
        for (const node of [importButton, exportButton, resetButton, apply]) node.disabled = busy || !loaded;
        cancel.disabled = busy;
        files.append(importButton, exportButton, resetButton);
        actions.append(cancel, apply);
        ui.footer.replaceChildren(files, actions);
    }

    function renderInspector() {
        coordInputs = {};
        ui.inspector.replaceChildren();
        if (tool === 'appearance') appearanceFields();
        if (tool === 'media') mediaFields();
        if (tool === 'sound') audioFields();
        if (tool === 'layout') layoutFields();
        if (tool === 'decor') decorFields();
    }

    function defaultAsset(slot) {
        if (slot === 'hubBackground') {
            const hero = hub.querySelector('.art-hero');
            if (draft.builtinStyle === 'panopticon') return hero?.tagName === 'IMG' ? panopticonPoster : panopticonVideo;
            return hero?.getAttribute('src') || defaults.hubBackground;
        }
        if (slot === 'gameMenuBackground' && draft.builtinStyle === 'panopticon') return panopticonVideo;
        return screen === 'loading' && slot === 'gameLogo' ? defaults.gameLoadingLogo : defaults[slot];
    }

    function rgbFromHex(hex) {
        return hex.match(/[a-f0-9]{2}/gi).map((value) => parseInt(value, 16));
    }

    function hexFromRgb(rgb) {
        return '#' + rgb.map((value) => Math.round(value).toString(16).padStart(2, '0')).join('');
    }

    function hsvFromRgb(rgb, previousHue = 0) {
        const [r, g, b] = rgb.map((value) => value / 255);
        const brightness = Math.max(r, g, b);
        const chroma = brightness - Math.min(r, g, b);
        let hue = previousHue;
        if (chroma !== 0) {
            const sector = brightness === r ? (g - b) / chroma : brightness === g ? 2 + (b - r) / chroma : 4 + (r - g) / chroma;
            hue = (sector * 60 + 360) % 360;
        }
        return {hue, saturation: brightness === 0 ? 0 : chroma / brightness, brightness};
    }

    function rgbFromHsv({hue, saturation, brightness}) {
        const sector = hue % 360 / 60;
        const chroma = saturation * brightness;
        const intermediate = chroma * (1 - Math.abs(sector % 2 - 1));
        const channels = [[chroma, intermediate, 0], [intermediate, chroma, 0], [0, chroma, intermediate],
            [0, intermediate, chroma], [intermediate, 0, chroma], [chroma, 0, intermediate]];
        return channels[Math.floor(sector)].map((value) => Math.round((value + brightness - chroma) * 255));
    }

    function accentFields() {
        const section = el('section', 'st-block pt-design-block');
        section.append(el('h2', 'st-block__head', text('accent')));
        const row = el('div', 'pt-design-color-row');
        const picker = el('div', 'pt-design-color-picker');
        picker.id = 'pt-design-color-picker';
        const choice = button('', () => setExpanded(picker.hidden), 'st-dd__btn pt-design-color-choice');
        identifyButton(choice, text('chooseColor'));
        choice.setAttribute('aria-controls', picker.id);
        const sample = el('span', 'pt-design-color-sample');
        const caret = el('span', 'pt-design-color-caret');
        choice.append(sample, caret);
        const hex = el('input', 'pt-design-hex');
        hex.type = 'text';
        hex.maxLength = 7;
        hex.spellcheck = false;
        hex.setAttribute('aria-label', text('accent') + ' HEX');
        const restore = button(text('restore'), () => selectHex());
        row.append(choice, hex, restore);

        const plane = el('button', 'pt-design-color-plane');
        plane.type = 'button';
        const point = el('span', 'pt-design-color-point');
        plane.append(point);
        const hue = el('input', 'st-volume__range pt-design-color-hue');
        hue.type = 'range';
        hue.min = '0';
        hue.max = '360';
        const rgbRow = el('div', 'pt-design-rgb');
        const channels = ['R', 'G', 'B'].map((label, index) => {
            const field = el('input');
            field.type = 'number';
            field.min = '0';
            field.max = '255';
            field.addEventListener('change', () => {
                if (field.value !== '' && Number.isFinite(field.valueAsNumber)) {
                    const next = [...rgb];
                    next[index] = clamp(field.valueAsNumber, 0, 255);
                    selectHex(hexFromRgb(next));
                } else sync();
            });
            rgbRow.append(titleRow(label, field));
            return field;
        });
        const palette = el('div', 'pt-design-palette');
        const swatches = colors.map((color) => {
            const swatch = button('', () => selectHex(color), 'pt-design-swatch');
            identifyButton(swatch, color);
            swatch.style.setProperty('--swatch', color);
            return swatch;
        });
        palette.append(...swatches);
        let rgb = rgbFromHex(stroke.test(draft.accent || '') ? draft.accent : defaultAccent);
        let hsv = hsvFromRgb(rgb);

        function setExpanded(open) {
            picker.hidden = !open;
            choice.setAttribute('aria-expanded', String(open));
        }

        function sync() {
            const value = hexFromRgb(rgb);
            hex.value = value;
            sample.style.backgroundColor = value;
            plane.style.setProperty('--pt-picker-hue', String(hsv.hue));
            point.style.left = hsv.saturation * 100 + '%';
            point.style.top = (1 - hsv.brightness) * 100 + '%';
            point.style.backgroundColor = value;
            plane.setAttribute('aria-label', [text('colorPlane'), text('saturation'), percent(hsv.saturation * 100) + '%.', text('brightness'), percent(hsv.brightness * 100) + '%.'].join(' '));
            hue.value = Math.round(hsv.hue);
            hue.setAttribute('aria-valuetext', hue.value + '°');
            channels.forEach((field, index) => { field.value = rgb[index]; });
            swatches.forEach((swatch, index) => swatch.setAttribute('aria-pressed', String(value === colors[index])));
        }

        function select(value) {
            if (value) draft.accent = value;
            else delete draft.accent;
            sync();
            setColor(editor, draft);
            ui.stage.classList.toggle('pt-design-game-default', screen !== 'hub' && !draft.accent);
        }

        function selectHex(value) {
            rgb = rgbFromHex(value || defaultAccent);
            hsv = hsvFromRgb(rgb, hsv.hue);
            select(value?.toLowerCase());
        }

        function selectHsv(values) {
            Object.assign(hsv, values);
            rgb = rgbFromHsv(hsv);
            select(hexFromRgb(rgb));
        }

        function pickPoint(event) {
            const bounds = plane.getBoundingClientRect();
            selectHsv({
                saturation: clamp((event.clientX - bounds.left) / bounds.width, 0, 1),
                brightness: clamp(1 - (event.clientY - bounds.top) / bounds.height, 0, 1),
            });
        }

        hex.addEventListener('change', () => {
            const value = '#' + hex.value.trim().replace(/^#/, '');
            if (stroke.test(value)) selectHex(value);
            else sync();
        });
        hue.addEventListener('input', () => selectHsv({hue: hue.valueAsNumber}));
        plane.addEventListener('pointerdown', (event) => {
            if (event.button !== 0 || !event.isPrimary) return;
            plane.setPointerCapture(event.pointerId);
            plane.focus({preventScroll: true});
            pickPoint(event);
        });
        plane.addEventListener('pointermove', (event) => {
            if (plane.hasPointerCapture(event.pointerId)) pickPoint(event);
        });
        plane.addEventListener('keydown', (event) => {
            const directions = {ArrowLeft: [-1, 0], ArrowRight: [1, 0], ArrowUp: [0, 1], ArrowDown: [0, -1]};
            const delta = directions[event.key];
            if (!delta) return;
            event.preventDefault();
            const step = event.shiftKey ? .1 : .01;
            selectHsv({
                saturation: clamp(hsv.saturation + delta[0] * step, 0, 1),
                brightness: clamp(hsv.brightness + delta[1] * step, 0, 1),
            });
        });
        section.addEventListener('keydown', (event) => {
            if (event.key !== 'Escape' || picker.hidden) return;
            event.preventDefault();
            setExpanded(false);
            choice.focus({preventScroll: true});
        });
        picker.append(plane, titleRow(text('hue'), hue), rgbRow);
        section.append(row, picker, palette);
        setExpanded(false);
        sync();
        return section;
    }

    function appearanceFields() {
        const identity = el('section', 'st-block pt-design-block');
        identity.append(el('h2', 'st-block__head', text('identity')));
        for (const field of ['hubName', 'gameName']) {
            const input = el('input');
            input.type = 'text';
            input.value = draft[field] || '';
            input.placeholder = name({}, field);
            input.maxLength = 64;
            input.addEventListener('input', () => {
                const value = input.value.trim();
                if (value) draft[field] = value;
                else delete draft[field];
                updateWindowIdentity();
            });
            identity.append(titleRow(text(field), input));
        }
        ui.inspector.append(accentFields(), identity);
        const presets = builtinThemeFields();
        if (presets) ui.inspector.append(presets);
    }

    function builtinThemeFields() {
        if (!themes?.panopticon || !themes.shinri) return;
        const section = el('section', 'st-block pt-design-block pt-design-builtin-styles');
        section.append(el('h2', 'st-block__head', text('builtinStyles')));
        const actions = el('div', 'pt-design-media-actions');
        for (const [id, label] of [['panopticon', 'Panopticon Trial'], ['shinri', 'Shinri Trial']]) {
            const choice = button(label, () => {
                const theme = themes[id];
                if (theme.accent) draft.accent = theme.accent;
                else delete draft.accent;
                if (theme.builtinStyle) draft.builtinStyle = theme.builtinStyle;
                else delete draft.builtinStyle;
                draft.backgroundFilters ||= {};
                for (const slot of themedBackgrounds) {
                    const filter = theme.backgroundFilters?.[slot];
                    if (filter) draft.backgroundFilters[slot] = {...filter};
                    else delete draft.backgroundFilters[slot];
                }
                notice(null);
                renderEditor();
                ui.inspector.querySelector('[data-theme="' + id + '"]')?.focus({preventScroll: true});
            });
            choice.dataset.theme = id;
            actions.append(choice);
        }
        section.append(actions);
        return section;
    }

    function mediaFields() {
        for (const [index, slot] of screens[screen].entries()) {
            const label = ['background', 'logo', 'windowIcon'][index];
            const block = el('section', 'st-block pt-design-block pt-design-media-row');
            const card = el('div', 'pt-design-media-card');
            const heading = el('div', 'pt-design-media-info');
            const display = el('div', 'pt-design-thumbnail');
            const labels = el('div');
            const allowed = index === 0 ? formats[screen === 'hub' ? 'hubBackground' : 'gameBackground'] : formats[index === 1 ? 'logo' : 'icon'];
            const custom = draft.assets?.[slot];
            showMedia(display, custom, defaultAsset(slot), () => notice('decodeError', true));
            labels.append(el('h2', 'st-set-name', text(label)), el('div', 'st-set-desc', allowed));
            heading.append(display, labels);
            const actions = el('div', 'pt-design-media-actions');
            const pick = button(text('choose'), () => pickAsset(slot));
            pick.setAttribute('aria-label', text(label) + ' · ' + text('choose'));
            actions.append(pick);
            if (custom) {
                const restore = button(text('restore'), () => {
                    delete draft.assets[slot];
                    notice(null);
                    renderEditor();
                });
                restore.setAttribute('aria-label', text(label) + ' · ' + text('restore'));
                actions.append(restore);
            }
            card.append(heading, actions);
            block.append(card);
            if (index === 0) block.append(backgroundFilterFields(slot, display));
            ui.inspector.append(block);
        }
    }

    function labelAudioButton(button, slot, playing) {
        const label = text(playing ? 'stop' : 'listen');
        button.textContent = label;
        identifyButton(button, text(slot) + ' · ' + label);
    }

    function stopAudioPreview() {
        if (!audioPreview) return;
        const {player, slot} = audioPreview;
        audioPreview = null;
        player.pause();
        player.removeAttribute('src');
        player.load();
        const listen = ui.inspector.querySelector('[data-pt-design-audio="' + slot + '"] [data-listen]');
        if (listen) labelAudioButton(listen, slot, false);
        window.PTHubMenuAudio?.setPreviewing(false);
    }

    function toggleAudioPreview(slot, url, listen) {
        const wasPlaying = audioPreview?.slot === slot;
        stopAudioPreview();
        if (wasPlaying) return;
        notice(null);
        const player = el('audio');
        audioPreview = {player, slot};
        window.PTHubMenuAudio?.setPreviewing(true);
        labelAudioButton(listen, slot, true);
        player.volume = slot === 'gameLoadingMusic' ? loadingMusicVolume() / 100 : .3;
        function failed() {
            if (audioPreview?.player !== player) return;
            stopAudioPreview();
            notice('audioError', true);
        }
        player.addEventListener('ended', () => {
            if (audioPreview?.player === player) stopAudioPreview();
        }, {once: true});
        player.addEventListener('error', failed, {once: true});
        player.src = url;
        player.play().catch(failed);
    }

    function loadingMusicVolume() {
        return draft.gameLoadingVolume ?? defaultLoadingVolume;
    }

    function loadingMusicControl(custom) {
        const row = el('label', 'pt-design-filter-row pt-design-loading-volume');
        const input = el('input', 'st-volume__range');
        const value = el('span', 'st-volume__value');
        input.type = 'range';
        input.min = '0';
        input.max = '100';
        input.value = String(loadingMusicVolume());
        input.disabled = !custom;
        input.setAttribute('aria-label', text('musicVolume'));
        value.setAttribute('aria-hidden', 'true');
        function render() {
            const percent = input.value + '%';
            input.style.setProperty('--st-volume', percent);
            input.setAttribute('aria-valuetext', percent);
            value.textContent = percent;
        }
        input.addEventListener('input', () => {
            const next = input.valueAsNumber;
            if (next === defaultLoadingVolume) delete draft.gameLoadingVolume;
            else draft.gameLoadingVolume = next;
            if (audioPreview?.slot === 'gameLoadingMusic') audioPreview.player.volume = next / 100;
            render();
        });
        render();
        row.append(el('span', '', text('musicVolume')), input, value);
        return row;
    }

    function audioFields() {
        const slot = soundScreens[screen];
        const custom = draft.assets?.[slot];
        const row = el('section', 'st-block pt-design-block pt-design-audio-row');
        row.dataset.ptDesignAudio = slot;
        row.append(el('h2', 'st-block__head', text(slot)));
        const url = audioKey.test(custom || '') ? source(custom) : defaults[slot];
        const state = text(custom ? 'customMusic' : url ? 'stockMusic' : 'silence');
        row.append(el('div', 'st-set-desc', state + ' · MP3 / OGG / WAV'));
        const actions = el('div', 'pt-design-media-actions');
        const focusChoose = () => {
            if (editor.open) ui.inspector.querySelector('[data-pt-design-audio="' + slot + '"] [data-choose]')?.focus();
        };
        const choose = button(text('choose'), async () => {
            await pickAsset(slot);
            focusChoose();
        });
        choose.dataset.choose = '';
        identifyButton(choose, text(slot) + ' · ' + text('choose'));
        actions.append(choose);
        if (url) {
            const listen = button('', () => toggleAudioPreview(slot, url, listen));
            listen.dataset.listen = '';
            labelAudioButton(listen, slot, audioPreview?.slot === slot);
            actions.append(listen);
        }
        if (custom) {
            const action = text(slot === 'gameMenuMusic' ? 'restore' : 'removeMusic');
            const restore = button(action, () => {
                stopAudioPreview();
                delete draft.assets[slot];
                if (slot === 'gameLoadingMusic') delete draft.gameLoadingVolume;
                notice(null);
                renderInspector();
                focusChoose();
            });
            identifyButton(restore, text(slot) + ' · ' + action);
            actions.append(restore);
        }
        row.append(actions);
        if (screen === 'loading') row.append(loadingMusicControl(custom));
        if (screen !== 'hub') {
            row.append(el('p', 'st-set-desc pt-design-audio-note', text(screen === 'menu' ? 'afterLaunchMusic' : 'afterLaunchLoadingMusic')));
        }
        ui.inspector.append(row);
    }

    function backgroundFilterFields(slot, thumbnail) {
        const panel = el('div', 'pt-design-correction');
        const heading = el('div', 'pt-design-position-heading');
        const reset = button(text('restore'), () => update({...initial}));
        reset.setAttribute('aria-label', text('correction') + ' · ' + text('restore'));
        heading.append(el('h3', 'st-set-name', text('correction')), reset);
        panel.append(heading);
        const initial = readBackgroundFilter({}, slot);
        let values = readBackgroundFilter(draft, slot);
        const controls = Object.entries(filterControls).map(([field, {min, max, unit}]) => {
            const row = el('label', 'pt-design-filter-row');
            const range = el('input', 'st-volume__range');
            range.type = 'range';
            range.min = String(min);
            range.max = String(max);
            const value = el('span', 'st-volume__value');
            value.setAttribute('aria-hidden', 'true');
            range.addEventListener('input', () => update({...values, [field]: range.valueAsNumber}));
            row.append(el('span', '', text(field)), range, value);
            panel.append(row);
            return {field, min, max, unit, range, value};
        });

        function sync() {
            for (const {field, min, max, unit, range, value} of controls) {
                const number = values[field];
                const label = number + unit;
                range.value = number;
                range.setAttribute('aria-valuetext', label);
                range.style.setProperty('--st-volume', (number - min) * 100 / (max - min) + '%');
                value.textContent = label;
            }
            reset.disabled = Object.keys(initial).every((field) => values[field] === initial[field]);
            applyBackgroundFilter(thumbnail, draft, slot);
        }

        function update(next) {
            values = next;
            const adjusted = Object.fromEntries(Object.entries(values).filter(([field, value]) => value !== initial[field]));
            if (Object.keys(adjusted).length > 0) {
                (draft.backgroundFilters ||= {})[slot] = adjusted;
            } else if (draft.backgroundFilters) {
                delete draft.backgroundFilters[slot];
                if (Object.keys(draft.backgroundFilters).length === 0) delete draft.backgroundFilters;
            }
            sync();
            applyBackgroundFilter(ui.backdrop, draft, slot);
        }

        sync();
        return panel;
    }

    function availableParts() {
        return Object.keys(parts).filter((id) => id !== 'promo' || hub.querySelector(parts.promo.selector) || draft.layout?.promo);
    }

    function changeLayout(values) {
        draft.layout ||= {};
        draft.layout[selection] = {...draft.layout[selection], ...values};
    }

    function layoutFields() {
        const list = el('div', 'pt-design-layout-list');
        for (const id of availableParts()) {
            const line = el('div', 'pt-design-layout-row');
            const select = button(text(id), () => {
                selection = id;
                renderInspector();
                placePreview();
                ui.inspector.querySelector('[aria-pressed="true"]')?.focus();
            }, 'pt-design-layout-select');
            select.setAttribute('aria-pressed', String(selection === id));
            line.append(select);
            if (!parts[id].required) {
                const toggle = el('input');
                toggle.type = 'checkbox';
                toggle.checked = !draft.layout?.[id]?.hidden;
                toggle.setAttribute('aria-label', text('shown') + ' · ' + text(id));
                toggle.addEventListener('change', () => {
                    selection = id;
                    changeLayout({hidden: !toggle.checked});
                    renderInspector();
                    renderPreview();
                    ui.inspector.querySelector('[aria-label="' + toggle.getAttribute('aria-label') + '"]')?.focus();
                });
                const control = el('label', 'st-toggle');
                control.append(toggle, el('span', 'st-toggle__track'), el('span', 'st-toggle__thumb'));
                line.append(control);
            }
            list.append(line);
        }
        const position = draft.layout?.[selection];
        const anchor = defaultPositions[selection] || parts[selection];
        ui.inspector.append(list, coordinates(position, anchor, changeLayout, () => {
            if (position?.hidden) draft.layout[selection] = {hidden: true};
            else delete draft.layout?.[selection];
        }));
    }

    function coordinates(position, anchor, change, reset, includeWidth = false) {
        const block = el('section', 'st-block pt-design-block pt-design-position');
        const heading = el('div', 'pt-design-position-heading');
        heading.append(el('h2', 'st-set-name', text('position')));
        if (reset) heading.append(button(text('restore'), () => {
            reset();
            renderInspector();
            placePreview();
        }));
        const fields = el('div', 'pt-design-coordinates');
        for (const axis of includeWidth ? ['x', 'y', 'width'] : ['x', 'y']) {
            const input = el('input');
            input.type = 'number';
            input.min = axis === 'width' ? '5' : '0';
            input.max = axis === 'width' ? '90' : '100';
            input.value = Math.round(geometry(position, axis, anchor[axis]));
            input.addEventListener('change', () => {
                const value = clamp(Number(input.value), Number(input.min), Number(input.max));
                input.value = value;
                change({[axis]: value});
                placePreview();
            });
            coordInputs[axis] = input;
            fields.append(titleRow(text(axis), input));
        }
        block.append(heading, fields);
        return block;
    }

    function decorFields() {
        const items = draft.decorations || [];
        const block = el('section', 'st-block pt-design-block pt-design-decor-panel');
        const add = button(text('add'), () => pickAsset('decoration'));
        add.disabled = items.length >= 8;
        const line = el('div', 'pt-design-position-heading');
        line.append(el('h2', 'st-set-name', text('decor')), add);
        block.append(line);
        if (!items.length) block.append(el('p', 'pt-design-tip', text('noDecor') + ' ' + formats.hubBackground));
        const choices = el('div', 'pt-design-decor-choices');
        items.forEach((item, index) => {
            const label = text('decoration') + ' ' + (index + 1);
            const choice = button('', () => {
                selectedDecor = index;
                renderInspector();
                placePreview();
                ui.inspector.querySelector('[aria-pressed="true"]')?.focus();
            }, 'pt-design-decor-choice');
            identifyButton(choice, label);
            choice.setAttribute('aria-pressed', String(index === selectedDecor));
            showMedia(choice, item.asset, '', () => notice('decodeError', true));
            choices.append(choice);
        });
        if (items.length) block.append(choices);
        ui.inspector.append(block);
        const current = items[selectedDecor];
        if (current) {
            ui.inspector.append(coordinates(current, current, (values) => Object.assign(current, values), null, true));
            ui.inspector.append(button(text('remove'), () => {
                items.splice(selectedDecor, 1);
                selectedDecor = Math.max(0, Math.min(selectedDecor, items.length - 1));
                notice(null);
                renderEditor();
            }, 'pt-design-button pt-design-remove st-btn st-btn--secondary'));
        }
    }

    function updateWindowIdentity() {
        ui.windowTitle.textContent = name(draft, screen === 'hub' ? 'hubName' : 'gameName');
        const slot = screens[screen][2];
        showMedia(ui.windowIcon, draft.assets?.[slot], defaults[slot], () => notice('decodeError', true));
    }

    function renderPreviewBackdrop() {
        const [slot] = screens[screen];
        showMedia(ui.backdrop, draft.assets?.[slot], defaultAsset(slot), () => notice('decodeError', true));
        applyBackgroundFilter(ui.backdrop, draft, slot);
    }

    function renderPreview() {
        const [, logoSlot] = screens[screen];
        updateWindowIdentity();
        if (ui.stage.dataset.screen !== screen) ui.stage.dataset.screen = screen;
        ui.stage.classList.toggle('pt-design-game-default', screen !== 'hub' && !draft.accent);
        ui.stockBase.hidden = ui.stockLighting.hidden = screen !== 'hub';
        renderPreviewBackdrop();
        ui.tip.textContent = screen !== 'hub' ? text('afterLaunch') : tool === 'layout' || tool === 'decor' ? text('layoutHint') : '';
        ui.tip.hidden = !ui.tip.textContent;
        ui.surface.replaceChildren();
        if (screen === 'hub') {
            for (const id of availableParts()) {
                if (draft.layout?.[id]?.hidden && !parts[id].required) continue;
                const part = el(tool === 'layout' ? 'button' : 'span', 'pt-design-part');
                part.dataset.part = id;
                if (tool === 'layout') {
                    part.type = 'button';
                    identifyButton(part, text(id));
                    part.classList.add('is-editable');
                }
                if (id === 'logo') {
                    showMedia(part, draft.assets?.hubLogo, defaults.hubLogo, () => notice('decodeError', true));
                } else if (id === 'onlinePanopticon' || id === 'onlineShinri') {
                    const count = hub.querySelector(parts[id].selector + ' .oc-txt b')?.textContent || '—';
                    const badge = el('img');
                    badge.src = id === 'onlinePanopticon' ? '/panopticon-online.png' : '/shinri-online.png';
                    badge.alt = '';
                    badge.draggable = false;
                    part.classList.add('pt-design-preview-online');
                    part.append(el('span', '', count), badge);
                } else {
                    part.textContent = icons[id] || text(id);
                }
                ui.surface.append(part);
            }
            const fixed = el('span', 'pt-design-part pt-design-preview-brush');
            fixed.innerHTML = brushIcon;
            ui.surface.append(fixed);
        } else {
            if (screen !== 'browser') {
                const logo = el('div', 'pt-design-game-logo');
                showMedia(logo, draft.assets?.[logoSlot], defaultAsset(logoSlot), () => notice('decodeError', true));
                ui.surface.append(logo);
            }
            if (screen === 'menu') {
                const selected = el('div', 'pt-design-game-menu');
                selected.append(el('span', '', text('activeServer')), el('b', '', text('noServer')));
                const buttons = el('div', 'pt-design-game-actions');
                buttons.setAttribute('aria-hidden', 'true');
                for (const icon of ['⚙', '◖◗', '↪']) buttons.append(el('span', '', icon));
                const music = el('span', 'pt-design-game-music', '♪');
                music.setAttribute('aria-hidden', 'true');
                ui.surface.append(selected, buttons, music);
            }
            if (screen === 'browser') {
                const panel = el('div', 'pt-design-game-list');
                const nav = el('div', 'pt-design-game-nav');
                nav.setAttribute('aria-hidden', 'true');
                for (const icon of ['⌂', '▤', '♡']) nav.append(el('span', '', icon));
                const tabs = el('div', 'pt-design-game-tabs');
                for (const labels of [['Panopticon Trial', 'Shinri Trial'], [text('localRegion'), 'Global']]) {
                    const pair = el('div');
                    for (const label of labels) pair.append(el('span', '', label));
                    tabs.append(pair);
                }
                panel.append(nav, tabs, el('strong', 'pt-design-game-servers-title', text('serverList')));
                for (const key of ['serverOne', 'serverTwo']) {
                    const row = el('div', 'pt-design-game-server');
                    row.append(el('span', '', text(key)), el('i'));
                    panel.append(row);
                }
                ui.surface.append(panel);
            }
            if (screen === 'loading') {
                const bar = el('div', 'pt-design-game-progress');
                bar.append(el('span', '', text('connecting')), el('i'));
                ui.surface.append(el('strong', 'pt-design-game-target', text('serverOne')), bar);
            }
        }
        placePreview();
    }

    function placePreview() {
        if (!draft || screen !== 'hub') {
            ui.decorations.replaceChildren();
            return;
        }
        updateDecor(ui.decorations, draft, true);
        const marker = ui.surface.querySelector('.pt-design-preview-brush');
        if (!marker) return;
        const measured = measurePreviewLayout();
        const {rect, fixed} = measured;
        if (!rect.width || !rect.height) return;
        const scaleX = ui.stage.clientWidth / rect.width;
        const scaleY = ui.stage.clientHeight / rect.height;
        function fit(node, box) {
            node.hidden = !box;
            if (!box) return;
            node.style.width = box.width * scaleX + 'px';
            node.style.height = box.height * scaleY + 'px';
            setCoords(node, box.x * scaleX, box.y * scaleY);
        }
        fit(marker, fixed);
        const placed = projectLayout(measured, draft);
        ui.surface.querySelectorAll('[data-part]').forEach((node) => {
            const id = node.dataset.part;
            fit(node, placed[id]);
            node.classList.toggle('is-selected', tool === 'layout' && selection === id);
        });
    }

    function moveTarget(node) {
        if (tool === 'layout') return node.closest('[data-part]');
        if (tool === 'decor') return node.closest('[data-decoration-index]');
        return null;
    }

    function selectTarget(target) {
        const next = tool === 'layout' ? target.dataset.part : Number(target.dataset.decorationIndex);
        if (tool === 'layout') selection = next;
        else selectedDecor = next;
        renderInspector();
        placePreview();
    }

    function moveTargetTo(x, y) {
        const point = {x: percent(x), y: percent(y)};
        if (tool === 'layout') changeLayout(point);
        else Object.assign(draft.decorations[selectedDecor], point);
        for (const axis of ['x', 'y']) if (coordInputs[axis]) coordInputs[axis].value = point[axis];
        placePreview();
    }

    function startMove(event) {
        if (busy || event.button !== 0 || !(event.target instanceof Element)) return;
        const target = moveTarget(event.target);
        if (!target) return;
        const rect = target.getBoundingClientRect();
        selectTarget(target);
        drag = {id: event.pointerId, x: event.clientX - rect.left - rect.width / 2, y: event.clientY - rect.top - rect.height / 2};
        target.focus({preventScroll: true});
        ui.stage.setPointerCapture(event.pointerId);
        event.preventDefault();
    }

    function move(event) {
        if (busy || drag?.id !== event.pointerId) return;
        const rect = ui.stage.getBoundingClientRect();
        moveTargetTo((event.clientX - drag.x - rect.left) * 100 / rect.width, (event.clientY - drag.y - rect.top) * 100 / rect.height);
    }

    function keyMove(event) {
        if (busy || !(event.target instanceof Element)) return;
        const target = moveTarget(event.target);
        if (!target) return;
        if (event.key === 'Enter' || event.key === ' ') {
            selectTarget(target);
            event.preventDefault();
            return;
        }
        const delta = {ArrowLeft: [-1, 0], ArrowRight: [1, 0], ArrowUp: [0, -1], ArrowDown: [0, 1]}[event.key];
        if (!delta) return;
        selectTarget(target);
        const step = event.shiftKey ? 10 : 2;
        moveTargetTo(Number(coordInputs.x.value) + delta[0] * step, Number(coordInputs.y.value) + delta[1] * step);
        event.preventDefault();
    }

    async function operation(work, errorMessage) {
        if (busy || !loaded) return;
        stopAudioPreview();
        busy = true;
        notice(null);
        renderEditor();
        try { await work(); }
        catch (error) { notice(errorMessage, true, error?.message); }
        finally {
            busy = false;
            if (editor.open) renderEditor();
        }
    }

    async function pickAsset(slot) {
        await operation(async () => {
            const key = await window.electron.invoke('designPickAsset', slot);
            if (!key) return;
            if (!await testMedia(key)) {
                notice('decodeError', true);
                return;
            }
            if (slot === 'decoration') {
                draft.decorations ||= [];
                selectedDecor = draft.decorations.length;
                draft.decorations.push({asset: key, x: 50, y: 48, width: 25});
            } else {
                draft.assets ||= {};
                draft.assets[slot] = key;
            }
        }, 'fileError');
    }

    async function checkProfile() {
        const keys = [...Object.values(draft.assets || {}), ...(draft.decorations || []).map((item) => item.asset)];
        const invalid = await Promise.all([...new Set(keys)].map(async (key) => [key, await testMedia(key)]));
        const issue = invalid.find(([, valid]) => !valid)?.[0];
        if (!issue) return true;
        const slot = Object.keys(draft.assets || {}).find((id) => draft.assets[id] === issue);
        if (slot) {
            const soundScreen = Object.keys(soundScreens).find((id) => soundScreens[id] === slot);
            tool = soundScreen ? 'sound' : 'media';
            screen = soundScreen || Object.keys(screens).find((id) => screens[id].includes(slot)) || 'hub';
        } else {
            tool = 'decor';
            screen = 'hub';
            selectedDecor = draft.decorations.findIndex((item) => item.asset === issue);
        }
        notice('decodeError', true);
        return false;
    }

    function importProfile() {
        return operation(async () => {
            const profile = await window.electron.invoke('designImport');
            if (!profile) return;
            draft = profile;
            selectedDecor = 0;
            if (await checkProfile()) notice('imported');
        }, 'importError');
    }

    function exportProfile() {
        return operation(async () => {
            if (await checkProfile() && await window.electron.invoke('designExport', draft)) notice('exported');
        }, 'exportError');
    }

    function saveProfile() {
        return operation(async () => {
            if (!await checkProfile()) return;
            const profile = await window.electron.invoke('designSave', draft);
            if (profile?.version !== 1) throw Error('Invalid design profile');
            saved = profile;
            editor.close();
            scheduleLive();
        }, 'saveError');
    }

    function visible() {
        windowVisible = nativeVisible && !document.hidden;
        if (!windowVisible) stopAudioPreview();
        document.querySelectorAll('.pt-design-actual-background video, .pt-design-actual-decor video, .pt-design video').forEach((node) => {
            if (windowVisible) node.play().catch(() => {});
            else node.pause();
        });
    }

    function start() {
        root = document.getElementById('root');
        if (!root || !window.electron) return;
        new MutationObserver(scheduleLive).observe(root, {subtree: true, childList: true, characterData: true, attributes: true, attributeFilter: ['disabled']});
        new MutationObserver(scheduleLive).observe(document.documentElement, {attributes: true, attributeFilter: ['lang']});
        window.addEventListener('resize', scheduleLive);
        let receivedVisibility = false;
        window.electron.on('hubWindowVisible', (value) => {
            receivedVisibility = true;
            nativeVisible = value;
            visible();
        });
        window.electron.invoke('isHubWindowVisible').then((value) => {
            if (!receivedVisibility && typeof value === 'boolean') {
                nativeVisible = value;
                visible();
            }
        }).catch(() => {});
        document.addEventListener('visibilitychange', visible);
        load();
        scheduleLive();
    }

    if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', start, {once: true});
    else start();
})();
