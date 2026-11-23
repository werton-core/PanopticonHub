local Reconnect = {}

-- После подтверждённого входа краткая загрузка не должна стирать адрес.
local ACTIVE_CONFIRMATION = 1.5
local OFFLINE_CONFIRMATION = 0.75
local RETRY_GRACE = 2

function Reconnect.New(validAddress, time)
    local active, joinedAt, offlineAt, pending, ignoredAddress
    local retry
    local state = {}

    function state.Observe(inGame, loading, server)
        local now = time()
        if loading then
            offlineAt = nil
            joinedAt = nil
            pending = nil
            if retry then retry.offlineAt = nil end
            return nil
        end

        if inGame then
            offlineAt = nil
            pending = nil
            if retry then retry.offlineAt = nil end
            local address = server and server.address
            local ignored = ignoredAddress ~= nil and address == ignoredAddress
            if not validAddress(address) or ignored then
                if not (active and active.confirmed) or ignored then
                    active, joinedAt = nil, nil
                end
                return nil
            end

            ignoredAddress = nil
            if not active or active.address ~= address then
                active = { address = address }
                joinedAt = now
            end
            joinedAt = joinedAt or now
            active.name = server.name or active.name
            active.project = server.project or active.project
            if now - joinedAt >= ACTIVE_CONFIRMATION then
                active.confirmed = true
                retry = nil
            end
            return nil
        end

        ignoredAddress = nil
        if retry then
            -- Повтор возможен по клику после выхода из загрузки.
            retry.offlineAt = retry.offlineAt or now
            if now >= retry.readyAt and now - retry.offlineAt >= OFFLINE_CONFIRMATION then
                pending, retry = retry.server, nil
                active, joinedAt, offlineAt = nil, nil, nil
            end
            return pending
        end
        if active then
            if not offlineAt then
                if not active.confirmed then
                    active, joinedAt = nil, nil
                    return nil
                end
                offlineAt = now
            end
            if now - offlineAt >= OFFLINE_CONFIRMATION then
                pending = active
                active, joinedAt, offlineAt = nil, nil, nil
            end
        end
        return pending
    end

    function state.Get()
        return pending
    end

    function state.Clear(address)
        ignoredAddress = address
        active, joinedAt, offlineAt, pending, retry = nil, nil, nil, nil, nil
    end

    function state.Take(expectedAddress, inGame, loading)
        if inGame or loading or not pending or pending.address ~= expectedAddress
            or not validAddress(expectedAddress) then return nil end
        retry = { server = pending, readyAt = time() + RETRY_GRACE }
        pending = nil
        return expectedAddress
    end

    return state
end

return Reconnect
