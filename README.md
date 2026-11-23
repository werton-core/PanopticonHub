# Panopticon Hub

[English](#english) · [Русский](#русский) · [Download for Windows / Скачать для Windows](https://github.com/werton-core/PanopticonHub/releases/latest) · [Changelog](https://github.com/werton-core/PanopticonHub/releases)

## English

Panopticon Hub is Werton’s unofficial Windows launcher for the Steam edition of Garry’s Mod. It provides a server browser for Panopticon Trial and Shinri Trial, Steam Workshop setup, and a customizable launcher, game menu, and connection screen.

Werton develops and releases Panopticon Hub independently of the Shinri Trial team.

### Install and play

1. Install Garry’s Mod through Steam on 64-bit Windows. In **Library → Garry’s Mod → Properties → Betas**, select **x86-64** and wait for Steam to finish updating. The launcher window uses the Microsoft Edge WebView2 Runtime.
2. Download **PanopticonHubSetup.exe** from the [latest GitHub release](https://github.com/werton-core/PanopticonHub/releases/latest) and run the installer.
3. With Steam open and signed in, start **Panopticon Hub**. Follow the first-run prompts and choose whether to prepare the optional Panopticon Trial Workshop collection. Use **Play** to start the game.
4. Open the server browser in the game menu to choose a community, region, and server.

Hub launches the installed 64-bit game through Steam. The custom menu applies for that session, and Hub restores the original menu files when the game closes.

### Updates

Hub checks GitHub Releases and asks before installing by default. In **Settings → Updates**, choose **Ask**, **Install automatically**, or **Check manually**. **Check for updates** works in every mode. Hub verifies the downloaded installer against the SHA-256 digest published by GitHub before running it. Installation waits until Garry’s Mod closes. An all-users installation may require Windows administrator approval.

Version 0.2.36 is the first to include the updater. If you use an earlier version, run the installer once to enable future update checks. For a manual update, close Garry’s Mod and Hub, then run the latest installer over your existing installation. Your settings and appearance profile carry over. [Browse releases](https://github.com/werton-core/PanopticonHub/releases).

### Customize

Select the brush beside Settings to open **Appearance**. Change the accent, window names and icons, logos, backgrounds, layout, decorations, and music. Use **Apply** to save. Game appearance takes effect the next time you launch through Hub. You can exchange appearance presets with their media as `.pthubtheme` files.

### Network

Hub polls Panopticon servers for player counts, reads public Shinri Trial server and player-count data, and checks GitHub Releases according to the selected update mode. Steam handles game and Workshop downloads. Garry’s Mod connects directly to the selected server. Website and social links open when selected.

### Build

Use PowerShell on x64 Windows with Rust’s `x86_64-pc-windows-msvc` toolchain, the MSVC build tools, and the Windows SDK. The repository includes the editable Rust, Lua and customization code, plus the inherited interface bundles used at build time in `ui/dist` and `game-ui/dist`.

```powershell
.\tools\build-host.ps1
.\tools\build-release.ps1
```

The first command builds `target/release/PanopticonHub.exe`. The second builds the host, game module, uninstaller, and installer, then writes `dist/PanopticonHubSetup.exe` and prints its SHA-256.

For a new release, bump the setup version in `installer/Cargo.toml`, update `Cargo.lock`, and push a matching version tag such as `v0.2.36`, then run `.\tools\publish-release.ps1 -NotesFile <release-notes.md>` with an authenticated GitHub CLI. The script verifies the installer uploaded to the draft before publishing it. To release only a setup change, keep the host version and pass its existing matching EXE to `installer/build-setup.ps1` with `-HostExe`. Setup leaves installed binaries in place when their SHA-256 matches the payload.

### Credits

- Parts of the launcher and game menu build on interface code and media from the Shinri Trial team’s official launcher. These materials are included with permission. [Official Shinri Hub documentation](https://blog.shinri-trial.ru/docs/tutorial-start/shinri-hub/).
- The academy backdrops come from screenshots published by the Shinri Trial team. [Shinri Trial 1.6 update notes](https://blog.shinri-trial.ru/updates/update-1-6/).
- The official Shinri Trial site identifies ООО «ШИНРИ ДЕВЕЛОПМЕНТ» as its operator. [Shinri Trial website](https://shinri-trial.ru/).

The original creators and rights holders retain ownership of their code, media, and marks. The official Shinri Trial launcher is Shinri Hub.

[Third-party credits and license notices](THIRD_PARTY.md)

## Русский

Panopticon Hub — неофициальный лаунчер Werton для Steam-версии Garry’s Mod на Windows. В нём можно открыть каталог серверов Panopticon Trial и Shinri Trial, настроить Steam Workshop и оформить лаунчер, главное меню игры и экран подключения.

Werton разрабатывает и выпускает Panopticon Hub независимо от команды Shinri Trial.

### Установка и запуск

1. Установите Garry’s Mod через Steam на 64-битной Windows. В библиотеке откройте **Garry’s Mod → Свойства → Бета-версии**, выберите **x86-64** и дождитесь завершения загрузки. Для окна лаунчера требуется Microsoft Edge WebView2 Runtime.
2. Скачайте **PanopticonHubSetup.exe** из [последнего релиза на GitHub](https://github.com/werton-core/PanopticonHub/releases/latest) и запустите установщик.
3. Войдите в Steam и откройте **Panopticon Hub**. При первом запуске можно выбрать тему и решить, скачивать ли коллекцию Panopticon Trial из Steam Workshop. Кнопка **Играть** запускает Garry’s Mod.
4. В игровом меню откройте каталог, выберите проект, регион и сервер. Часто используемые серверы можно добавить в избранное.

Hub запускает установленный 64-битный Garry’s Mod через Steam. Меню Hub действует в течение сеанса. После выхода Hub восстанавливает исходные файлы меню. Выбор языка доступен в главном меню и по Esc. В настройках Workshop можно управлять подготовкой коллекции Panopticon Trial и ограничить подключаемые при запуске подписки коллекциями двух проектов.

**Настройки → Подключение → Идентификатор Shinri Hub** включает отдельный режим совместимости для серверов Shinri Trial. По умолчанию он включён; его можно выключить вручную. Та же настройка доступна в игровом меню: **Интерфейс → Общее**, до подключения. Panopticon продолжает использовать обычный протокол 24 при любом положении переключателя.

Режим проверяет сервер по публичному реестру Shinri, запрашивает A2S_INFO, подготавливает объявленный Lua-пак и передаёт протокол `-1` штатным транспортом GMod. При ошибке A2S выполняются ещё две попытки с паузами 0,5 и 1 секунду; отмена действует и во время ожидания. Запускается исходный Steam gmod.exe; файлы engine.dll на диске и Steam-аутентификация сохраняются. Совместимость зависит от версии движка; при неизвестных сигнатурах Hub показывает ошибку. Проверен вход с загрузкой карты и клиентского Lua до экрана регистрации Shinri. Завершение регистрации и дальнейшие серверные проверки не проверялись.

### Обновления

По умолчанию Hub проверяет GitHub Releases при запуске и предлагает установить новую версию. Откройте **Настройки → Обновления**, чтобы выбрать режим.

- **Спрашивать**. При наличии новой версии Hub предлагает установить её или отложить.
- **Устанавливать автоматически**. Hub скачивает и запускает установку новой версии.
- **Проверять вручную**. Проверка запускается по кнопке **Проверить обновления**. Эта кнопка доступна в любом режиме.

Перед запуском установщика Hub сверяет SHA-256 файла с хешем, опубликованным GitHub. Установка откладывается до выхода из Garry’s Mod. Windows может запросить повышение прав, если Hub установлен для всех пользователей.

Обновления из приложения доступны начиная с 0.2.36. Если у вас версия ниже, один раз запустите новый установщик. Чтобы обновиться вручную, закройте игру и Hub, скачайте новый `PanopticonHubSetup.exe` и установите поверх текущей версии в тот же каталог. Настройки и профиль оформления сохраняются. [Все релизы](https://github.com/werton-core/PanopticonHub/releases).

### Оформление

Нажмите кисть рядом с шестерёнкой. Во вкладке **Стиль** можно выбрать тему, изменить акцент, логотипы, имена и значки окон. **Медиа** содержит отдельные фоны лаунчера, главного меню, каталога и загрузки. Тон, насыщенность и яркость настраиваются для каждого фона. **Раскладка** и **Декор** меняют размещение элементов и оформление лаунчера.

Для изображений подходят PNG, JPEG, WebP и GIF. Для фонов подходит WebM. В лаунчере также поддерживается MP4, а видео можно добавить в качестве декора. Видео воспроизводится без звука. Значки окон задаются через ICO. Во вкладке **Звук** можно назначить MP3, OGG или WAV отдельно для лаунчера, меню игры и загрузки.

**Импорт** и **Сброс** меняют черновик. Нажмите **Применить**, чтобы сохранить изменения. Оформление игры вступит в силу при следующем запуске из Hub. Экспорт `.pthubtheme` включает выбранные медиафайлы. Активный профиль находится в `%LOCALAPPDATA%\PanopticonHub\design`.

### Сетевое подключение

Hub опрашивает серверы Panopticon для счётчиков онлайна и получает публичный каталог и онлайн Shinri Trial. Обращения к GitHub Releases зависят от режима обновлений. Steam скачивает игру и подписки Workshop. Garry’s Mod подключается к выбранному серверу напрямую. Сайты и социальные ссылки открываются по нажатию.

### Сборка

Для сборки нужны 64-битная Windows, PowerShell, Rust с целевой платформой `x86_64-pc-windows-msvc`, инструменты сборки MSVC и Windows SDK. В репозитории есть редактируемый код Rust, Lua и скрипты оформления. Сборка также использует унаследованные пакеты интерфейсов из `ui/dist` и `game-ui/dist`.

Из корня проекта выполните нужную команду.

```powershell
.\tools\build-host.ps1
.\tools\build-release.ps1
```

Первая команда собирает лаунчер в `target/release/PanopticonHub.exe`. Вторая собирает лаунчер, модуль игры, деинсталлятор и установщик. Результат сохраняется в `dist/PanopticonHubSetup.exe`. Сценарий выводит его SHA-256.

Для выпуска обновления поднимите версию в `installer/Cargo.toml`, обновите `Cargo.lock` и отправьте в репозиторий тег, например `v0.2.36`, и выполните `.\tools\publish-release.ps1 -NotesFile <release-notes.md>` с авторизованным GitHub CLI. Скрипт проверит загруженный установщик в черновике релиза и опубликует его. Если изменился только установщик, сохраните версию лаунчера и передайте готовый EXE соответствующей версии через `-HostExe` в `installer/build-setup.ps1`. Установщик сохраняет установленные файлы, если их SHA-256 совпадает с файлами из пакета.

### Авторы и материалы

- В лаунчере и игровом меню использованы части кода интерфейса и медиа из официального лаунчера команды Shinri Trial. Эти материалы включены с разрешения. [Документация Shinri Hub](https://blog.shinri-trial.ru/docs/tutorial-start/shinri-hub/).
- Для фонов академии использованы скриншоты, опубликованные командой Shinri Trial. [Описание обновления Shinri Trial 1.6](https://blog.shinri-trial.ru/updates/update-1-6/).
- На официальном сайте Shinri Trial указано ООО «ШИНРИ ДЕВЕЛОПМЕНТ». [Сайт Shinri Trial](https://shinri-trial.ru/).

Права на исходный код, медиа и товарные знаки принадлежат их авторам и правообладателям. Официальный лаунчер Shinri Trial — Shinri Hub.

[Сторонние материалы и лицензии](THIRD_PARTY.md)
