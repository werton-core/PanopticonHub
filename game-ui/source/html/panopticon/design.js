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

    function readableAccent(rgb, minimum) {
        // Keep the chosen hue and saturation while raising brightness first.
        var peak = Math.max.apply(Math, rgb);
        var vivid = peak ? rgb.map(function (value) { return Math.round(value * 255 / peak); }) : [0, 0, 0];
        function at(amount) {
            return amount <= 1
                ? rgb.map(function (value, index) { return Math.round(value + (vivid[index] - value) * amount); })
                : vivid.map(function (value) { return Math.round(value + (255 - value) * (amount - 1)); });
        }
        if (luminance(rgb) >= minimum) return rgb;
        var low = 0, high = 1;
        if (luminance(vivid) < minimum) { low = 1; high = 2; }
        for (var step = 0; step < 12; step++) {
            var middle = (low + high) / 2;
            if (luminance(at(middle)) < minimum) low = middle;
            else high = middle;
        }
        return at(high);
    }

    if (typeof profile.accent === 'string' && /^#[0-9a-f]{6}$/i.test(profile.accent)) {
        var rgb = profile.accent.slice(1).match(/../g).map(function (pair) { return parseInt(pair, 16); });
        var websiteColors = websiteStyle && profile.accent.toLowerCase() === '#f0504f';
        var text = websiteColors ? [255, 102, 86] : websiteStyle ? readableAccent(rgb, .24) : rgb;
        while (!websiteStyle && luminance(text) < 0.24) {
            text = text.map(function (value) { return Math.round(value + (255 - value) * 0.12); });
        }
        root.style.setProperty('--pt-accent', profile.accent);
        if (websiteColors) root.style.setProperty('--pt-accent-fill', '#d93a41');
        root.style.setProperty('--pt-accent-rgb', rgb.join(', '));
        root.style.setProperty('--pt-accent-text-rgb', text.join(', '));
        root.style.setProperty('--pt-accent-text', 'rgb(' + text.join(', ') + ')');
        var pale = websiteStyle ? readableAccent(text, .3) : text.map(function (value) { return Math.round(value + (255 - value) * 0.29); });
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
        if (websiteStyle) root.classList.add('pt-design-panopticon');
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

    var gameWindowVisible;

    function mediaVisible() {
        return gameWindowVisible === undefined ? !document.hidden : gameWindowVisible;
    }

    window.PTHubGameMediaWake = function (visible, forceWake) {
        if (typeof visible !== 'boolean' || (gameWindowVisible === visible && !(visible && forceWake))) return;
        gameWindowVisible = visible;
        document.dispatchEvent(new CustomEvent('pt-game-media-visibility', { detail: visible && !!forceWake }));
    };

    function watchMediaVisibility(onChange) {
        var visible = mediaVisible();
        function visibilityChanged(event) {
            var next = mediaVisible();
            var forcedWake = next && event.type === 'pt-game-media-visibility' && event.detail;
            if (next === visible && !forcedWake) return;
            visible = next;
            onChange(visible, true);
        }
        function wake() {
            if (visible) onChange(true, false);
        }
        document.addEventListener('visibilitychange', visibilityChanged);
        document.addEventListener('pt-game-media-visibility', visibilityChanged);
        window.addEventListener('focus', wake);
        window.addEventListener('pageshow', wake);
        return function () {
            document.removeEventListener('visibilitychange', visibilityChanged);
            document.removeEventListener('pt-game-media-visibility', visibilityChanged);
            window.removeEventListener('focus', wake);
            window.removeEventListener('pageshow', wake);
        };
    }

    function watchLoopVideo(video, canPlay, onFailure) {
        var stopped = false;
        var suspended = false;
        var pending;

        function updatePlayback(refreshFrame) {
            if (stopped) return;
            if (!mediaVisible() || !canPlay()) {
                pending = undefined;
                suspended = true;
                if (!video.paused) video.pause();
                return;
            }
            if (refreshFrame || suspended) {
                suspended = false;
                // CEF can retain the last texture when its window stops presenting frames.
                if (video.readyState >= HTMLMediaElement.HAVE_METADATA && video.currentTime > 0 && !video.seeking) {
                    video.currentTime = video.currentTime;
                }
            }
            if (!video.paused || pending) return;
            var attempt = video.play();
            pending = attempt;
            attempt.then(function () {
                if (pending === attempt) pending = undefined;
            }, function (error) {
                if (pending !== attempt) return;
                pending = undefined;
                if (error.name === 'NotSupportedError') onFailure();
            });
        }

        function retry() { updatePlayback(false); }
        function failed() {
            if (video.error && video.error.code !== MediaError.MEDIA_ERR_ABORTED) onFailure();
        }
        var unwatch = watchMediaVisibility(function (visible, transitioned) {
            if (visible && transitioned && pending) {
                pending = undefined;
                video.pause();
            }
            updatePlayback(visible);
        });
        window.addEventListener('pointerdown', retry);
        video.addEventListener('error', failed);
        retry();
        var stop = function () {
            stopped = true;
            pending = undefined;
            unwatch();
            window.removeEventListener('pointerdown', retry);
            video.removeEventListener('error', failed);
            video.pause();
        };
        stop.updatePlayback = retry;
        return stop;
    }

    function mountBackground(container, key, onReady, canPlay) {
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
        var playback;
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
            if (isVideo) {
                if (playback) playback();
                media.removeAttribute('src');
                media.load();
            }
            media.remove();
            container.classList.remove('pt-design-backdrop--ready');
        }

        function fallback() {
            if (disposed) return;
            removeCustom();
            if (onReady) onReady(false);
        }

        if (isVideo) {
            media.loop = true;
            media.muted = true;
            media.defaultMuted = true;
            media.playsInline = true;
            media.disablePictureInPicture = true;
            function showDecodedFrame() { if (media.videoWidth) show(); }
            media.addEventListener('loadeddata', showDecodedFrame, { once: true });
            media.addEventListener('playing', showDecodedFrame, { once: true });
        } else {
            media.alt = '';
            media.addEventListener('load', show, { once: true });
            media.addEventListener('error', fallback, { once: true });
        }
        container.appendChild(media);
        media.src = url;
        if (isVideo) playback = watchLoopVideo(media, canPlay || function () { return true; }, fallback);
        var unmount = function () {
            removeCustom();
            removeStock();
        };
        if (playback) unmount.updatePlayback = playback.updatePlayback;
        return unmount;
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
        mediaVisible: mediaVisible,
        watchMediaVisibility: watchMediaVisibility,
        watchLoopVideo: watchLoopVideo,
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
