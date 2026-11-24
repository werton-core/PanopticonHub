if type(GameDetails) ~= "function" then return end
if not PanopticonNative then pcall(require, "panopticon_native") end

local environment = getfenv(GameDetails)
local preference = {GetBool = function()
    local native = environment.PanopticonNative
    return native ~= nil and native.GetServerLoadingScreens ~= nil and native.GetServerLoadingScreens() == true
end}

-- Только GameDetails читает предпочтение Hub. Штатный ConVar остаётся общим для Steam.
setfenv(GameDetails, setmetatable({
    GetConVar = function(name)
        if name == "cl_enable_loadingurl" then return preference end
        return environment.GetConVar(name)
    end,
}, {__index = environment, __newindex = environment}))
