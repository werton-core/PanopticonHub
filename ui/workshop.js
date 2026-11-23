(function () {
  'use strict';

  const copy = {
    ru: {
      filter: 'Ограничить аддоны Workshop',
      filterHelp: 'При запуске разрешены подписки из обеих коллекций, Shinri Trial и Panopticon Trial.',
      panopticon: 'Проверять и скачивать аддоны Panopticon Trial',
      panopticonHelp: 'Подготовить коллекцию при первом запуске. На список разрешённых подписок эта настройка не влияет.',
      settingsError: 'Не удалось сохранить настройки Workshop.',
      settingsLoadError: 'Не удалось загрузить настройки Workshop.',
      downloadTitle: 'Скачать аддоны Panopticon Trial?',
      completeTitle: 'Докачать аддоны Panopticon Trial?',
      downloadHelp: 'Hub оформит недостающие подписки в Steam и скачает файлы при запуске игры.',
      choiceHelp: 'Эту настройку можно изменить позже в Hub.',
      install: 'Подписаться и скачать',
      onlyShinri: 'Только Shinri Trial',
      subscribing: 'Оформляем подписки…',
      saving: 'Сохраняем выбор…',
      subscribeError: 'Steam не подтвердил подписки. Повторите попытку или выберите только Shinri Trial.',
      unknownTitle: 'Аддон Workshop',
      unknownAuthor: 'Автор не указан',
      unknownSize: 'Размер неизвестен',
      more: 'Остальные аддоны',
      units: ['Б', 'КиБ', 'МиБ', 'ГиБ', 'ТиБ'],
    },
    en: {
      filter: 'Limit Workshop addons',
      filterHelp: 'Steam subscriptions from both Shinri Trial and Panopticon Trial can load at startup.',
      panopticon: 'Check and download Panopticon Trial addons',
      panopticonHelp: 'Prepare the collection during first-time setup. The allowed subscriptions are unaffected.',
      settingsError: 'Could not save Workshop settings.',
      settingsLoadError: 'Could not load Workshop settings.',
      downloadTitle: 'Download Panopticon Trial addons?',
      completeTitle: 'Download the remaining Panopticon Trial addons?',
      downloadHelp: 'Hub will add any missing Steam subscriptions and download the files when you start the game.',
      choiceHelp: 'You can change this preference later in Hub settings.',
      install: 'Subscribe and download',
      onlyShinri: 'Shinri Trial only',
      subscribing: 'Subscribing…',
      saving: 'Saving your choice…',
      subscribeError: 'Steam did not confirm the subscriptions. Try again or choose Shinri Trial only.',
      unknownTitle: 'Workshop addon',
      unknownAuthor: 'Author unavailable',
      unknownSize: 'Size unavailable',
      more: 'Remaining addons',
      units: ['B', 'KiB', 'MiB', 'GiB', 'TiB'],
    },
    fr: {
      filter: 'Limiter les addons du Workshop',
      filterHelp: 'Les abonnements aux collections Shinri Trial et Panopticon Trial sont autorisés au lancement.',
      panopticon: 'Vérifier et télécharger les addons Panopticon Trial',
      panopticonHelp: 'Prépare la collection au premier lancement. Les abonnements autorisés restent les mêmes.',
      settingsError: 'Impossible d’enregistrer les paramètres du Workshop.',
      settingsLoadError: 'Impossible de charger les paramètres du Workshop.',
      downloadTitle: 'Télécharger les addons Panopticon Trial ?',
      completeTitle: 'Télécharger les addons Panopticon Trial manquants ?',
      downloadHelp: 'Le Hub ajoutera les abonnements Steam manquants et téléchargera les fichiers au lancement du jeu.',
      choiceHelp: 'Vous pourrez modifier ce choix dans les paramètres du Hub.',
      install: 'S’abonner et télécharger',
      onlyShinri: 'Shinri Trial uniquement',
      subscribing: 'Abonnement en cours…',
      saving: 'Enregistrement du choix…',
      subscribeError: 'Steam n’a pas confirmé les abonnements. Réessayez ou choisissez Shinri Trial uniquement.',
      unknownTitle: 'Addon du Workshop',
      unknownAuthor: 'Auteur non indiqué',
      unknownSize: 'Taille inconnue',
      more: 'Addons restants',
      units: ['o', 'Kio', 'Mio', 'Gio', 'Tio'],
    },
  };

  const settingsSelector = '.modal.m-settings > .modal-body';
  const keys = ['workshopFilter', 'workshopPanopticon'];
  const settingsByViewport = new WeakMap();
  let onboarding;
  let activeSettings;

  function language() {
    const lang = document.documentElement.lang.toLowerCase().split('-')[0];
    return Object.prototype.hasOwnProperty.call(copy, lang) ? lang : 'ru';
  }

  function create(tag, className, content) {
    const element = document.createElement(tag);
    if (className) element.className = className;
    if (content !== undefined) element.textContent = content;
    return element;
  }

  function store(action, ...args) {
    return window.electron.invoke('store', action, ...args);
  }

  function buildSwitch(key) {
    const row = create('div', 'st-set-row');
    const labels = create('div');
    const title = create('div', 'st-set-name');
    const help = create('div', 'st-set-desc');
    title.id = key + '-name';
    help.id = key + '-help';
    labels.append(title, help);

    const toggle = create('label', 'st-toggle');
    const input = create('input');
    input.type = 'checkbox';
    input.disabled = true;
    input.setAttribute('role', 'switch');
    input.setAttribute('aria-labelledby', title.id);
    input.setAttribute('aria-describedby', help.id);
    toggle.append(input, create('span', 'st-toggle__track'), create('span', 'st-toggle__thumb'));
    row.append(labels, toggle);
    return { row, title, help, input };
  }

  function buildSettings(viewport) {
    const section = create('section', 'st-block pt-workshop-settings');
    const header = create('h3', 'st-block__head', 'Steam Workshop');
    const body = create('div', 'st-block__body');
    const status = create('p', 'st-set-desc pt-workshop-error');
    status.setAttribute('role', 'status');
    status.hidden = true;
    const controls = new Map(keys.map(key => [key, buildSwitch(key)]));
    for (const control of controls.values()) body.append(control.row);
    section.append(header, body, status);
    viewport.append(section);

    const settings = { section, controls, lang: null };
    for (const [key, control] of controls) {
      control.input.addEventListener('change', async () => {
        const value = control.input.checked;
        status.hidden = true;
        control.input.disabled = true;
        try {
          await store('set', key, value);
        } catch (_) {
          control.input.checked = !value;
          status.textContent = copy[language()].settingsError;
          status.hidden = false;
        } finally {
          control.input.disabled = false;
        }
      });
    }

    Promise.all(keys.map(key => store('get', key))).then(values => {
      keys.forEach((key, index) => {
        const input = controls.get(key).input;
        input.checked = values[index] !== false;
        input.disabled = false;
      });
    }).catch(() => {
      status.textContent = copy[language()].settingsLoadError;
      status.hidden = false;
    });
    return settings;
  }

  function refreshSettings() {
    const viewport = document.querySelector(settingsSelector);
    if (!viewport) {
      activeSettings = undefined;
      return;
    }
    let settings = settingsByViewport.get(viewport);
    if (!settings || !viewport.contains(settings.section)) {
      settings = buildSettings(viewport);
      settingsByViewport.set(viewport, settings);
    }
    activeSettings = settings;
    const lang = language();
    if (settings.lang === lang) return;
    settings.lang = lang;
    const labels = copy[lang];
    const filter = settings.controls.get('workshopFilter');
    const panopticon = settings.controls.get('workshopPanopticon');
    filter.title.textContent = labels.filter;
    filter.help.textContent = labels.filterHelp;
    panopticon.title.textContent = labels.panopticon;
    panopticon.help.textContent = labels.panopticonHelp;
  }

  function sizeLabel(bytes, lang, labels) {
    const size = Number(bytes);
    if (!Number.isSafeInteger(size) || size <= 0) return labels.unknownSize;
    let value = size;
    let unit = 0;
    while (value >= 1024 && unit < labels.units.length - 1) {
      value /= 1024;
      unit++;
    }
    const formatted = new Intl.NumberFormat(lang, { maximumFractionDigits: unit > 0 ? 1 : 0 }).format(value);
    return formatted + '\u00a0' + labels.units[unit];
  }

  function addonRow(item, lang, labels) {
    const row = create('li', 'pt-workshop-addon');
    const thumbnail = create('div', 'pt-workshop-addon__thumbnail');
    if (typeof item.preview === 'string' && /^\/workshop\/[0-9]+\.webp$/.test(item.preview)) {
      const image = create('img');
      image.alt = '';
      image.referrerPolicy = 'no-referrer';
      image.addEventListener('error', () => image.remove(), { once: true });
      image.src = item.preview;
      thumbnail.append(image);
    }
    const names = create('div', 'pt-workshop-addon__names');
    const title = create('div', 'pt-workshop-addon__title', item.title || labels.unknownTitle);
    title.title = title.textContent;
    const author = create('div', 'pt-workshop-addon__author', item.author || labels.unknownAuthor);
    author.title = author.textContent;
    names.append(title, author);
    row.append(thumbnail, names, create('div', 'pt-workshop-addon__size', sizeLabel(item.bytes, lang, labels)));
    return row;
  }

  function onboardingDialog(preview) {
    const lang = language();
    const labels = copy[lang];
    const modal = create('dialog', 'modal pt-workshop-dialog no-drag-window');
    const heading = create('h2', 'gm-title', preview.hasAny ? labels.completeTitle : labels.downloadTitle);
    const description = create('p', 'gm-txt', labels.downloadHelp);
    heading.id = 'pt-workshop-title';
    description.id = 'pt-workshop-description';
    modal.setAttribute('aria-labelledby', heading.id);
    modal.setAttribute('aria-describedby', description.id);

    const list = create('ul', 'pt-workshop-addons');
    for (const item of preview.missing.slice(0, 4)) list.append(addonRow(item, lang, labels));
    modal.append(heading, description, list);
    if (preview.missing.length > 4) {
      const remaining = preview.missing.length - 4;
      const extra = create('div', 'pt-workshop-extra', '+' + remaining);
      extra.title = labels.more;
      extra.setAttribute('aria-label', labels.more + ' ' + remaining);
      modal.append(extra);
    }

    const help = create('p', 'st-set-desc pt-workshop-choice-help', labels.choiceHelp);
    const status = create('p', 'pt-workshop-status');
    status.setAttribute('role', 'status');
    status.hidden = true;

    const actions = create('div', 'gm-actions pt-workshop-actions');
    const decline = create('button', 'a-btn ghost', labels.onlyShinri);
    const accept = create('button', 'a-btn', labels.install);
    decline.type = accept.type = 'button';
    decline.autofocus = true;
    actions.append(decline, accept);
    modal.append(help, status, actions);

    return new Promise(resolve => {
      let subscribed = false;
      let busy = false;
      let result = false;

      modal.addEventListener('close', () => {
        modal.remove();
        resolve(result);
      }, { once: true });
      modal.addEventListener('cancel', event => {
        if (busy) event.preventDefault();
      });

      async function choose(accepted) {
        if (busy) return;
        busy = true;
        decline.disabled = accept.disabled = true;
        status.classList.remove('pt-workshop-error');
        status.textContent = accepted && !subscribed ? labels.subscribing : labels.saving;
        status.hidden = false;
        try {
          if (accepted && !subscribed) {
            await window.electron.invoke('workshopSubscribe');
            subscribed = true;
            status.textContent = labels.saving;
          }
          if (!accepted) {
            await store('set', 'workshopPanopticon', false);
            const control = activeSettings?.controls.get('workshopPanopticon');
            if (control) control.input.checked = false;
          }
          await store('set', 'workshopOnboarding', accepted ? 'prepare' : 'done');
          result = accepted;
          modal.close();
        } catch (_) {
          busy = false;
          decline.disabled = accept.disabled = false;
          status.classList.add('pt-workshop-error');
          status.textContent = accepted && !subscribed ? labels.subscribeError : labels.settingsError;
        }
      }

      decline.addEventListener('click', () => choose(false));
      accept.addEventListener('click', () => choose(true));
      document.body.append(modal);
      modal.showModal();
    });
  }

  async function showFirstRun() {
    try {
      const [state, enabled, consent] = await Promise.all([
        store('get', 'workshopOnboarding'),
        store('get', 'workshopPanopticon'),
        store('get', 'legalConsent'),
      ]);
      if (state !== 'pending' || enabled === false || !consent?.acceptedAt) return undefined;
      const preview = await window.electron.invoke('workshopPreview');
      if (!Array.isArray(preview?.missing)) return false;
      if (preview.missing.length !== 0) return await onboardingDialog(preview);
      await store('set', 'workshopOnboarding', 'done');
      return true;
    } catch (_) {
      // An unavailable Steam session can be checked again the next time Hub opens.
      return false;
    }
  }

  window.panopticonWorkshopReady = async function (retry = false) {
    if (retry && onboarding && await onboarding === false) onboarding = undefined;
    if (!onboarding) {
      onboarding = showFirstRun().then(result => {
        if (result === undefined) onboarding = undefined;
        return result;
      });
    }
    return onboarding;
  };

  function start() {
    const root = document.getElementById('root');
    if (!root) return;
    const updates = new MutationObserver(refreshSettings);
    updates.observe(root, { childList: true, subtree: true });
    updates.observe(document.documentElement, { attributes: true, attributeFilter: ['lang'] });
    refreshSettings();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', start, { once: true });
  } else {
    start();
  }
})();
