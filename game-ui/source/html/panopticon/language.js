(function () {
    'use strict';

    const countries = {
        bg: ['BG', 'Bulgaria'], cs: ['CZ', 'Czechia'], da: ['DK', 'Denmark'],
        de: ['DE', 'Germany'], el: ['GR', 'Greece'], en: ['GB', 'United Kingdom'],
        'es-es': ['ES', 'Spain'], et: ['EE', 'Estonia'], fi: ['FI', 'Finland'],
        fr: ['FR', 'France'], he: ['IL', 'Israel'], hr: ['HR', 'Croatia'],
        hu: ['HU', 'Hungary'], it: ['IT', 'Italy'], ja: ['JP', 'Japan'],
        ko: ['KR', 'South Korea'], lt: ['LT', 'Lithuania'], nl: ['NL', 'Netherlands'],
        no: ['NO', 'Norway'], pl: ['PL', 'Poland'], 'pt-br': ['BR', 'Brazil'],
        'pt-pt': ['PT', 'Portugal'], ru: ['RU', 'Russia'], sk: ['SK', 'Slovakia'],
        'sv-se': ['SE', 'Sweden'], th: ['TH', 'Thailand'], tr: ['TR', 'Türkiye'],
        uk: ['UA', 'Ukraine'], vi: ['VN', 'Vietnam'], 'zh-cn': ['CN', 'China'],
        'zh-tw': ['TW', 'Taiwan']
    };
    const labels = {
        en: { language: 'Game language', pirate: 'Pirate English' },
        ru: { language: 'Язык игры', pirate: 'Пиратский английский' },
        fr: { language: 'Langue du jeu', pirate: 'Anglais pirate' }
    };
    const flagCodes = ['en', ...Object.keys(countries).filter(code => code !== 'en'), 'en-pt'];

    function normalize(value) {
        const code = String(value || '').trim().toLowerCase().replace(/_/g, '-');
        switch (code) {
            case 'english': return 'en';
            case 'russian': return 'ru';
            case 'french': return 'fr';
            default: return code;
        }
    }

    window.PTHubLanguagePicker = function ({ react: React, bridge, locale, gameLanguage }) {
        const [available, setAvailable] = React.useState([]);
        const [open, setOpen] = React.useState(false);
        const [hovered, setHovered] = React.useState(null);
        const [current, setCurrent] = React.useState(() => normalize(gameLanguage));
        const root = React.useRef(null);
        const toggle = React.useRef(null);
        const names = React.useMemo(() => typeof Intl.DisplayNames === 'function'
            ? new Intl.DisplayNames([locale], { type: 'region' }) : null, [locale]);
        const text = labels[locale] || labels.en;
        const selected = available.includes(current) ? current
            : available.includes(locale) ? locale : available[0];

        React.useEffect(() => {
            if (!bridge.isNative) return;
            const stopList = bridge.on('updateLanguages', files => {
                if (!Array.isArray(files)) return;
                const installed = new Set(files.map(file => String(file).toLowerCase()));
                setAvailable(flagCodes.filter(code => installed.has(code + '.png')));
            });
            const stopCurrent = bridge.on('updateLanguage', value => setCurrent(normalize(value)));
            bridge.runLua('UpdateLanguages(); stMenu.ApplyHubLanguage()');
            return () => { stopList(); stopCurrent(); };
        }, [bridge]);

        function close() {
            setOpen(false);
            setHovered(null);
        }

        React.useEffect(() => {
            if (!open) return;
            function outside(event) {
                if (root.current && !root.current.contains(event.target)) close();
            }
            function escape(event) {
                if (event.key !== 'Escape') return;
                event.preventDefault();
                event.stopImmediatePropagation();
                close();
                toggle.current?.focus();
            }
            function visibilityChanged() {
                if (document.hidden) close();
            }
            document.addEventListener('pointerdown', outside);
            document.addEventListener('visibilitychange', visibilityChanged);
            window.addEventListener('blur', close);
            window.addEventListener('keydown', escape, true);
            return () => {
                document.removeEventListener('pointerdown', outside);
                document.removeEventListener('visibilitychange', visibilityChanged);
                window.removeEventListener('blur', close);
                window.removeEventListener('keydown', escape, true);
            };
        }, [open]);

        if (!selected) return null;
        const element = React.createElement;

        function name(code) {
            if (code === 'en-pt') return text.pirate;
            const [region, fallback] = countries[code];
            return names?.of(region) || fallback;
        }

        function flag(code) {
            return element('img', {
                src: '../resource/localization/' + code + '.png',
                alt: '', draggable: false
            });
        }

        return element('div', {
            className: 'pt-language', ref: root,
            onBlur: event => {
                if (open && !event.currentTarget.contains(event.relatedTarget)) close();
            }
        },
            element('button', {
                className: 'glass-button pt-language__toggle', type: 'button', ref: toggle,
                'aria-expanded': open, 'aria-controls': open ? 'pt-language-panel' : undefined,
                'aria-label': text.language + ' · ' + name(selected),
                title: text.language + ' · ' + name(selected),
                onClick: () => open ? close() : setOpen(true)
            }, flag(selected)),
            open && element('div', { className: 'pt-language__panel', id: 'pt-language-panel' },
                element('div', { className: 'pt-language__label' }, hovered ? name(hovered) : text.language),
                element('div', { className: 'pt-language__grid', role: 'group', 'aria-label': text.language },
                    available.map(code => element('button', {
                        className: 'glass-button pt-language__choice',
                        type: 'button', key: code, 'aria-label': name(code),
                        'aria-pressed': code === selected,
                        onMouseEnter: () => setHovered(code),
                        onMouseLeave: () => setHovered(null),
                        onFocus: () => setHovered(code),
                        onBlur: () => setHovered(null),
                        onClick: () => {
                            if (code !== selected) bridge.runLua('stMenu.SetLanguage(%s)', code);
                            close();
                            toggle.current?.focus();
                        }
                    }, flag(code))))));
    };
})();
