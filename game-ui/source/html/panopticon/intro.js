(function () {
    'use strict';

    var CHANGE_LAYOUT_AT = 7.30;
    var SHOW_WARNING_AT = 11.08;
    var stablePaintMs = 300;
    var engineReady = window.location.protocol !== 'asset:';
    var skipLoadingScreen = window.PanopticonDesign && window.PanopticonDesign.skipLoadingScreen === true;
    var introAlreadyPlayed = false;
    var finishAlreadyPlayed;

    var overlay = document.getElementById('pt-intro');
    var video = overlay.querySelector('video');
    var curtain = overlay.querySelector('.pt-intro__curtain');
    var fallback = overlay.querySelector('.pt-intro__fallback');
    var message = overlay.querySelector('.pt-intro__message');
    var button = overlay.querySelector('.pt-intro__hit');
    if (!skipLoadingScreen) {
        video.src = video.dataset.src;
        fallback.src = fallback.dataset.src;
    }

    function completeIntro() {
        if (window.lua && typeof window.lua.Run === 'function') {
            window.lua.Run('stMenu.CompleteIntro()');
        }
    }

    function update(title, body, age) {
        message.querySelector('h1').textContent = title;
        message.querySelector('.pt-intro__age').textContent = age;
        var copy = message.querySelector('.pt-intro__copy');
        copy.textContent = '';
        body.split(/\r?\n\r?\n/).forEach(function (paragraph) {
            var p = document.createElement('p');
            paragraph.split(/\r?\n/).forEach(function (line, index) {
                if (index) p.appendChild(document.createElement('br'));
                line.split(/(<b>[\s\S]*?<\/b>)/).forEach(function (part) {
                    var bold = /^<b>([\s\S]*)<\/b>$/.exec(part);
                    if (bold) {
                        var strong = document.createElement('strong');
                        strong.textContent = bold[1];
                        p.appendChild(strong);
                    } else {
                        p.appendChild(document.createTextNode(part));
                    }
                });
            });
            copy.appendChild(p);
        });
    }

    function start(onDone) {
        if (introAlreadyPlayed) {
            onDone();
            return;
        }
        if (skipLoadingScreen) {
            introAlreadyPlayed = true;
            completeIntro();
            onDone();
            return;
        }
        var done = false;
        var closing = false;
        var showingFallback = false;
        var paintId;
        var videoId;
        var playAttempt = 0;
        var fallbackTimer;
        var transitionTimer;
        var presentedTime = 0;
        var recoveries = 0;
        var waitingSince;
        var stableSince;
        var lastPaint;
        var frameCallbacks = typeof video.requestVideoFrameCallback === 'function';
        var design = window.PTHubGameDesign;
        var previousFocus = document.activeElement;
        overlay.hidden = false;
        video.hidden = false;
        fallback.hidden = true;
        syncWarning(0);
        if (video.currentTime !== 0) video.currentTime = 0;
        button.focus({ preventScroll: true });

        function cancelFrames() {
            playAttempt++;
            cancelAnimationFrame(paintId);
            if (videoId !== undefined && video.cancelVideoFrameCallback) video.cancelVideoFrameCallback(videoId);
            paintId = undefined;
            videoId = undefined;
        }

        function stop(notify) {
            if (done) return;
            done = true;
            cancelFrames();
            clearTimeout(fallbackTimer);
            clearTimeout(transitionTimer);
            document.removeEventListener('keydown', skipKey, true);
            overlay.removeEventListener('click', skip, true);
            overlay.removeEventListener('auxclick', skip, true);
            overlay.removeEventListener('contextmenu', skip, true);
            overlay.removeEventListener('animationend', transitionEnded);
            video.removeEventListener('ended', finished);
            video.removeEventListener('error', showFallback);
            unwatchVisibility();
            video.pause();
            overlay.hidden = true;
            overlay.classList.remove('pt-intro--closing', 'pt-intro--revealing', 'pt-intro--mark');
            finishAlreadyPlayed = undefined;
            if (previousFocus && document.contains(previousFocus)) previousFocus.focus();
            if (notify) {
                completeIntro();
                onDone();
            }
        }

        function revealMenu() {
            if (done || !overlay.classList.replace('pt-intro--closing', 'pt-intro--revealing')) return;
            clearTimeout(transitionTimer);
            transitionTimer = setTimeout(function () { stop(true); }, 350);
        }

        function transitionEnded(event) {
            if (event.target === curtain && event.animationName === 'pt-intro-cover') revealMenu();
            if (event.target === overlay && event.animationName === 'pt-intro-reveal') stop(true);
        }

        function finish() {
            if (done || closing) return;
            closing = true;
            cancelFrames();
            clearTimeout(fallbackTimer);
            video.pause();
            overlay.classList.add('pt-intro--closing');
            transitionTimer = setTimeout(revealMenu, 350);
        }

        function skip(event) {
            event.preventDefault();
            event.stopImmediatePropagation();
            finish();
        }

        function skipKey(event) {
            if (event.code === 'Space' || event.key === ' ' || event.key === 'Spacebar') skip(event);
        }

        function syncWarning(time) {
            var showMark = time >= CHANGE_LAYOUT_AT;
            video.classList.toggle('pt-intro__video--mark', showMark);
            overlay.classList.toggle('pt-intro--mark', showMark);
            message.hidden = time < SHOW_WARNING_AT;
        }

        function showFallback() {
            if (done || closing || showingFallback) return;
            showingFallback = true;
            cancelFrames();
            video.pause();
            video.hidden = true;
            fallback.hidden = false;
            syncWarning(SHOW_WARNING_AT);
            fallbackTimer = setTimeout(finish, 5100);
        }

        function recover() {
            if (recoveries++ >= 2) {
                showFallback();
                return;
            }
            video.pause();
            video.currentTime = presentedTime;
            waitForPaint();
        }

        function finished() {
            if (done || closing) return;
            if (frameCallbacks && presentedTime < SHOW_WARNING_AT) recover();
            else finish();
        }

        function onVideoFrame(_, metadata) {
            videoId = undefined;
            if (done || closing || showingFallback) return;
            // После остановки отрисовки продолжить с последнего показанного кадра.
            if (metadata.mediaTime - presentedTime > 0.45) {
                recover();
                return;
            }
            presentedTime = metadata.mediaTime;
            syncWarning(presentedTime);
            videoId = video.requestVideoFrameCallback(onVideoFrame);
        }

        function onLegacyFrame() {
            presentedTime = video.currentTime;
            syncWarning(presentedTime);
            paintId = requestAnimationFrame(onLegacyFrame);
        }

        function startWhenVisible(now) {
            paintId = undefined;
            if (!design.mediaVisible()) return;
            var mediaReady = video.readyState >= HTMLMediaElement.HAVE_FUTURE_DATA && !video.seeking;
            var waited = now - waitingSince;
            if (!mediaReady || now - lastPaint > 125) stableSince = undefined;
            else if (stableSince === undefined) stableSince = now;
            lastPaint = now;

            if (stableSince !== undefined && now - stableSince >= stablePaintMs
                && (engineReady || waited >= 8000)) {
                video.style.visibility = '';
                if (frameCallbacks) videoId = video.requestVideoFrameCallback(onVideoFrame);
                else paintId = requestAnimationFrame(onLegacyFrame);
                var attempt = ++playAttempt;
                video.play().catch(function (error) {
                    if (attempt === playAttempt && error.name !== 'AbortError') showFallback();
                });
            } else if (video.error || (!mediaReady && waited >= 15000)) {
                showFallback();
            } else {
                paintId = requestAnimationFrame(startWhenVisible);
            }
        }

        function waitForPaint() {
            cancelFrames();
            video.style.visibility = 'hidden';
            waitingSince = performance.now();
            stableSince = undefined;
            lastPaint = waitingSince;
            if (design.mediaVisible()) paintId = requestAnimationFrame(startWhenVisible);
        }

        var unwatchVisibility = design.watchMediaVisibility(function (visible, transitioned) {
            if (done || closing || showingFallback) return;
            if (!visible) {
                cancelFrames();
                video.pause();
                return;
            }
            if (!transitioned && (!video.paused || paintId !== undefined)) return;
            video.pause();
            if (video.currentTime !== presentedTime) video.currentTime = presentedTime;
            waitForPaint();
        });
        document.addEventListener('keydown', skipKey, true);
        overlay.addEventListener('click', skip, true);
        overlay.addEventListener('auxclick', skip, true);
        overlay.addEventListener('contextmenu', skip, true);
        overlay.addEventListener('animationend', transitionEnded);
        video.addEventListener('ended', finished);
        video.addEventListener('error', showFallback);
        finishAlreadyPlayed = function () { stop(true); };
        waitForPaint();
        return function () { stop(false); };
    }

    window.PTHubIntroReady = function () { engineReady = true; };
    window.PTHubIntroAlreadyPlayed = function () {
        introAlreadyPlayed = true;
        if (finishAlreadyPlayed) finishAlreadyPlayed();
    };
    window.panopticonIntro = { start: start, update: update };
}());

