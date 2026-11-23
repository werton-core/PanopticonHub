(function () {
  'use strict';

  var service = '/game-region';
  var descriptions = {
    'Регион': {
      help: 'Выбирает вкладку по умолчанию при первом открытии списка серверов после запуска Garry’s Mod. В игре регион можно переключить.',
      loadError: 'Не удалось прочитать сохранённый регион.',
      error: 'Не удалось сохранить регион.',
    },
    'Region': {
      help: 'Sets the default server tab the first time you open the list after starting Garry’s Mod. You can switch regions in the game.',
      loadError: 'Could not read the saved region.',
      error: 'Could not save the region.',
    },
    'Région': {
      help: 'Définit l’onglet affiché à la première ouverture de la liste des serveurs après le lancement de Garry’s Mod. La région reste modifiable en jeu.',
      loadError: 'Impossible de lire la région enregistrée.',
      error: 'Impossible d’enregistrer la région.',
    },
  };
  var viewportSelector = '.modal.m-settings > .modal-body';
  var viewports = new WeakMap();
  var pendingRegionWrite;
  var reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)');
  var dragProperties = ['app-region', '-webkit-app-region'];
  var oldDrag;
  var identitySettings = new WeakMap();

  function configureIdentity(viewport) {
    if (identitySettings.has(viewport)) return;
    var section = document.createElement('section');
    section.className = 'st-block';
    var header = document.createElement('h3');
    header.className = 'st-block__head';
    header.textContent = 'Подключение';
    var body = document.createElement('div');
    body.className = 'st-block__body';
    var row = document.createElement('div');
    row.className = 'st-set-row';
    var labels = document.createElement('div');
    var name = document.createElement('div');
    name.className = 'st-set-name';
    name.id = 'pt-shinri-identity-name';
    name.textContent = 'Идентификатор Shinri Hub';
    var help = document.createElement('div');
    help.className = 'st-set-desc';
    help.id = 'pt-shinri-identity-help';
    help.textContent = 'Включает совместимость при выборе серверов Shinri Trial. Подключения Panopticon остаются обычными. Меняется до запуска игры.';
    labels.append(name, help);
    var toggle = document.createElement('label');
    toggle.className = 'st-toggle';
    var input = document.createElement('input');
    input.id = 'pt-shinri-identity';
    input.type = 'checkbox';
    input.disabled = true;
    input.setAttribute('role', 'switch');
    input.setAttribute('aria-labelledby', name.id);
    input.setAttribute('aria-describedby', help.id);
    var track = document.createElement('span'); track.className = 'st-toggle__track';
    var thumb = document.createElement('span'); thumb.className = 'st-toggle__thumb';
    toggle.append(input, track, thumb);
    row.append(labels, toggle);
    var error = document.createElement('p');
    error.className = 'st-set-desc'; error.setAttribute('role', 'status'); error.hidden = true;
    body.append(row, error); section.append(header, body); viewport.append(section);
    var saved = false;
    var writing = false;
    var active = false;
    function disabled() { input.disabled = writing || active; }
    function refreshActive() {
      if (!viewport.isConnected) return;
      window.electron.invoke('getGmodPid').then(function (pid) { active = !!pid; disabled(); }).catch(function () { input.disabled = true; });
    }
    var onPid = function (pid) { active = !!pid; disabled(); };
    var onLaunch = function (state) { if (state) { active = true; disabled(); } else refreshActive(); };
    window.electron.on('gmodUpdatePid', onPid);
    window.electron.on('gmodLaunchState', onLaunch);
    var cleanup = new MutationObserver(function () {
      if (viewport.isConnected) return;
      window.electron.removeListener('gmodUpdatePid', onPid);
      window.electron.removeListener('gmodLaunchState', onLaunch);
      cleanup.disconnect();
    });
    cleanup.observe(document.body, {childList: true, subtree: true});
    window.electron.invoke('store', 'get', 'shinriIdentity').then(function (value) {
      saved = value === true; input.checked = saved; refreshActive();
    }).catch(function () { error.textContent = 'Не удалось прочитать идентификатор Shinri Hub.'; error.hidden = false; });
    input.addEventListener('change', async function () {
      var desired = input.checked;
      writing = true; disabled(); error.hidden = true;
      try { await window.electron.invoke('store', 'set', 'shinriIdentity', desired); saved = desired; }
      catch (_) { error.textContent = 'Измените идентификатор после выхода из игры. Если игра закрыта, проверьте доступ к настройкам.'; error.hidden = false; }
      finally { input.checked = saved; writing = false; refreshActive(); }
    });
    identitySettings.set(viewport, {refresh: refreshActive});
  }

  function syncDrag(viewport) {
    var host = viewport && viewport.closest('.hub.drag-window');
    if (oldDrag && oldDrag.host === host) return;
    if (oldDrag) {
      for (var item of oldDrag.properties) {
        oldDrag.host.style.setProperty(item.name, item.value, item.priority);
      }
      oldDrag = null;
    }
    if (!host) return;
    oldDrag = {
      host: host,
      properties: dragProperties.map(function (name) {
        return {name: name, value: host.style.getPropertyValue(name), priority: host.style.getPropertyPriority(name)};
      }),
    };
    for (var name of dragProperties) host.style.setProperty(name, 'no-drag', 'important');
  }

  function getDelta(event, viewport) {
    if (event.deltaMode === WheelEvent.DOM_DELTA_LINE) return event.deltaY * 20;
    if (event.deltaMode === WheelEvent.DOM_DELTA_PAGE) return event.deltaY * viewport.clientHeight;
    return event.deltaY;
  }

  function configureScroll(viewport) {
    if (viewports.has(viewport)) return;
    viewport.tabIndex = 0;
    viewport.style.outline = 'none';
    viewport.parentElement.style.maxHeight = 'calc(100vh - 60px)';
    viewport.style.overflowY = 'auto';
    viewport.style.overscrollBehaviorY = 'contain';
    viewport.style.scrollBehavior = 'auto';
    for (var name of dragProperties) viewport.style.setProperty(name, 'no-drag');
    if (!viewport.contains(document.activeElement)) viewport.focus({preventScroll: true});

    var end = 0;
    var frame = 0;
    function reset() {
      if (frame) cancelAnimationFrame(frame);
      frame = 0;
    }
    viewport.addEventListener('pointerdown', reset);
    viewport.addEventListener('keydown', reset);
    viewports.set(viewport, function (event) {
      var limit = viewport.scrollHeight - viewport.clientHeight;
      if (event.ctrlKey || event.shiftKey || !event.deltaY || limit <= 0) return;
      event.preventDefault();
      var from = viewport.scrollTop;
      end = Math.max(0, Math.min(limit, (frame ? end : from) + getDelta(event, viewport)));
      reset();
      if (reducedMotion.matches || end === from) {
        viewport.scrollTop = end;
        return;
      }
      var start = performance.now();
      function step(now) {
        if (!viewport.isConnected) {
          frame = 0;
          return;
        }
        var time = Math.max(0, Math.min(1, (now - start) / 180));
        viewport.scrollTop = from + (end - from) * (1 - Math.pow(1 - time, 3));
        frame = time < 1 ? requestAnimationFrame(step) : 0;
      }
      frame = requestAnimationFrame(step);
    });
  }

  function onWheel(event) {
    var viewport = document.querySelector(viewportSelector);
    if (!viewport) return;
    var rect = viewport.getBoundingClientRect();
    if (event.clientX < rect.left || event.clientX >= rect.right ||
        event.clientY < rect.top || event.clientY >= rect.bottom) return;
    if (event.target instanceof Element && event.target.closest('.st-dialog-dim')) return;
    var scroll = viewports.get(viewport);
    if (scroll) scroll(event);
  }

  async function loadRegion(region) {
    var controller = new AbortController();
    var timer = setTimeout(function () { controller.abort(); }, 4000);
    try {
      var response = await fetch(region ? service + '/' + region : service, {
        method: region ? 'POST' : 'GET',
        cache: 'no-store',
        signal: controller.signal,
      });
      if (!response.ok) throw new Error('Region preference is unavailable.');
      var data = await response.json();
      if (data.region !== 'cis' && data.region !== 'eu') throw new Error('Invalid region preference.');
      return data.region;
    } finally {
      clearTimeout(timer);
    }
  }

  window.PTHubGameRegion = {
    descriptions: descriptions,
    async read() {
      try { await pendingRegionWrite; } catch (_) { /* Read the saved value after a failed write. */ }
      return loadRegion();
    },
    async write(value) {
      var write = loadRegion(value);
      pendingRegionWrite = write;
      try { return await write; }
      finally { if (pendingRegionWrite === write) pendingRegionWrite = null; }
    },
  };

  function refresh() {
    var viewport = document.querySelector(viewportSelector);
    syncDrag(viewport);
    if (!viewport) return;
    configureScroll(viewport);
    configureIdentity(viewport);
  }

  function start() {
    var root = document.getElementById('root');
    if (!root) return;
    document.addEventListener('wheel', onWheel, {capture: true, passive: false});
    new MutationObserver(refresh).observe(root, {childList: true, characterData: true, subtree: true});
    refresh();
  }

  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', start, {once: true});
  else start();
})();
