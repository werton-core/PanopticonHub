local reconnect = ... or dofile("game-ui/source/lua/menu/panopticon_reconnect.lua")
local now = 0
local function reset()
    now = 0
    return reconnect.New(function(address) return address == "example:27015" end, function() return now end)
end
local server = { address = "example:27015", name = "Локальный тест" }
local function step(state, at, inGame, loading, current)
    now = at
    return state.Observe(inGame, loading, current)
end

local state = reset()
assert(not step(state, 0, true, false, server))
assert(not step(state, 1, false, true))
assert(not step(state, 2, false, false))
assert(not step(state, 3, false, false))

state = reset()
assert(not step(state, 0, true, false, server))
assert(not step(state, 2, true, false, server))
assert(not step(state, 3, true, false))
assert(not step(state, 4, false, true))
assert(not step(state, 5, false, false))
assert(step(state, 6, false, false).address == server.address)
assert(not state.Take("other:27015", false, false))
assert(not state.Take(server.address, true, false))
assert(state.Take(server.address, false, false) == server.address)
assert(not state.Take(server.address, false, false))
assert(not step(state, 6.5, false, true))
assert(not step(state, 7.5, false, false))
assert(step(state, 8.5, false, false).address == server.address)
state.Clear(server.address)
assert(not step(state, 9, false, false))
assert(not state.Take(server.address, false, false))
