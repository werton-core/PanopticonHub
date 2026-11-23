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
  var regionControls = new WeakMap();
  var reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)');
  var dragProperties = ['app-region', '-webkit-app-region'];
  var oldDrag;

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
    if (event.target instanceof Element && event.target.closest('select, .st-dialog-dim')) return;
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

  function makeRegionControl(row, original, originalDescription) {
    var select = document.createElement('select');
    select.className = 'st-dd__btn';
    select.style.cssText = 'flex-shrink:0;width:min(288px, 52%);background:#141414';
    select.disabled = true;
    for (var choice of ['cis', 'eu']) select.add(new Option(choice, choice));

    var description = document.createElement('div');
    description.className = 'st-set-desc';
    description.id = 'pt-settings-region-help';
    select.setAttribute('aria-describedby', description.id);
    var error = document.createElement('div');
    error.className = 'st-set-desc';
    error.setAttribute('role', 'status');
    error.hidden = true;

    original.hidden = true;
    originalDescription.hidden = true;
    originalDescription.after(description, error);
    row.append(select);

    var control = {select: select, description: description, error: error, copy: null};
    var saved = 'cis';
    loadRegion().then(function (region) { saved = region; }).catch(function () {
      error.textContent = control.copy.loadError;
      error.hidden = false;
    }).finally(function () {
      select.value = saved;
      select.disabled = false;
    });
    select.addEventListener('change', async function () {
      var desired = select.value;
      if (desired === saved) return;
      select.disabled = true;
      error.hidden = true;
      try {
        saved = await loadRegion(desired);
      } catch (_) {
        error.textContent = control.copy.error;
        error.hidden = false;
      } finally {
        select.value = saved;
        select.disabled = false;
      }
    });
    return control;
  }

  function configureRegion(viewport) {
    for (var row of viewport.querySelectorAll('.st-set-row')) {
      var name = row.querySelector('.st-set-name');
      var label = name && name.textContent.trim();
      var copy = Object.prototype.hasOwnProperty.call(descriptions, label) && descriptions[label];
      if (!copy) continue;
      var original = row.querySelector('.st-dd');
      var originalDescription = row.querySelector('.st-set-desc');
      if (!original || !originalDescription) return;
      var labels = original.querySelectorAll('.st-dd__opt');
      if (labels.length !== 2) return;
      var control = regionControls.get(row);
      if (!control) {
        control = makeRegionControl(row, original, originalDescription);
        regionControls.set(row, control);
      }
      control.copy = copy;
      if (control.description.textContent !== copy.help) {
        control.description.textContent = copy.help;
        control.select.setAttribute('aria-label', label);
        for (var i = 0; i < labels.length; i++) {
          control.select.options[i].text = labels[i].textContent.trim();
        }
      }
      return;
    }
  }

  function refresh() {
    var viewport = document.querySelector(viewportSelector);
    syncDrag(viewport);
    if (!viewport) return;
    configureScroll(viewport);
    configureRegion(viewport);
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
