-- First probe: boot the ROM, list the memory domains, advance some frames, take a screenshot, quit.
local out = os.getenv("LS_HAWK_OUT") or "."
local f = io.open(out .. "/probe.txt", "w")
f:write("system=" .. emu.getsystemid() .. "\n")
f:write("core=" .. tostring(client.get_lua_engine and client.get_lua_engine() or "?") .. "\n")
for _, d in ipairs(memory.getmemorydomainlist()) do
  f:write("domain " .. d .. " size=" .. memory.getmemorydomainsize(d) .. "\n")
end
for i = 1, 600 do emu.frameadvance() end
f:write("frame=" .. emu.framecount() .. "\n")
f:close()
client.screenshot(out .. "/probe.png")
client.exit()
