(function () {
    'use strict';

    const copy = {
        ru: {
            title: 'Выберите оформление хаба',
            panopticon: 'Красный акцент', shinri: 'Оригинальное оформление', play: 'Играть',
            note: 'Стиль можно изменить в разделе «Оформление».',
            saving: 'Сохранение оформления…', error: 'Не удалось сохранить оформление. Выберите стиль ещё раз.',
        },
        en: {
            title: 'Choose your Hub style',
            panopticon: 'Red accent', shinri: 'Original style', play: 'Play',
            note: 'You can change the style in Appearance.',
            saving: 'Saving appearance…', error: 'Could not save your appearance. Choose a style again to retry.',
        },
        fr: {
            title: 'Choisissez le style du hub',
            panopticon: 'Accent rouge', shinri: 'Style d’origine', play: 'Jouer',
            note: 'Vous pouvez changer de style dans Apparence.',
            saving: 'Enregistrement de l’apparence…', error: 'Impossible d’enregistrer l’apparence. Choisissez de nouveau un style.',
        },
    };
    const styles = {
        panopticon: {
            name: 'Panopticon Trial', accent: '#f0504f',
            poster: '/design-defaults/panopticon-background.jpg', movie: '/design-defaults/panopticon-background.webm',
        },
        shinri: {
            name: 'Shinri Trial', accent: '#ebcb7c',
            poster: './background-Bd5Lor2N.jpg', movie: './background-q0xJoMR6.webm',
        },
    };
    let active;
    let mousePosition;

    function isMouse(event) {
        return (!event.pointerType || event.pointerType === 'mouse') && !event.sourceCapabilities?.firesTouchEvents;
    }

    function rememberMouse(event) {
        mousePosition = isMouse(event) ? {x: event.clientX, y: event.clientY} : null;
    }

    for (const name of ['pointermove', 'pointerdown', 'mousemove']) {
        document.addEventListener(name, rememberMouse, {capture: true, passive: true});
    }
    document.documentElement.addEventListener('mouseleave', () => { mousePosition = undefined; });
    window.addEventListener('blur', () => { mousePosition = undefined; });

    function el(tag, className, label) {
        const node = document.createElement(tag);
        if (className) node.className = className;
        if (label) node.textContent = label;
        return node;
    }

    function bounded(value, min, max, fallback) {
        return Number.isFinite(value) ? Math.round(Math.min(max, Math.max(min, value))) : fallback;
    }

    function makeCard(id, style, theme) {
        const button = el('button', 'pt-initial-style-card');
        button.type = 'button';
        button.dataset.style = id;
        button.style.setProperty('--pt-style-accent', /^#[0-9a-f]{6}$/i.test(theme?.accent || '') ? theme.accent : style.accent);
        const preview = el('span', 'pt-initial-style-preview');
        preview.setAttribute('aria-hidden', 'true');
        const art = el('span', 'pt-initial-style-art');
        const image = el('img');
        image.src = style.poster;
        image.alt = '';
        const filter = theme?.backgroundFilters?.hubBackground;
        if (filter) {
            const hue = bounded(filter.hue, -180, 180, 0);
            const saturation = bounded(filter.saturation, 0, 200, 100);
            const brightness = bounded(filter.brightness, 0, 200, 100);
            if (hue !== 0 || saturation !== 100 || brightness !== 100) {
                art.style.filter = `hue-rotate(${hue}deg) saturate(${saturation}%) brightness(${brightness}%)`;
            }
        }
        art.append(image);
        const logo = el('img', 'pt-initial-style-logo');
        logo.src = './panopticon-logo.png';
        logo.alt = '';
        const play = el('span', 'pt-initial-style-play');
        preview.append(art, el('span', 'pt-initial-style-shade'), logo, play);

        const label = el('span', 'pt-initial-style-label');
        const name = el('span', 'pt-initial-style-name', style.name);
        const description = el('span', 'pt-initial-style-description');
        name.id = `pt-initial-style-name-${id}`;
        description.id = `pt-initial-style-description-${id}`;
        label.append(name, description);
        button.setAttribute('aria-labelledby', name.id);
        button.setAttribute('aria-describedby', description.id);
        button.append(preview, label);
        return {id, button, art, description, play, movie: style.movie};
    }

    function open({themes, select}) {
        if (active) return active;

        const dialog = el('dialog', 'modal pt-initial-style no-drag-window');
        const header = el('header', 'modal-head');
        const heading = el('h1', 'modal-title');
        heading.id = 'pt-initial-style-title';
        heading.tabIndex = -1;
        const body = el('div', 'modal-body pt-initial-style-cards');
        const footer = el('footer', 'pt-initial-style-footer');
        const note = el('p');
        note.id = 'pt-initial-style-note';
        const status = el('p', 'pt-initial-style-status');
        status.setAttribute('role', 'status');
        dialog.setAttribute('aria-labelledby', heading.id);
        dialog.setAttribute('aria-describedby', note.id);
        dialog.setAttribute('closedby', 'none');
        const cards = Object.entries(styles).map(([id, style]) => makeCard(id, style, themes?.[id]));
        header.append(heading);
        body.append(...cards.map(({button}) => button));
        footer.append(note, status);
        dialog.append(header, body, footer);

        const lifecycle = new AbortController();
        const options = {signal: lifecycle.signal};
        const reduced = window.matchMedia('(prefers-reduced-motion: reduce)');
        const native = window.electron;
        let nativeVisible = true;
        let receivedVisibility = false;
        let preview;
        let pointedCard;
        let selection;
        let failed = false;
        let result;
        let resolve;

        function renderCopy() {
            const labels = copy[document.documentElement.lang.toLowerCase().split('-')[0]] || copy.en;
            heading.textContent = labels.title;
            note.textContent = labels.note;
            for (const {id, description, play} of cards) {
                description.textContent = labels[id];
                play.textContent = labels.play;
            }
            status.textContent = failed ? labels.error : selection ? labels.saving : '';
            status.classList.toggle('is-error', failed);
        }

        function releaseVideo() {
            if (!preview) return;
            const video = preview;
            preview = undefined;
            video.pause();
            video.removeAttribute('src');
            video.load();
            video.remove();
        }

        function stopPreview() {
            pointedCard = undefined;
            releaseVideo();
        }

        function startPreview(card) {
            if (pointedCard === card || !nativeVisible || document.hidden || !dialog.open || selection) return;
            releaseVideo();
            pointedCard = card;
            const video = el('video', 'pt-initial-style-video');
            video.muted = true;
            video.defaultMuted = true;
            video.playsInline = true;
            video.loop = true;
            video.preload = 'auto';
            const showFrame = () => {
                if (preview === video && video.readyState >= HTMLMediaElement.HAVE_CURRENT_DATA) {
                    video.classList.add('is-playing');
                }
            };
            const restorePoster = () => {
                if (preview === video) releaseVideo();
            };
            video.addEventListener('loadeddata', showFrame, {once: true});
            video.addEventListener('playing', showFrame, {once: true});
            video.addEventListener('error', restorePoster, {once: true});
            preview = video;
            card.art.append(video);
            video.src = card.movie;
            video.play().then(showFrame, restorePoster);
        }

        async function choose(card) {
            if (selection || !dialog.open) return;
            stopPreview();
            failed = false;
            selection = card;
            for (const {button} of cards) button.setAttribute('aria-disabled', 'true');
            card.button.classList.add('is-saving');
            renderCopy();
            try {
                result = await select(card.id);
                dialog.close();
            } catch (_) {
                selection = undefined;
                failed = true;
                for (const {button} of cards) button.removeAttribute('aria-disabled');
                card.button.classList.remove('is-saving');
                renderCopy();
            }
        }

        function visibilityChanged(visible) {
            if (typeof visible !== 'boolean') return;
            receivedVisibility = true;
            nativeVisible = visible;
            if (!visible) stopPreview();
        }

        for (const card of cards) {
            card.button.addEventListener('click', () => choose(card));
            const enter = (event) => {
                if (!reduced.matches && isMouse(event)) startPreview(card);
            };
            const move = (event) => {
                if (isMouse(event)) startPreview(card);
            };
            const leave = (event) => {
                if (pointedCard === card && isMouse(event)) stopPreview();
            };
            card.button.addEventListener('pointerenter', enter);
            card.button.addEventListener('mouseenter', enter);
            card.button.addEventListener('pointermove', move);
            card.button.addEventListener('mousemove', move);
            card.button.addEventListener('pointerleave', leave);
            card.button.addEventListener('mouseleave', leave);
            card.button.addEventListener('pointercancel', stopPreview);
        }
        dialog.addEventListener('keydown', (event) => {
            if (event.key === 'Escape') event.preventDefault();
        });
        dialog.addEventListener('cancel', (event) => event.preventDefault());
        dialog.addEventListener('close', () => {
            stopPreview();
            lifecycle.abort();
            locale.disconnect();
            native?.removeListener?.('hubWindowVisible', visibilityChanged);
            dialog.remove();
            active = undefined;
            resolve(result);
        }, {once: true});
        const locale = new MutationObserver(renderCopy);
        locale.observe(document.documentElement, {attributes: true, attributeFilter: ['lang']});
        window.addEventListener('blur', stopPreview, options);
        window.addEventListener('pagehide', stopPreview, options);
        document.addEventListener('visibilitychange', () => {
            if (document.hidden) stopPreview();
        }, options);
        reduced.addEventListener('change', stopPreview, options);
        native?.on?.('hubWindowVisible', visibilityChanged);
        native?.invoke?.('isHubWindowVisible').then((visible) => {
            if (!receivedVisibility) visibilityChanged(visible);
        }).catch(() => {});
        renderCopy();
        active = new Promise((complete) => { resolve = complete; });
        document.body.append(dialog);
        dialog.showModal();
        heading.focus({preventScroll: true});
        requestAnimationFrame(() => {
            if (reduced.matches || mousePosition === null) return;
            const node = mousePosition ? document.elementFromPoint(mousePosition.x, mousePosition.y) : dialog.querySelector('button:hover');
            const card = cards.find(({button}) => button.contains(node));
            if (card) startPreview(card);
        });
        return active;
    }

    window.PTHubInitialStyle = {open};
})();
