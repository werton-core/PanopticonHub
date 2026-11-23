local Favorites = {}

local cookieName = "panopticon_hub_server_favorites"
local metadata = {}
local maxEntries = 512

local function validScope(project, region)
    return project == "panopticon" and region == "cis"
        or project == "shinri" and (region == "cis" or region == "global")
end

local function validAddress(address)
    if type(address) ~= "string" or #address > 21 then return false end
    local a, b, c, d, port = address:match("^(%d+)%.(%d+)%.(%d+)%.(%d+):(%d+)$")
    a, b, c, d, port = tonumber(a), tonumber(b), tonumber(c), tonumber(d), tonumber(port)
    if not port or a < 1 or a >= 224 or b > 255 or c > 255 or d > 255 or port < 1 or port > 65535 then return false end
    if a == 10 or a == 127 or a == 169 and b == 254 or a == 172 and b >= 16 and b <= 31
        or a == 192 and b == 168 or a == 100 and b >= 64 and b <= 127 then return false end
    return address == string.format("%d.%d.%d.%d:%d", a, b, c, d, port)
end

local function serverName(name)
    if type(name) == "string" and #name <= 512 and name:find("%S") and not name:find("%c") then return name end
end

local function remember(project, region, row)
    local address = row.address
    if not validScope(project, region) or not validAddress(address) then return false end
    local previous = metadata[address]
    local name = serverName(row.name) or (previous and previous.name) or address
    if previous and previous.project == project and previous.region == region and previous.name == name then return false end
    if not previous then
        local count = 0
        for _ in pairs(metadata) do count = count + 1 end
        if count >= maxEntries then return false end
    end
    metadata[address] = {project = project, region = region, name = name}
    return true
end

local function save()
    cookie.Set(cookieName, util.TableToJSON(metadata))
end

local stored = cookie.GetString(cookieName, "")
local decoded = #stored <= 512 * 1024 and util.JSONToTable(stored)
if type(decoded) == "table" then
    for address, row in pairs(decoded) do
        if type(row) == "table" then
            remember(row.project, row.region, {address = address, name = row.name})
        end
    end
end

function Favorites.Remember(project, region, row)
    if remember(project, region, row) then save() end
end

function Favorites.Forget(address)
    if metadata[address] then
        metadata[address] = nil
        save()
    end
end

function Favorites.Get(directory, project, region, callback, active)
    if not validScope(project, region) then
        callback({}, "noGlobal")
        return
    end
    if not serverlist or not serverlist.IsServerFavorite then
        callback({}, "unavailable")
        return
    end

    local candidates, current = {}, {}
    local function consider(address)
        if not validAddress(address) or not serverlist.IsServerFavorite(address) then return end
        local row = metadata[address]
        if not row or row.project == project and row.region == region then
            candidates[address] = true
        end
    end

    for address, row in pairs(metadata) do
        if row.project == project and row.region == region then consider(address) end
    end

    local url
    local native = directory.Native
    local categories = directory.Categories[project == "panopticon" and project or region]
    if native and native.DirectoryPlan and categories then
        for _, category in ipairs(categories) do
            local encoded = native.DirectoryPlan(project, region, category)
            local plan = encoded and util.JSONToTable(encoded)
            if type(plan) == "table" then
                url = url or plan.url
                for _, address in ipairs(plan.addresses or {}) do consider(address) end
            end
        end
    end

    local function resolve(groups)
        if active and not active() then return end
        for _, rows in pairs(groups or {}) do
            for _, row in ipairs(rows) do
                consider(row.address)
                if candidates[row.address] then current[row.address] = row end
            end
        end

        local pending = {}
        for address in pairs(candidates) do
            if not current[address] then pending[#pending + 1] = address end
        end
        table.sort(pending)

        local function finish(replies)
            if active and not active() then return end
            for _, row in ipairs(replies or {}) do current[row.address] = row end
            local result, changed = {}, false
            for address in pairs(candidates) do
                if serverlist.IsServerFavorite(address) then
                    local row = current[address]
                    if row then
                        changed = remember(project, region, row) or changed
                    else
                        row = {address = address, name = (metadata[address] or {}).name or address,
                            online = false, connectable = false}
                        changed = remember(project, region, row) or changed
                    end
                    result[#result + 1] = row
                end
            end
            if changed then save() end
            callback(result, #result > 0 and "ready" or "empty")
        end

        if #pending == 0 then finish() else directory.PingAddresses(pending, finish, active) end
    end

    if url then directory.FetchRegion(url, resolve) else resolve() end
end

return Favorites
