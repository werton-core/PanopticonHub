(function () {
    'use strict';

    var profile = window.PanopticonDesign;
    if (!profile || profile.version !== 1) profile = {};
    var root = document.documentElement;
    var isBrowserToolbar = root.hasAttribute('data-pt-browser');
    var mediaDir = './panopticon/design-media/';
    var imageExtensions = ['png', 'jpg', 'jpeg', 'webp', 'gif'];
    var backgroundExtensions = imageExtensions.concat('webm');
    var audioExtensions = ['mp3', 'wav', 'ogg'];
    var assets = profile.assets || {};
    var gameName = typeof profile.gameName === 'string' ? profile.gameName.trim() : '';
    var websiteStyle = profile.builtinStyle === 'panopticon';
    var iconColors;

    function mediaUrl(key, extensions) {
        var file = assets[key];
        var match = typeof file === 'string' && /^[0-9a-f]{64}\.([a-z0-9]+)$/.exec(file);
        return match && extensions.indexOf(match[1]) !== -1 ? mediaDir + file : '';
    }

    function luminance(rgb) {
        return rgb.reduce(function (sum, value, channel) {
            var normalized = value / 255;
            var linear = normalized <= 0.04045 ? normalized / 12.92 : Math.pow((normalized + 0.055) / 1.055, 2.4);
            return sum + linear * [0.2126, 0.7152, 0.0722][channel];
        }, 0);
    }

    if (typeof profile.accent === 'string' && /^#[0-9a-f]{6}$/i.test(profile.accent)) {
        var rgb = profile.accent.slice(1).match(/../g).map(function (pair) { return parseInt(pair, 16); });
        var websiteColors = websiteStyle && profile.accent.toLowerCase() === '#f0504f';
        var text = websiteColors ? [255, 156, 154] : rgb;
        while (luminance(text) < 0.24) {
            text = text.map(function (value) { return Math.round(value + (255 - value) * 0.12); });
        }
        root.style.setProperty('--pt-accent', profile.accent);
        if (websiteColors) root.style.setProperty('--pt-accent-fill', '#d93a41');
        root.style.setProperty('--pt-accent-rgb', rgb.join(', '));
        root.style.setProperty('--pt-accent-text-rgb', text.join(', '));
        root.style.setProperty('--pt-accent-text', 'rgb(' + text.join(', ') + ')');
        var pale = text.map(function (value) { return Math.round(value + (255 - value) * 0.29); });
        root.style.setProperty('--pt-accent-pale-rgb', pale.join(', '));
        function shade(level) {
            return 'rgb(' + text.map(function (value) { return Math.round(value * level / 100); }).join(', ') + ')';
        }
        [22, 38, 53, 55].forEach(function (level) {
            root.style.setProperty('--pt-accent-depth-' + level, shade(level));
        });
        iconColors = {
            f2d9a6: shade(100),
            ffeac0: 'rgb(' + pale.join(', ') + ')',
            fff1d5: 'rgb(' + pale.join(', ') + ')',
            ffe4ae: shade(100),
            dcc57b: shade(100),
            '5e441e': shade(38),
            '6f5021': shade(53),
            '120e02': shade(7),
            '201904': shade(13),
            '241b05': shade(15)
        };
        root.style.setProperty('--pt-accent-ink', !websiteColors && luminance(rgb) > 0.179 ? '#000' : '#fff');
        root.classList.add('pt-design-accent');
    }

    var logoUrl = !isBrowserToolbar && mediaUrl('gameLogo', imageExtensions);
    if (logoUrl) {
        var logo = new Image();
        logo.addEventListener('load', function () {
            root.style.setProperty('--pt-game-logo', 'url("' + logo.src + '")');
            root.classList.add('pt-design-logo');
        }, { once: true });
        logo.src = logoUrl;
    }

    var iconUrl = mediaUrl('gameIcon', imageExtensions.concat('ico'));
    if (iconUrl) {
        var icon = document.createElement('link');
        icon.rel = 'icon';
        icon.href = iconUrl;
        document.head.appendChild(icon);
    }

    function backgroundUrl(key) {
        return mediaUrl(key, backgroundExtensions);
    }

    function backgroundFilter(key) {
        var settings = profile.backgroundFilters && profile.backgroundFilters[key];
        if (!settings) return '';
        function value(value, min, max, fallback) {
            return Number.isFinite(value) && value >= min && value <= max ? value : fallback;
        }
        var hue = value(settings.hue, -180, 180, 0);
        var saturation = value(settings.saturation, 0, 200, 100);
        var brightness = value(settings.brightness, 0, 200, 100);
        if (hue === 0 && saturation === 100 && brightness === 100) return '';
        return 'hue-rotate(' + hue + 'deg) saturate(' + saturation + '%) brightness(' + brightness + '%)';
    }

    var menuFilter = backgroundFilter('gameMenuBackground');
    if (menuFilter) {
        root.classList.add('pt-design-filter-menu');
        root.style.setProperty('--pt-design-menu-filter', menuFilter);
    }
    var loadingFilter = backgroundFilter('gameLoadingBackground');
    if (loadingFilter && document.querySelector('.enter-screen')) {
        root.classList.add('pt-design-filter-loading');
        root.style.setProperty('--pt-design-loading-filter', loadingFilter);
    }
    var chooserFilter = backgroundFilter('gameBrowserBackground');

    function stockChooser(container) {
        var carousel = document.createElement('div');
        carousel.className = 'pt-design-stock-chooser';
        carousel.style.filter = chooserFilter;
        [1, 2, 3].forEach(function (number) {
            var img = document.createElement('img');
            img.alt = '';
            img.src = './panopticon/backdrops/academy-0' + number + '.jpg';
            carousel.appendChild(img);
        });
        container.classList.add('pt-design-backdrop--stock');
        container.appendChild(carousel);
        return carousel;
    }

    function hasBackground(key) {
        return !!backgroundUrl(key) || (key === 'gameBrowserBackground' && !!chooserFilter);
    }

    function mountBackground(container, key, onReady) {
        var url = backgroundUrl(key);
        var carousel = key === 'gameBrowserBackground' && chooserFilter && stockChooser(container);

        function removeStock() {
            if (carousel) {
                carousel.remove();
                container.classList.remove('pt-design-backdrop--stock');
            }
        }

        if (!url) return removeStock;

        var isVideo = url.endsWith('.webm');
        var media = document.createElement(isVideo ? 'video' : 'img');
        var disposed = false;
        var ready = false;
        media.className = 'pt-design-backdrop__media';
        var filter = backgroundFilter(key);
        if (filter) media.style.setProperty('--pt-design-media-filter', filter);

        function show() {
            if (disposed || ready) return;
            ready = true;
            container.classList.add('pt-design-backdrop--ready');
            if (onReady) onReady(true);
        }

        function removeCustom() {
            if (disposed) return;
            disposed = true;
            document.removeEventListener('visibilitychange', updatePlayback);
            if (isVideo) {
                media.pause();
                media.removeAttribute('src');
                media.load();
            }
            media.remove();
            container.classList.remove('pt-design-backdrop--ready');
        }

        function fallback() {
            if (disposed) return;
            removeCustom();
            if (ready && onReady) onReady(false);
        }

        function updatePlayback() {
            if (document.hidden) media.pause();
            else media.play().catch(function (error) {
                if (error.name !== 'AbortError') fallback();
            });
        }

        media.addEventListener('error', fallback, { once: true });
        if (isVideo) {
            media.loop = true;
            media.muted = true;
            media.defaultMuted = true;
            media.playsInline = true;
            media.disablePictureInPicture = true;
            media.addEventListener('playing', function () {
                if (media.videoWidth) show();
                else fallback();
            }, { once: true });
            document.addEventListener('visibilitychange', updatePlayback);
        } else {
            media.alt = '';
            media.addEventListener('load', show, { once: true });
        }
        container.appendChild(media);
        media.src = url;
        if (isVideo) updatePlayback();
        return function () {
            removeCustom();
            removeStock();
        };
    }

    function accentIcon(url) {
        if (!iconColors) return url;
        // Preserve the geometry and opacity of each packaged menu glyph.
        return url.replace(/%23([0-9a-f]{6})\b/gi, function (match, color) {
            var themed = iconColors[color.toLowerCase()];
            return themed ? encodeURIComponent(themed) : match;
        });
    }

    var loadingVolume = profile.gameLoadingVolume;
    if (!Number.isInteger(loadingVolume) || loadingVolume < 0 || loadingVolume > 100) loadingVolume = 30;

    window.PTHubGameDesign = {
        gameName: gameName,
        websiteStyle: websiteStyle,
        menuMusicUrl: mediaUrl('gameMenuMusic', audioExtensions),
        loadingMusicUrl: mediaUrl('gameLoadingMusic', audioExtensions),
        loadingMusicVolume: loadingVolume / 100,
        hasBackground: hasBackground,
        mountBackground: mountBackground,
        accentIcon: accentIcon
    };

    if (gameName) {
        document.title = isBrowserToolbar ? gameName + ' · Browser' : gameName;
    }
    document.addEventListener('DOMContentLoaded', function () {
        document.querySelectorAll('[data-pt-background]').forEach(function (container) {
            mountBackground(container, container.dataset.ptBackground);
        });
        var loadingLogo = document.querySelector('.enter-screen__logo');
        if (gameName && loadingLogo) loadingLogo.alt = gameName;
    });
}());
