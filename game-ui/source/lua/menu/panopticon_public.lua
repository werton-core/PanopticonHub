-- Реконструкция: адаптер подлинного HTML renderer к стандартному Steam GMod.
-- Совместимость Shinri включается отдельно; игровой процесс и Steam остаются штатными.
stMenu = stMenu or {}
local hub = stMenu
if hub.PanopticonReady then hub.Ready() return end
local directory = include("menu/panopticon_directory.lua")
local favorites = include("menu/panopticon_favorites.lua")
local native = directory.Native
local selection = {project = "panopticon", region = "cis", category = "lobby"}
if file.Read("panopticon_hub_region.txt", "DATA") == "eu" then
    selection.project, selection.region = "shinri", "global"
end
local categoryCodes = {lobby = "l", game = "f", story = "g", test = "t", media = "m", favorite = "favorite"}
local rendererCategories = {game = "fast_game", story = "game"}
local serversByAddress = {}
local connectedProjects = {}
local scan = 0
local revision = 0
local menuDefaults = {mainMenuMusic = true, menuSoundEffects = true, musicVolume = 0.3}
local sharedDefaults = {
    volume_dialogs = 0.8, volume_dances = 0.6, corpseDiscoverSound = true, monokumaNotifySound = true,
    closelookSound = true, musicInDances = true, musicInOwnDances = true, showTooltips = true,
    showBindTooltips = true, showNamesAboveHead = true, showHUD = true, showInteractionMarkers = true,
    closelookPostprocessing = true, displayQuests = true, corpseDetectNoise = true, streamerMode = false,
    droview_bob = true, dro_thirdperson = false, dro_thirdperson_crosshair = true,
    dro_thirdperson_pos_updown = 0, dro_thirdperson_pos_rightleft = 20, dro_thirdperson_pos_forwardback = -85
}
local menuCookie = "panopticon_hub_menu_shared"
local shared = table.Copy(menuDefaults)
local engineMap = {
    mouseSensitivity = "sensitivity", volumeMaster = "volume", shadowDepth = "r_flashlightdepthtexture", waterExpensive = "r_waterforceexpensive",
    shaderHdrLevel = "mat_hdr_level", textureQuality = "mat_picmip", shadowQuality = "r_shadowrendertotexture", vsync = "mat_vsync",
    mouseRawInput = "m_rawinput", volumeEffects = "volume_sfx", textureAnisotropy = "mat_forceaniso",
    motionBlur = "mat_motion_blur_enabled", antiAliasing = "mat_antialias", shaderBumpmap = "mat_bumpmap", waterReflectEntities = "r_waterforcereflectentities",
    mouseAcceleration = "m_customaccel", soundMuteLoseFocus = "snd_mute_losefocus", fieldOfView = "fov_desired", mousePitch = "m_pitch",
    modelQuality = "r_rootlod", shaderSpecular = "mat_specular", mouseFilter = "m_filter", frameRate = "fps_max",
    textureTrilinear = "mat_trilinear", voiceEnabled = "voice_enable", voicePlayers = "voice_scale"
}
local bindingDefaults = {
    ["+forward"] = "w", ["+back"] = "s", ["+moveleft"] = "a", ["+moveright"] = "d",
    ["+jump"] = "space", ["+duck"] = "ctrl", ["+speed"] = "shift", ["+walk"] = "alt",
    ["+attack"] = "mouse1", ["+attack2"] = "mouse2", ["+reload"] = "r", ["+use"] = "e",
    ["+menu"] = "q", ["+menu_context"] = "c", ["+voicerecord"] = "x", gmod_undo = "z",
    invprev = "mwheelup", invnext = "mwheeldown", lastinv = "", noclip = "v",
    messagemode = "y", toggleconsole = "`"
}
local customBindings = {
    ["+closelook"] = true, ["+interact"] = true, ["+inspect"] = true, ["+openinv"] = true,
    ["+fast_actions"] = true, ["+dro_graffiti_place"] = true, ["+dro_thirdperson_toggle"] = true,
    ["+dro_music_player"] = true
}
local function call(name, ...)
    if not IsValid(pnlMainMenu) then return end
    local args = {}
    for index = 1, select("#", ...) do
        local value = select(index, ...)
        if isstring(value) then args[#args + 1] = string.format("%q", value)
        elseif istable(value) then args[#args + 1] = util.TableToJSON(value) or "{}"
        elseif isbool(value) then args[#args + 1] = tostring(value)
        elseif isnumber(value) then args[#args + 1] = tostring(value)
        else args[#args + 1] = "null" end
    end
    pnlMainMenu:Call("window." .. name .. " && window." .. name .. "(" .. table.concat(args, ",") .. ")")
end
local function settingsError(message)
    call("PTHubSettingsNotice", message)
end
local function validAddress(value)
    if not isstring(value) then return false end
    local a, b, c, d, port = value:match("^(%d+)%.(%d+)%.(%d+)%.(%d+):(%d+)$")
    a, b, c, d, port = tonumber(a), tonumber(b), tonumber(c), tonumber(d), tonumber(port)
    if not port or not a or not b or not c or not d or a > 255 or b > 255 or c > 255 or d > 255 or port < 1 or port > 65535 then return false end
    return not (a == 0 or a == 10 or a == 127 or a >= 224 or a == 169 and b == 254 or a == 172 and b >= 16 and b <= 31 or a == 192 and b == 168 or a == 100 and b >= 64 and b <= 127)
end
local reconnect = include("menu/panopticon_reconnect.lua").New(validAddress, SysTime)
local function categories(project, region)
    return directory.Categories[project == "panopticon" and "panopticon" or region]
end
local function rendererCategory(category)
    return selection.project == "shinri" and rendererCategories[category] or category
end
local function showSelection(status)
    local tabs = {}
    for index, category in ipairs(categories(selection.project, selection.region)) do
        tabs[index] = rendererCategory(category)
    end
    call("PTHubProjectState", {
        project = selection.project, region = selection.region, category = rendererCategory(selection.category),
        categories = tabs, status = status
    })
end
function hub.SelectDirectory(project, region, category)
    if project ~= "panopticon" and project ~= "shinri" then return end
    if region == "eu" then region = "global" end
    if region ~= "cis" and region ~= "global" then return end
    if project ~= selection.project then
        if category ~= "favorite" then category = "lobby" end
        if project == "panopticon" then region = "cis" end
    end
    local available = categories(project, region)
    if not table.HasValue(available, category) then category = available[1] end
    if selection.project == project and selection.region == region and selection.category == category then return end
    selection.project, selection.region, selection.category = project, region, category
    scan = scan + 1
    revision = revision + 1
    serversByAddress = {}
    showSelection(project == "panopticon" and region == "global" and "noGlobal" or "loading")
    call("PTHubRefresh")
end
function hub.ScanServers(token, requestId)
    if not isstring(token) or #token > 80 or not token:match("^[%w%-]+$") then return end
    scan = scan + 1
    local activeScan = scan
    local category = selection.category
    local categoryCode = selection.project == "panopticon" and category == "game" and "g" or categoryCodes[category]
    local function active() return scan == activeScan end
    local function complete(servers, status)
        if not active() then return end
        serversByAddress = {}
        for _, server in ipairs(servers) do
            if validAddress(server.address) then
                server.project = selection.project
                serversByAddress[server.address] = server
                call("AddServer", token, requestId, server.ping or 0, server.name, "", server.map or "",
                    server.players or "—", server.maxplayers or "—", 0, server.password == true, 0, server.address, "",
                    "0", false, tostring(VERSION), serverlist.IsServerFavorite(server.address), "", categoryCode,
                    server.connectable ~= false, server.online == false)
            end
        end
        showSelection(status)
        call("FinishedServers", token)
    end
    showSelection(selection.project == "panopticon" and selection.region == "global" and "noGlobal" or "loading")
    if category == "favorite" then
        favorites.Get(directory, selection.project, selection.region, complete, active)
    else
        directory.GetServers(selection.project, selection.region, category, complete, active)
    end
end

function hub.SetServerFavorite(address, enabled)
    local row = serversByAddress[address]
    if not row or not validAddress(address) or not isbool(enabled) then return end
    if enabled then
        serverlist.AddServerToFavorites(address)
        favorites.Remember(selection.project, selection.region, row)
    else
        serverlist.RemoveServerFromFavorites(address)
        favorites.Forget(address)
        if selection.category == "favorite" then serversByAddress[address] = nil end
    end
end

function hub.PingServer(address)
    if not serversByAddress[address] then return end
    local selectedRevision = revision
    serverlist.PingServer(address, function(ping, name, _, map, players, maxplayers, bots, password, _, updated)
        if selectedRevision == revision and name then
            call("UpdateServer", address, ping or 0, name, map or "", players or 0, maxplayers or 0, bots or 0, password == true)
        end
    end)
end
local function gameState()
    if native and native.GameState then return native.GameState() end
    return IsInGame(), IsInLoading()
end

local function currentAddress()
    local address = engine.CurrentServerAddress()
    return isstring(address) and address or ""
end

local currentServer, lastPing
local function updateCurrentServer(inGame, force)
    local address = inGame and currentAddress()
    if not inGame then
        if force or currentServer then call("PTHubCurrentServer", nil) end
        currentServer = nil
        return
    end

    if not currentServer or currentServer.address ~= address then
        local row = serversByAddress[address]
        currentServer = {address = address, name = row and row.name,
            project = connectedProjects[address] or row and row.project,
            online = row and tonumber(row.players), maxPlayers = row and tonumber(row.maxplayers)}
        lastPing = nil
        force = true
    end
    if force then call("PTHubCurrentServer", currentServer) end

    if not validAddress(address) or not gui.IsGameUIVisible() then return end
    local now = SysTime()
    if lastPing and now - lastPing < 15 then return end
    lastPing = now
    local request = currentServer
    serverlist.PingServer(address, function(_, name, _, _, players, maxPlayers)
        local active, loading = gameState()
        if not name or not active or loading or currentServer ~= request or currentAddress() ~= address then return end
        request.name, request.online, request.maxPlayers = name, tonumber(players), tonumber(maxPlayers)
        call("PTHubCurrentServer", request)
    end)
end

local connection = include("menu/panopticon_identity.lua").New({
    now = SysTime,
    valid = function(address, confirmed)
        local inGame, loading = gameState()
        if loading or not validAddress(address) then return false end
        if inGame then return address ~= currentAddress() and confirmed == currentAddress() end
        return confirmed == nil
    end,
    prepare = function(address, project)
        if not native or not native.PrepareConnection then return {status = "error", message = "Игровой модуль Hub недоступен."} end
        return util.JSONToTable(native.PrepareConnection(address, project) or "")
    end,
    poll = function(id) return util.JSONToTable(native.PollConnection(id) or "") end,
    cancel = function() if native and native.CancelConnection then native.CancelConnection() end end,
    status = function(value) call("PTHubConnectionPreparation", value) end,
    error = settingsError,
    idle = function() local inGame, loading = gameState() return not inGame and not loading end,
    reset = function()
        if not native or not native.ResetConnectionProtocol then return false, "Игровой модуль Hub недоступен." end
        return native.ResetConnectionProtocol()
    end,
    join = function(address, retry, project)
        if retry then
            local inGame, loading = gameState()
            if not reconnect.Take(address, inGame, loading) then return end
        else reconnect.Clear(currentAddress()) end
        call("PTHubReconnect", nil)
        hub.SetMicrophoneTest(false)
        connectedProjects[address] = project
        JoinServer(address)
    end,
})
hub.Connection = connection
function hub.CancelConnection() connection.Cancel() end

function hub.ConnectToServer(address, confirmedAddress)
    local inGame, loading = gameState()
    if loading then return end
    if inGame then
        local current = currentAddress()
        if address == current or confirmedAddress ~= current then return end
    elseif confirmedAddress ~= nil then return end

    local server = serversByAddress[address]
    if not server or server.connectable == false then return end
    connection.Start(address, confirmedAddress, false, server.project or selection.project)
end

function hub.DisconnectFromServer(confirmedAddress)
    local inGame, loading = gameState()
    if not inGame or loading or confirmedAddress ~= currentAddress() then return end
    connection.Cancel()
    reconnect.Clear(confirmedAddress)
    call("PTHubReconnect", nil)
    hub.SetMicrophoneTest(false)
    if native and native.GameAction and native.GameAction("Disconnect") then return end
    RunGameUICommand("Disconnect")
end

function hub.RequestInGame()
    local inGame, loading = gameState()
    call("SetInGame", inGame)
    if not loading then updateCurrentServer(inGame, true) end
    call("PTHubReconnect", reconnect.Get())
end

function hub.RetryConnection(address)
    local inGame, loading = gameState()
    local target = reconnect.Get()
    if inGame or loading or not target or target.address ~= address then return end
    connection.Start(address, nil, true, target.project or connectedProjects[address] or "panopticon")
end
function hub.RequestLoading()
    local _, loading = gameState()
    call("SetLoading", loading)
end
function hub.ApplyHubLanguage()
    call("UpdateLanguage", GetConVarString("gmod_language"))
    call("UpdateHubRegion", selection.region == "global" and "eu" or "cis")
end
local function readClientConfig()
    local data = native and native.ReadSharedSettings and native.ReadSharedSettings()
    return data and util.JSONToTable(data) or {}
end
function hub.RequestEngineSettings()
    local result = {}
    for key, command in pairs(engineMap) do
        local value = GetConVar(command)
        if value then result[key] = value:GetFloat() end
    end
    if native and native.GetVoiceSettings then
        local voice = util.JSONToTable(native.GetVoiceSettings() or "")
        if istable(voice) then table.Merge(result, voice) end
    end
    if result.volumeEffects == nil then result.volumeEffects = readClientConfig()["volume.effects"] or 0.7 end
    call("STEngineSettings", result)
end
local function writeClientConfig(changes)
    if not native or not native.ApplySharedSettings then return false end
    return native.ApplySharedSettings(util.TableToJSON(changes))
end
local loadingSettings = {
    serverLoadingScreens = {get = "GetServerLoadingScreens", set = "SetServerLoadingScreens", default = false, label = "Экраны загрузки серверов"},
    loadingIndicator = {get = "GetLoadingIndicator", set = "SetLoadingIndicator", default = false, label = "Штатный индикатор загрузки"},
    shinriIdentity = {get = "GetShinriIdentity", set = "SetShinriIdentity", default = true, label = "Идентификатор Shinri Hub"}
}
local function loadingValue(setting)
    if native and native[setting.get] and native[setting.set] then return native[setting.get]() end
end
local function withLoadingSettings(values)
    for key, setting in pairs(loadingSettings) do values[key] = loadingValue(setting) end
    return values
end
local function saveLoadingSetting(setting, value, action)
    local previous = loadingValue(setting)
    if previous == nil then return false end
    if previous == value or native[setting.set](value) then return true end
    settingsError("Не удалось " .. action .. " настройку «" .. setting.label .. "».")
    return false
end
function hub.RequestSharedSettings()
    local values = table.Copy(shared)
    local data = readClientConfig()
    for key, default in pairs(sharedDefaults) do
        if type(data[key]) == type(default) then
            values[key] = data[key]
        else
            values[key] = default
        end
    end
    call("STSharedSettings", withLoadingSettings(values))
end
local function nativeBindings(name)
    local raw = native and native[name] and native[name]()
    return raw and util.JSONToTable(raw) or {}
end
function hub.RequestBindings()
    local values = nativeBindings("ReadCustomBindings")
    for command in pairs(bindingDefaults) do
        values[command] = input.LookupBinding(command, true) or ""
    end
    call("STBindings", values)
end
function hub.RequestVideoModes()
    local state = native and native.VideoModes and util.JSONToTable(native.VideoModes() or "")
    if istable(state) then
        call("STVideoModes", state.modes or {}, state.current or {})
    else
        call("STVideoModes", {}, {})
    end
end
local function finite(value)
    return isnumber(value) and value == value and math.abs(value) < 100000
end
local videoCommands = {
    r_flashlightdepthtexture = true, r_waterforceexpensive = true, mat_hdr_level = true,
    mat_picmip = true, r_shadowrendertotexture = true, mat_vsync = true, mat_forceaniso = true,
    mat_motion_blur_enabled = true, mat_antialias = true, mat_bumpmap = true,
    r_waterforcereflectentities = true, r_rootlod = true, mat_specular = true, mat_trilinear = true
}
local videoSavePending = false
local deferredVideoReset = {}
local deferredIdentityReset = false
local quitAfterReset = false
local function saveEngine(video)
    videoSavePending = videoSavePending or video == true
    timer.Create("PanopticonHub.SaveSettings", 0.4, 1, function()
        if native and native.SaveEngineConfig then native.SaveEngineConfig(videoSavePending) end
        videoSavePending = false
    end)
end
function hub.SetLanguage(locale)
    if not isstring(locale) or not locale:match("^[a-z][a-z%-]+$")
        or not file.Exists("resource/localization/" .. locale .. ".png", "MOD") then return end

    RunConsoleCommand("gmod_language", locale)
    saveEngine()
end

local function applyEngineSettings(values)
    local commands, video, changed, failed = {}, false, false, false
    for key, value in pairs(values) do
        local command, number = engineMap[key], tonumber(value)
        if finite(number) then
            local convar = command and GetConVar(command)
            if convar then
                if math.abs(convar:GetFloat() - number) > 0.00001 then
                    commands[command] = number
                    video = video or videoCommands[command] == true
                end
            elseif key == "volumeEffects" then
                local volume = math.Clamp(number, 0, 1)
                if not writeClientConfig({["volume.effects"] = volume}) then failed = true end
            elseif (key == "voiceMicrophone" or key == "voiceBoost") and native and native.SetVoiceSetting then
                if native.SetVoiceSetting(key, number) then changed = true else failed = true end
            end
        end
    end
    if next(commands) then
        if native and native.ApplyEngineSettings and native.ApplyEngineSettings(util.TableToJSON(commands)) then
            changed = true
        else
            failed = true
        end
    end
    if changed then saveEngine(video) end
    if failed then settingsError("Не удалось применить настройки Garry’s Mod.") end
end
function hub.ApplyEngineSettings(raw)
    local values = util.JSONToTable(tostring(raw or "")) or {}
    for key, value in pairs(values) do
        if finite(tonumber(value)) then deferredVideoReset[key] = nil end
    end
    applyEngineSettings(values)
end
function hub.ApplySharedSettings(raw)
    local values = util.JSONToTable(tostring(raw or "")) or {}
    local changes, menuChanged, loadingTouched = {}, false, false
    for key, value in pairs(values) do
        local default = sharedDefaults[key]
        local loadingSetting = loadingSettings[key]
        if loadingSetting and isbool(value) then
            if key == "shinriIdentity" and (connection.Busy() or IsInGame() or IsInLoading()) then
                hub.RequestSharedSettings()
            else
                saveLoadingSetting(loadingSetting, value, "сохранить")
                loadingTouched = true
            end
        elseif key == "musicVolume" and finite(value) then
            shared[key], menuChanged = math.Clamp(value, 0, 1), true
        elseif type(menuDefaults[key]) == "boolean" and isbool(value) then
            shared[key], menuChanged = value, true
        elseif default ~= nil and type(value) == type(default) and (not isnumber(value) or finite(value)) then
            if key == "volume_dialogs" or key == "volume_dances" then value = math.Clamp(value, 0, 1) end
            changes[key] = value
        end
    end
    if menuChanged then
        cookie.Set(menuCookie, util.TableToJSON(shared))
        call("STSharedSettings", withLoadingSettings(table.Copy(shared)))
    end
    if next(changes) ~= nil then
        local saved, active = writeClientConfig(changes)
        if not saved then
            settingsError(active and "Не удалось применить настройки в текущей игре." or "Не удалось сохранить настройки этого клиента.")
            hub.RequestSharedSettings()
            return
        end
        if active then hub.RequestSharedSettings() end
    end
    if loadingTouched and not menuChanged then hub.RequestSharedSettings() end
end
local function bindableKey(key)
    return isstring(key) and #key > 0 and #key <= 30 and key:match("^[%w_`%[%]'/.,=+;%\\%-]+$") ~= nil
end

function hub.SetBindCapture(enabled)
    local id = "PanopticonHub.BindCapture"
    timer.Remove(id)
    if enabled ~= true and enabled ~= 1 and enabled ~= "1" and enabled ~= "true" then return end

    local keys, down = {}, {}
    local function watch(first, last, isDown)
        for code = first, last do
            local name = input.GetKeyName(code)
            if code == KEY_ESCAPE or (bindableKey(name) and not name:upper():find("TOGGLE", 1, true)) then
                keys[#keys + 1] = {code = code, name = name, isDown = isDown}
                down[code] = isDown(code)
            end
        end
    end
    watch(KEY_0, KEY_LAST, input.IsKeyDown)
    watch(MOUSE_FIRST, MOUSE_LAST, input.IsMouseDown)
    timer.Create(id, 0, 0, function()
        if not IsValid(pnlMainMenu) or IsInLoading()
            or (gui and gui.IsGameUIVisible and not gui.IsGameUIVisible()) then
            timer.Remove(id)
            call("STBindCaptureCancelled")
            return
        end
        for _, key in ipairs(keys) do
            local pressed = key.isDown(key.code)
            if pressed and not down[key.code] then
                timer.Remove(id)
                if key.code == KEY_ESCAPE then
                    call("STBindCaptureCancelled")
                else
                    call("STBindCaptured", key.name)
                end
                return
            end
            down[key.code] = pressed
        end
    end)
end

local function liveBindingKeys()
    local keys = {}
    for code = KEY_0, BUTTON_CODE_LAST or JOYSTICK_LAST or MOUSE_LAST do
        local command = input.LookupKeyBinding(code)
        if isstring(command) and command ~= "" then
            local key = input.GetKeyName(code)
            if bindableKey(key) then keys[key:lower()] = command end
        end
    end
    return keys
end

local function applyStockBindings(plan, applyCustom)
    if not next(plan) then
        applyCustom()
        hub.RequestBindings()
        return
    end
    if not (native and native.ApplyEngineBindings and native.ApplyEngineBindings(util.TableToJSON(plan))) then
        settingsError("Не удалось применить привязки Garry’s Mod.")
        hub.RequestBindings()
        return
    end
    timer.Simple(0.1, function()
        for key, command in pairs(plan) do
            local code = input.GetKeyCode(key)
            if (input.LookupKeyBinding(code) or ""):lower() ~= command then
                settingsError("Garry’s Mod не применил одну из привязок клавиш.")
                hub.RequestBindings()
                return
            end
        end
        applyCustom()
        hub.RequestBindings()
    end)
end

local function applyBindings(changes, resetCustom)
    local stock, custom, requested = {}, {}, {}
    local available = nativeBindings("ReadCustomBindings")
    if resetCustom then
        for command in pairs(available) do custom[command] = false end
    end
    for command, key in pairs(changes) do
        if (bindingDefaults[command] == nil and not customBindings[command]) or (key ~= "" and not bindableKey(key)) then
            settingsError("Недопустимая привязка клавиши.")
            hub.RequestBindings()
            return
        end
        local code = key == "" and -1 or (input.GetKeyCode(key:lower()) or 0)
        if (key ~= "" and code <= 0) or code == KEY_ESCAPE then
            settingsError("Garry’s Mod не распознал выбранную клавишу.")
            hub.RequestBindings()
            return
        end
        if customBindings[command] then
            if available[command] ~= nil then custom[command] = code end
        else
            if key ~= "" then
                key = input.GetKeyName(code)
                if not bindableKey(key) then
                    settingsError("Garry’s Mod не распознал выбранную клавишу.")
                    hub.RequestBindings()
                    return
                end
                key = key:lower()
            end
            if key ~= "" and requested[key] then
                settingsError("Одна клавиша назначена нескольким действиям.")
                hub.RequestBindings()
                return
            end
            if key ~= "" then requested[key] = command end
            if (input.LookupBinding(command, true) or ""):lower() ~= key then stock[command] = key end
        end
    end

    local plan = {}
    if next(stock) or next(custom) then
        for key, command in pairs(liveBindingKeys()) do
            command = command:lower()
            if stock[command] ~= nil or custom[command] ~= nil then plan[key] = "" end
        end
        for command, key in pairs(stock) do
            if key ~= "" then plan[key] = command end
        end
    end
    applyStockBindings(plan, function()
        for command, code in pairs(custom) do
            if not native.SetCustomBinding(command, code or nil) then
                settingsError("Не удалось применить клавишу действия игрового режима.")
                return
            end
        end
    end)
end
function hub.ApplyBindings(raw)
    local changes = isstring(raw) and #raw <= 4096 and util.JSONToTable(raw)
    if istable(changes) then applyBindings(changes, false) end
end
function hub.ApplyVideoMode(width, height, mode)
    if native and native.ApplyVideoMode and native.ApplyVideoMode(tonumber(width), tonumber(height), mode) then
        timer.Simple(0.35, hub.RequestVideoModes)
    else
        settingsError("Не удалось применить выбранный видеорежим.")
    end
end

local voiceTimer = "PanopticonHub.VoiceTest"
local voiceStatus = "idle"

local function voiceVolumePercent()
    local gain = native and native.VoiceGain and native.VoiceGain()
    if gain then return math.floor(gain * 100 + 0.5) end
end

function hub.RequestVoiceTestState()
    call("PTVoiceTestState", voiceStatus, voiceVolumePercent())
end

local function stopVoiceTest(status)
    timer.Remove(voiceTimer)
    if voiceStatus == "active" and native and native.VoiceStop then native.VoiceStop() end
    if voiceStatus ~= status then
        voiceStatus = status
        hub.RequestVoiceTestState()
    end
end

function hub.SetMicrophoneTest(enabled)
    if enabled ~= true and enabled ~= 1 and enabled ~= "1" and enabled ~= "true" then
        stopVoiceTest("idle")
        return
    end
    if IsInGame() or IsInLoading() then
        stopVoiceTest("connected")
        return
    end
    if not native or not native.VoiceStart or not native.VoiceStart() then
        stopVoiceTest("unavailable")
        return
    end
    voiceStatus = "active"
    hub.RequestVoiceTestState()
    timer.Create(voiceTimer, 0.04, 0, function()
        if not IsValid(pnlMainMenu) or (gui and gui.IsGameUIVisible and not gui.IsGameUIVisible()) then
            stopVoiceTest("idle")
            return
        end
        if IsInGame() or IsInLoading() then
            stopVoiceTest("connected")
            return
        end
        local level = native.VoicePoll()
        if level == nil then
            stopVoiceTest("unavailable")
            return
        end
        call("PTVoiceTestFrame", level)
    end)
end

function hub.SetMicrophoneVolume(percent, commit)
    local volume = tonumber(percent)
    if voiceStatus ~= "active" or not volume or volume ~= volume or volume < 0 or volume > 100 then return end
    if native.VoiceSetGain then native.VoiceSetGain(volume / 100) end
    if commit == 1 then call("PTVoiceTestVolume", voiceVolumePercent()) end
end

local pendingUrlPanel
local function openConfirmedURL(url)
    if not native.OpenConfirmedURL(url) then settingsError("Не удалось открыть эту ссылку.") end
end

function hub.OpenURL(url)
    if native and native.OpenURL and native.OpenConfirmedURL and isfunction(engine.CurrentServerAddress) then
        -- В главном меню движок возвращает ноль значений, а штатный диалог
        -- вызывает tostring напрямую. Нормализуем результат только при открытии.
        local currentAddress = engine.CurrentServerAddress
        local createPanel = vgui.CreateFromTable
        local newPanel
        engine.CurrentServerAddress = function()
            local address = currentAddress()
            return address
        end
        vgui.CreateFromTable = function(definition, ...)
            local panel = createPanel(definition, ...)
            if isfunction(definition.DoYesAction) and panel.Type == "openurl" then newPanel = panel end
            return panel
        end
        local ok, opened = pcall(native.OpenURL, url)
        engine.CurrentServerAddress = currentAddress
        vgui.CreateFromTable = createPanel
        if ok and opened then
            local panel = IsValid(newPanel) and newPanel or pendingUrlPanel
            if IsValid(panel) and panel.Type == "openurl" and panel:GetURL() == url then
                local original = panel.PanopticonOpenURL or panel.DoYesAction
                panel.PanopticonOpenURL = original
                panel.DoYesAction = function(self, ...)
                    if self.Type == "openurl" and self:GetURL() == url then
                        openConfirmedURL(url)
                    else
                        original(self, ...)
                    end
                end
                pendingUrlPanel = panel
                return
            end
        end
    end
    settingsError("Не удалось открыть эту ссылку.")
end
local function quitGame()
    if timer.Exists("PanopticonHub.SaveSettings") then
        timer.Remove("PanopticonHub.SaveSettings")
        if native and native.SaveEngineConfig then native.SaveEngineConfig(videoSavePending) end
        videoSavePending = false
    end
    if native and native.GameAction and native.GameAction("Quit") then return end
    RunGameUICommand("QuitNoConfirm")
end
function hub.Quit()
    connection.Cancel()
    stopVoiceTest("idle")
    if timer.Exists("PanopticonHub.DeferredReset") then
        quitAfterReset = true
        if IsInGame() then RunGameUICommand("Disconnect") end
        return
    end
    quitGame()
end
local function finishDeferredReset()
    if IsInGame() or IsInLoading() or connection.Busy() then return end
    timer.Remove("PanopticonHub.DeferredReset")
    local graphics = deferredVideoReset
    deferredVideoReset = {}
    if next(graphics) then applyEngineSettings(graphics) end
    if deferredIdentityReset then
        deferredIdentityReset = false
        saveLoadingSetting(loadingSettings.shinriIdentity, true, "сбросить")
    end
    timer.Simple(0.25, function()
        hub.RequestEngineSettings()
        hub.RequestSharedSettings()
        hub.RequestVideoModes()
    end)
    if quitAfterReset then timer.Simple(0.6, quitGame) end
end
function hub.ResetAllSettings()
    if connection.Busy() or IsInLoading() then
        call("STSettingsReset", false)
        settingsError("Сброс настроек доступен после завершения подключения.")
        return
    end
    call("STSettingsReset", true)
    local inGame = IsInGame()
    local values = {}
    local voice = nativeBindings("GetVoiceSettings")
    if voice.voiceMicrophone ~= nil and voice.voiceMicrophone ~= 1 then values.voiceMicrophone = 1 end
    if voice.voiceBoost ~= nil and voice.voiceBoost ~= 0 then values.voiceBoost = 0 end
    deferredVideoReset = {}
    for key, command in pairs(engineMap) do
        local convar = GetConVar(command)
        if convar then
            local default = tonumber(convar:GetDefault())
            if finite(default) and math.abs(convar:GetFloat() - default) > 0.00001 then
                if inGame and videoCommands[command] then deferredVideoReset[key] = default
                else values[key] = default end
            end
        end
    end
    applyEngineSettings(values)
    shared = table.Copy(menuDefaults)
    cookie.Set(menuCookie, util.TableToJSON(shared))
    for key, setting in pairs(loadingSettings) do
        if key == "shinriIdentity" and inGame then
            deferredIdentityReset = loadingValue(setting) == false
        else
            saveLoadingSetting(setting, setting.default, "сбросить")
        end
    end
    local defaults = table.Copy(sharedDefaults)
    defaults["volume.effects"] = 0.7
    local reset = writeClientConfig(defaults)
    if not reset then settingsError("Не удалось сбросить настройки этого клиента.") end
    applyBindings(bindingDefaults, true)
    if next(deferredVideoReset) or deferredIdentityReset then
        timer.Create("PanopticonHub.DeferredReset", 0.25, 0, finishDeferredReset)
        local message = next(deferredVideoReset) and "Настройки графики будут сброшены после выхода с сервера."
            or "Идентификатор Shinri Hub будет сброшен после выхода с сервера."
        settingsError(message)
    end
    timer.Simple(0.2, function()
        hub.RequestEngineSettings()
        hub.RequestSharedSettings()
        hub.RequestBindings()
        hub.RequestVideoModes()
    end)
end
local lastWindowMediaVisible
local lastWindowMediaCheck
local function publishWindowMediaVisibility(force)
    if not native or not native.WindowMediaVisible then return end
    local now = SysTime()
    local resumed = lastWindowMediaCheck ~= nil and now - lastWindowMediaCheck > 0.75
    lastWindowMediaCheck = now
    local visible = native.WindowMediaVisible()
    if isbool(visible) and (force or (visible and resumed) or visible ~= lastWindowMediaVisible) then
        lastWindowMediaVisible = visible
        call("PTHubGameMediaWake", visible, visible and resumed)
    end
end
function hub.Ready()
    if native and native.ApplyWindowIdentity then native.ApplyWindowIdentity() end
    publishWindowMediaVisibility(true)
    hub.ApplyHubLanguage()
    hub.RequestInGame()
    hub.RequestLoading()
    call("PTHubConnectionPreparation", connection.Get())
    hub.RequestEngineSettings()
    hub.RequestSharedSettings()
    hub.RequestBindings()
    hub.RequestVideoModes()
    showSelection("ready")

    if hub.IntroCompleted then
        call("PTHubIntroAlreadyPlayed")
        return
    end

    -- Дождаться, пока главное меню начнёт рисоваться без пауз после запуска игры.
    local previous, stableSince = SysTime(), nil
    timer.Create("PanopticonHub.IntroReady", 0, 0, function()
        local now = SysTime()
        if now - previous > 0.125 or not gui.IsGameUIVisible() then
            stableSince = nil
        else
            stableSince = stableSince or now
        end
        previous = now
        if stableSince and now - stableSince >= 0.4 and IsValid(pnlMainMenu) then
            timer.Remove("PanopticonHub.IntroReady")
            call("PTHubIntroReady")
        end
    end)
end
function hub.CompleteIntro()
    hub.IntroCompleted = true
    timer.Remove("PanopticonHub.IntroReady")
end
function STSetLoadingTranslations() end
function STSetDiscordTranslations() end
local stored = util.JSONToTable(cookie.GetString(menuCookie, ""))
if istable(stored) then for key in pairs(shared) do if type(stored[key]) == type(shared[key]) then shared[key] = stored[key] end end end
local lastInGame, lastLoading, lastReconnect
timer.Create("PanopticonHub.Loading", 0.15, 0, function()
    publishWindowMediaVisibility(false)
    connection.Poll()
    local inGame, loading = gameState()
    if (lastInGame or lastLoading) and not inGame and not loading then connection.Cleanup() end
    if inGame ~= lastInGame then
        lastInGame = inGame
        call("SetInGame", inGame)
    end
    if loading ~= lastLoading then
        lastLoading = loading
        call("SetLoading", loading)
    end
    if not loading then updateCurrentServer(inGame, false) end
    local nextReconnect = reconnect.Observe(inGame, loading, currentServer)
    if nextReconnect ~= lastReconnect then
        lastReconnect = nextReconnect
        call("PTHubReconnect", nextReconnect)
    end
end)
if native and native.ApplyWindowIdentity then
    timer.Create("PanopticonHub.WindowIdentity", 1, 0, native.ApplyWindowIdentity)
end
concommand.Add("panopticon_hub_menu", function()
    if IsValid(pnlMainMenu) and IsValid(pnlMainMenu.HTML) then pnlMainMenu.HTML:OpenURL("asset://garrysmod/html/menu.html") gui.ActivateGameUI() end
end)
hub.ValidAddress = validAddress
hub.PanopticonReady = true
