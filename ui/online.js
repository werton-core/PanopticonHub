(function () {
  'use strict';

  var badgeSelector = '.topbar > .online-chip:not(.pt-online)';
  var latest = null;
  var visibleBadge;
  var nativeBadge;
  var timer;

  function makeMark(source, label) {
    var mark = document.createElement('img');
    mark.className = 'pt-online__mark';
    mark.src = source;
    mark.alt = label;
    mark.width = 20;
    mark.height = 20;
    mark.style.objectFit = 'contain';
    mark.style.borderRadius = '4px';
    mark.style.flexShrink = '0';
    mark.style.pointerEvents = 'none';
    mark.draggable = false;
    return mark;
  }

  function makeBadge(reference) {
    reference.setAttribute('role', 'group');
    reference.setAttribute('aria-label', 'Онлайн Shinri Trial');
    reference.appendChild(makeMark('/shinri-online.png', 'Shinri Trial'));

    var badge = document.createElement('div');
    badge.className = 'online-chip pt-online';
    badge.setAttribute('role', 'group');
    badge.setAttribute('aria-label', 'Онлайн Panopticon Trial');
    var text = document.createElement('span');
    text.className = 'oc-txt';
    text.append(document.createElement('b'), document.createElement('span'));
    var icon = reference.querySelector('.oc-ico');
    if (icon) badge.append(icon.cloneNode(true));
    badge.append(text, makeMark('/panopticon-online.png', 'Panopticon Trial'));
    reference.before(badge);
    return badge;
  }

  function render() {
    var reference = document.querySelector(badgeSelector);
    if (!reference) return;
    if (nativeBadge !== reference || !visibleBadge || !visibleBadge.isConnected) {
      if (visibleBadge) visibleBadge.remove();
      nativeBadge = reference;
      var oldMark = reference.querySelector('.pt-online__mark');
      if (oldMark) oldMark.remove();
      visibleBadge = makeBadge(reference);
    }

    var originalLabel = reference.querySelector('.oc-txt > span');
    var onlineLabel = originalLabel ? originalLabel.textContent : 'онлайн';
    var count = visibleBadge.querySelector('.oc-txt > b');
    var label = visibleBadge.querySelector('.oc-txt > span');
    var value = latest && latest.total !== null ? String(latest.total) : '—';
    var title = 'Онлайн Panopticon Trial недоступен.';
    if (latest && latest.total !== null) {
      title = 'Онлайн Panopticon Trial — ' + latest.total + '.';
      if (latest.available < latest.expected) {
        title += ' Ответили ' + latest.available + ' из ' + latest.expected + ' серверов.';
      }
    }
    if (count.textContent !== value) count.textContent = value;
    if (label.textContent !== onlineLabel) label.textContent = onlineLabel;
    if (visibleBadge.title !== title) visibleBadge.title = title;
  }

  function valid(payload) {
    var data = payload && payload.panopticon;
    if (!data || !Number.isSafeInteger(data.expected) || data.expected < 1 ||
        !Number.isSafeInteger(data.available) || data.available < 0 || data.available > data.expected) return null;
    if (data.available === 0 ? data.total !== null : !Number.isSafeInteger(data.total) || data.total < 0) return null;
    return data;
  }

  async function poll() {
    var controller = new AbortController();
    var timeout = setTimeout(function () { controller.abort(); }, 4000);
    try {
      var response = await fetch('/online', { signal: controller.signal, cache: 'no-store' });
      latest = response.ok ? valid(await response.json()) : null;
    } catch (_) {
      latest = null;
    } finally {
      clearTimeout(timeout);
      render();
      timer = setTimeout(poll, latest && latest.available ? 45000 : 3500);
    }
  }

  function start() {
    var root = document.getElementById('root');
    if (!root) return;
    new MutationObserver(render).observe(root, { childList: true, characterData: true, subtree: true });
    render();
    poll();
  }

  window.addEventListener('beforeunload', function () { clearTimeout(timer); });
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', start, { once: true });
  else start();
})();
