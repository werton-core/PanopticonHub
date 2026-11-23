(function () {
    'use strict';
    var logo = document.querySelector('.enter-screen__logo');
    if (logo) {
        logo.style.left = '50%';
        logo.style.maxWidth = '90%';
        logo.style.transform = 'translateX(-50%)';
    }
    var russianStatuses = {
        'connecting to server': 'Подключение к серверу…',
        'retrieving server info': 'Получение данных сервера…',
        'sending client info': 'Отправка данных клиента…',
        'waiting for steam workshop to finish': 'Ожидание Steam Workshop…',
        'mounting addons': 'Подключение аддонов…',
        'loading server cache content': 'Загрузка кэша сервера…',
        'loading the map': 'Загрузка карты…',
        'starting lua': 'Запуск Lua…'
    };
    var lang = 'ru';
    var tipTitle = '', tipText = '';
    var status = '', hasNativeStatus = false;
    var framePending = false;

    function paint() {
        framePending = false;
        var russian = lang === 'ru';
        var next = { lang: lang, tipTitle: tipTitle, tipText: tipText };
        var queue = status.match(/^(\d+)\/(\d+)(?:\s*\(([^)]+)\))?\s+-\s+(.+?)\s+is downloading[.!]?$/i);
        var workshop = status.match(/\bDownloading\s+'(.+)'\s+via Workshop[.!]?$/i);
        var download = status.match(/^Downloading\s+(.+)$/i);

        if (queue) {
            next.label = russian ? 'Загрузка файла' : 'Downloading file';
            next.detail = queue[4].replace(/\\/g, '/');
            var current = Number(queue[1]);
            var total = Number(queue[2]);
            if (Number.isSafeInteger(total) && current >= 1 && current <= total) {
                next.current = current;
                next.total = total;
                // The engine announces the current item at the start of its transfer.
                next.percent = 100 * (current - 1) / total;
            }
            var filePercent = (queue[3] || '').match(/^(\d+(?:[.,]\d+)?)\s*%$/);
            if (filePercent && Number(filePercent[1].replace(',', '.')) <= 100) {
                next.detail += ' · ' + queue[3];
            }
        } else if (workshop) {
            next.label = russian ? 'Загрузка из Steam Workshop…' : 'Downloading from Steam Workshop…';
            next.detail = workshop[1];
        } else if (download) {
            next.label = russian ? 'Загрузка файла…' : 'Downloading file…';
            next.detail = download[1];
        } else {
            var key = status.replace(/[.!…]+$/, '').toLowerCase();
            var localized = russian && Object.prototype.hasOwnProperty.call(russianStatuses, key) && russianStatuses[key];
            next.label = localized || status || (russian ? 'Подключение к серверу…' : 'Connecting to server…');
        }

        window.STLoadingProgress(next);
    }

    function publish() {
        if (framePending) return;
        framePending = true;
        window.requestAnimationFrame(paint);
    }
    function setStatus(value) {
        status = String(value || '').trim();
        publish();
    }

    window.GameDetails = function (name, url, map, maxPlayers, steamId, mode, volume, language, title) {
        tipTitle = name || (window.PTHubGameDesign && window.PTHubGameDesign.gameName) || 'Panopticon Hub';
        tipText = [title || mode || "Garry's Mod", map].filter(Boolean).join(' · ');
        if (language) lang = language;
        publish();
    };

    window.STLoadingStatus = function (value) {
        hasNativeStatus = true;
        setStatus(value);
    };

    // Older menu builds can still supply status text and the current file name.
    window.DownloadingFile = function (file) {
        if (!hasNativeStatus && file) setStatus('Downloading ' + file);
    };
    window.SetStatusChanged = function (value) {
        if (!hasNativeStatus) setStatus(value);
    };

    paint();
})();
