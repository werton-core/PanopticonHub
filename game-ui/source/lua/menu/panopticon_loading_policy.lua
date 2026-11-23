if type(GameDetails) ~= "function" then return end

local environment = getfenv(GameDetails)
local disabled = {GetBool = function() return false end}

-- Сохраняем локальный экран загрузки в сессии Hub. Остальные вызовы GetConVar работают как обычно.
setfenv(GameDetails, setmetatable({
    GetConVar = function(name)
        if name == "cl_enable_loadingurl" then return disabled end
        return environment.GetConVar(name)
    end,
}, {__index = environment, __newindex = environment}))
