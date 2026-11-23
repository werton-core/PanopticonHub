(function () {
    'use strict';

    var directory = {
        project: 'panopticon', region: 'cis', category: 'lobby',
        categories: ['lobby', 'game', 'favorite'], status: 'ready'
    };
    window.PTHubProjectState = function (next) {
        directory = next;
        window.dispatchEvent(new Event('pt-directory-change'));
    };

    window.PTHubUseDirectory = function (React) {
        var state = React.useState(directory);
        React.useEffect(function () {
            function update() { state[1](directory); }
            window.addEventListener('pt-directory-change', update);
            update();
            return function () { window.removeEventListener('pt-directory-change', update); };
        }, []);
        return state[0];
    };

    function select(project, region, category) {
        var categoryMap = { fast_game: 'game', game: 'story' };
        if (project === 'shinri') category = categoryMap[category] || category;
        if (window.lua) window.lua.Run('stMenu.SelectDirectory(%s, %s, %s)', project, region, category);
    }

    window.PTHubSelectCategory = function (category) {
        select(directory.project, directory.region, category);
    };

    window.PTHubServerFilters = function (props) {
        var React = props.react;
        var state = props.directory;
        function group(label, field, options) {
            return React.createElement('div', { className: 'pt-directory__group', role: 'group', 'aria-label': label },
                options.map(function ([value, text]) {
                    return React.createElement('button', {
                        key: value, type: 'button', 'aria-pressed': state[field] === value,
                        onClick: function () {
                            select(field === 'project' ? value : state.project,
                                field === 'region' ? value : state.region, state.category);
                        }
                    }, text);
                }));
        }
        return React.createElement('div', { className: 'pt-directory' },
            group('Проект', 'project', [['panopticon', 'Panopticon Trial'], ['shinri', 'Shinri Trial']]),
            group('Регион', 'region', [['cis', 'СНГ'], ['global', 'Global']]));
    };

    function attach(remaining) {
        var bridge = window.lua;
        if (!bridge || typeof bridge.Run !== 'function') {
            if (remaining > 0) setTimeout(function () { attach(remaining - 1); }, 50);
            return;
        }
        var run = bridge.Run;
        bridge.Run = function (command, ...args) {
            if (command === 'gui.OpenURL(%s)') return run.call(bridge, 'stMenu.OpenURL(%s)', ...args);
            if (command === 'RunGameUICommand(%s)' && String(args[0]).toLowerCase() === 'quit') {
                return run.call(bridge, 'stMenu.Quit()');
            }
            return run.call(bridge, command, ...args);
        };
        run.call(bridge, 'include("menu/panopticon_public.lua")');
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', function () { run.call(bridge, 'stMenu.Ready()'); }, { once: true });
        } else run.call(bridge, 'stMenu.Ready()');
    }
    attach(100);

    function keepVoiceSliderKeys(event) {
        if (event.target && event.target.id === 'pt-voice-monitor'
            && /^(Arrow(?:Up|Down|Left|Right)|Home|End|PageUp|PageDown)$/.test(event.key)) {
            event.stopImmediatePropagation();
        }
    }
    window.addEventListener('keydown', keepVoiceSliderKeys, true);
    window.addEventListener('keyup', keepVoiceSliderKeys, true);

    var settingsListSelector = '.settings-page__list';
    var preparation;
    var preparationPanel;
    var connectionInGame = false, connectionLoading = false;
    function identityLock() { window.PTHubIdentityLocked = connectionInGame || connectionLoading || !!preparation; }
    window.PTHubConnectionPreparation = function (value) {
        preparation = value;
        identityLock();
        if (!preparationPanel && value) {
            preparationPanel = document.createElement('section');
            preparationPanel.id = 'pt-connection-preparation';
            preparationPanel.setAttribute('role', 'status');
            preparationPanel.style.cssText = 'position:fixed;z-index:10000;bottom:48px;left:50%;transform:translateX(-50%);padding:18px 24px;background:#141414;color:#fff;border:1px solid #6c6c6c;font:16px sans-serif;display:flex;align-items:center;gap:24px';
            var label = document.createElement('span');
            label.textContent = 'Подготовка подключения Shinri…';
            var cancel = document.createElement('button');
            cancel.type = 'button';
            cancel.textContent = 'Отменить (Esc)';
            cancel.style.cssText = 'color:inherit;background:transparent;border:1px solid #888;padding:8px 14px;cursor:pointer';
            cancel.addEventListener('click', function () { window.lua.Run('stMenu.CancelConnection()'); });
            preparationPanel.append(label, cancel);
            document.body.append(preparationPanel);
        }
        if (preparationPanel) preparationPanel.style.display = value ? 'flex' : 'none';
    };
    window.addEventListener('keydown', function (event) {
        if (preparation && event.code === 'Escape') {
            event.preventDefault(); event.stopImmediatePropagation();
            window.lua.Run('stMenu.CancelConnection()');
        }
    }, true);
    var unavailableSettings = '[data-setting-id="discord-enabled"], [data-setting-id="admin-monowallhack"]';

    function hideUnavailableSettings(list) {
        list.querySelectorAll(unavailableSettings).forEach(function (row) {
            row.closest('.settings-section').hidden = true;
        });
    }

    window.addEventListener('keydown', function (event) {
        if (event.code !== 'ArrowDown' && event.code !== 'KeyS') return;
        // Closed renderer navigation includes the unavailable final two sections.
        var list = document.querySelector(settingsListSelector);
        if (!list || !list.querySelector('.settings-section[hidden]')
            || !list.querySelector('[data-setting-id="ui-streamer-mode"].setting-row--selected')) return;
        event.preventDefault();
        event.stopImmediatePropagation();
    }, true);

    var voiceTestScript = document.createElement('script');
    voiceTestScript.src = './panopticon/voice-test.js';
    document.head.appendChild(voiceTestScript);

    window.addEventListener('DOMContentLoaded', function () {
        var root = document.getElementById('root');
        if (root) {
            new MutationObserver(function (records) {
                for (var record of records) {
                    // Settings children are mounted together; hover/slider mutations are ignored.
                    if (record.target instanceof Element && record.target.matches(settingsListSelector)) {
                        hideUnavailableSettings(record.target);
                        continue;
                    }
                    for (var node of record.addedNodes) {
                        if (!(node instanceof Element)) continue;
                        if (node.matches(settingsListSelector)) hideUnavailableSettings(node);
                        else if (node.matches('.settings-page, .settings-page__content')) {
                            var list = node.querySelector(settingsListSelector);
                            if (list) hideUnavailableSettings(list);
                        }
                    }
                }
            }).observe(root, { childList: true, subtree: true });
        }
        var setLoading = window.SetLoading;
        window.SetLoading = function (loading) {
            connectionLoading = !!loading; identityLock();
            document.body.classList.toggle('pt-loading', !!loading);
            if (setLoading) setLoading(loading);
            if (loading && window.STOpenMenuPage) window.STOpenMenuPage('home');
        };
        var setInGame = window.SetInGame;
        window.SetInGame = function (inGame) {
            connectionInGame = !!inGame; identityLock();
            if (setInGame) setInGame(inGame);
        };
    });
})();
