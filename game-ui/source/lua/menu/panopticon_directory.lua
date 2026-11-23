local Directory = {}

Directory.Categories = {
    panopticon = {"lobby", "game", "favorite"},
    cis = {"lobby", "story", "game", "media", "other", "test", "favorite"},
    global = {"lobby", "story", "game", "media", "other", "test", "favorite"},
}

if not PanopticonNative then
    local ok, err = pcall(require, "panopticon_native")
    if not ok then print("[Panopticon Hub] Ошибка загрузки DLL: " .. tostring(err)) end
end
Directory.Native = PanopticonNative

local function pingAddresses(addresses, callback, active)
    if #addresses == 0 or not serverlist or not serverlist.PingServer then
        callback({}, "unavailable")
        return
    end

    local result, waiting, completed = {}, #addresses, false
    local function finish()
        if completed or (active and not active()) then return end
        completed = true
        local servers = {}
        for index = 1, #addresses do
            if result[index] then servers[#servers + 1] = result[index] end
        end
        callback(servers, #servers > 0 and "ready" or "unavailable")
    end

    local interval = 0.025
    timer.Simple(4 + (#addresses - 1) * interval, finish)
    for index, address in ipairs(addresses) do
        local function query()
            if completed or (active and not active()) then return end
            local received = false
            serverlist.PingServer(address, function(ping, name, _, map, players, maxplayers, _, password)
                if completed or received or (active and not active()) then return end
                received = true
                if type(ping) == "number" and ping >= 0 and type(name) == "string"
                    and #name <= 512 and name:find("%S") and not name:find("%c") then
                    result[index] = {address = address, name = name, map = map, players = players,
                        maxplayers = maxplayers, password = password, ping = ping}
                end
                waiting = waiting - 1
                if waiting == 0 then finish() end
            end)
        end
        if index == 1 then query() else timer.Simple((index - 1) * interval, query) end
    end
end

local function fetchRegion(url, callback)
    if not http or not http.Fetch then callback() return end
    local finished = false
    local function finish(body)
        if finished then return end
        finished = true
        local result = body and Directory.Native.DirectoryResponse(body)
        callback(result and util.JSONToTable(result))
    end
    timer.Simple(2.5, finish)
    http.Fetch(url, function(body, _, _, status)
        finish(status == 200 and body)
    end, function() finish() end, {Accept = "application/json"})
end

Directory.FetchRegion = fetchRegion
Directory.PingAddresses = pingAddresses

function Directory.GetServers(project, region, category, callback, active)
    if project == "panopticon" and region == "global" then
        callback({}, "noGlobal")
        return
    end
    local encoded = Directory.Native and Directory.Native.DirectoryPlan(project, region, category)
    local plan = encoded and util.JSONToTable(encoded)
    if not plan then callback({}, "unavailable") return end
    local function resolve(groups)
        if active and not active() then return end
        local rows = groups and groups[category]
        if type(rows) == "table" then
            callback(rows, #rows > 0 and "ready" or "empty")
        else
            pingAddresses(plan.addresses, callback, active)
        end
    end
    if plan.url then fetchRegion(plan.url, resolve) else resolve() end
end

return Directory
