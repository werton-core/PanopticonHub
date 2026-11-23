-- Подготовка завершается до штатного JoinServer; Lua остаётся в главном потоке.
local module = {}
function module.New(api)
    local pending
    local cleanupAt, cleanupError
    local state = {}
    local function finish(message)
        pending = nil
        api.status(nil)
        if message then api.error(message) end
    end
    function state.Cleanup() cleanupAt = api.now() end
    function state.Busy() return pending ~= nil end
    function state.Cancel()
        if not pending then return end
        api.cancel()
        state.Cleanup()
        finish()
    end
    local function accept(result)
        local request = pending
        if not request then return end
        if type(result) ~= "table" then state.Cleanup() finish("Не получен ответ подготовки подключения.") return end
        if result.status == "pending" and type(result.id) == "number" then
            request.id = result.id
            return
        end
        if result.status ~= "ready" then state.Cleanup() finish(result.message or "Не удалось подготовить подключение.") return end
        if not api.valid(request.address, request.confirmed) then state.Cancel() return end
        finish()
        cleanupAt, cleanupError = api.now() + 2, nil
        api.join(request.address, request.retry, request.project)
    end
    function state.Start(address, confirmed, retry, project)
        if pending or not api.valid(address, confirmed) then return false end
        pending = {address = address, confirmed = confirmed, retry = retry, project = project or "panopticon", since = api.now()}
        local result = api.prepare(address, pending.project)
        if type(result) == "table" and result.status == "pending" then api.status({address = address}) end
        accept(result)
        return true
    end
    function state.Poll()
        if cleanupAt and api.reset and not pending and api.now() >= cleanupAt and api.idle() then
            local restored, message = api.reset()
            if restored then cleanupAt, cleanupError = nil, nil
            else
                cleanupAt = api.now() + 0.3
                if message ~= "busy" and not cleanupError then
                    cleanupError = true
                    api.error(message or "Не удалось восстановить протокол. Перезапустите Garry’s Mod.")
                end
            end
        end
        if not pending then return end
        if not api.valid(pending.address, pending.confirmed) then state.Cancel() return end
        if api.now() - pending.since > 120 then
            api.cancel()
            state.Cleanup()
            finish("Истекло время подготовки подключения.")
            return
        end
        if pending.id then accept(api.poll(pending.id)) end
    end
    function state.Get() return pending and {address = pending.address} or nil end
    return state
end
return module
