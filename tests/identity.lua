local identity = ... or dofile("game-ui/source/lua/menu/panopticon_identity.lua")
local now, loading, active, confirmed = 0, false, false, nil
local joins, cancels, errors, statuses = {}, 0, {}, {}
local prepared, polled = {status="pending",id=1}, {status="pending",id=1}
local state = identity.New({
    now=function() return now end,
    valid=function(address, confirmation)
        if loading or address ~= "185.71.66.223:27015" then return false end
        return active and confirmation == confirmed or not active and confirmation == nil
    end,
    prepare=function() return prepared end, poll=function() return polled end,
    cancel=function() cancels=cancels+1 end,
    error=function(message) errors[#errors+1]=message end,
    status=function(value) statuses[#statuses+1]=value or false end,
    join=function(address,retry) joins[#joins+1]={address=address,retry=retry} end,
})
local address="185.71.66.223:27015"
assert(not state.Start("invalid"))
assert(state.Start(address))
assert(state.Busy() and not state.Start(address))
now=6 -- Подготовка дольше прежнего двухсекундного окна reconnect.
state.Poll(); assert(#joins==0)
polled={status="ready",id=1}; state.Poll(); state.Poll()
assert(#joins==1 and not state.Busy())
assert(state.Start(address)); state.Cancel(); state.Poll()
assert(#joins==1 and cancels==1)
prepared={status="error",message="Отказ реестра"}; state.Start(address)
assert(#errors==1 and #joins==1 and not state.Busy())
prepared={status="pending",id=2}; polled={status="ready",id=2}
state.Start(address); loading=true; state.Poll(); loading=false
assert(#joins==1 and cancels==2)
active=true; confirmed="old-server:27015"
assert(not state.Start(address))
assert(state.Start(address,confirmed)); confirmed="another-server:27015"; state.Poll()
assert(#joins==1 and cancels==3)
active=false; confirmed=nil
state.Start(address,nil,true); state.Poll()
assert(#joins==2 and joins[2].retry)
prepared={status="ready"}; state.Start(address)
assert(#joins==3 and not state.Busy()) -- Выключенный режим не задерживает штатный Join.
prepared={status="pending",id=3}; state.Start(address); now=200; state.Poll()
assert(#joins==3 and cancels==4 and #errors==2)
print("Идентификатор: подготовка, отмена, отказ, тайм-аут, смена сеанса и повтор — OK")

local cleanupCalls, cleanupErrors, idle = 0, 0, false
local cleanup = identity.New({
    now=function() return now end, valid=function() return true end,
    prepare=function() return {status="ready"} end, join=function() end,
    status=function() end, error=function() cleanupErrors=cleanupErrors+1 end,
    idle=function() return idle end,
    reset=function() cleanupCalls=cleanupCalls+1; if cleanupCalls<=2 then return false,"busy" end return true end,
})
now=0; cleanup.Start(address); now=5; cleanup.Poll(); assert(cleanupCalls==0)
idle=true; cleanup.Poll(); now=5.4; cleanup.Poll(); now=5.8; cleanup.Poll(); now=6.2; cleanup.Poll()
assert(cleanupCalls==3 and cleanupErrors==0) -- Отмена I/O не должна навсегда пропускать откат.
