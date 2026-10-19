-- Warps the debug-enabled ROM to a room: RAM g_RmNum1 = room - 1, then P2 Start (the ROM's debug "next room"). Env: LS_WARP_ROOM, LS_HAWK_OUT, LS_HAWK_END, LS_HAWK_STATE, LS_HAWK_LOG (symbols as in hawk.lua), LS_HAWK_INPUT (P1 buttons after the warp, frame numbers counted from the warp)
local here = os.getenv("LS_HAWK_DIR") or ""
local ram = dofile(here .. "ram.lua")
local out = os.getenv("LS_HAWK_OUT") or "."
local function env(n, d) local v = os.getenv(n); if v == nil or v == "" then return d end return v end
local bus = "M68K BUS"
local function read(sym)
  local size = sym:match(":(%a)$"); local name = sym:gsub(":%a$", "")
  if name == "vsramA" then return memory.read_u16_be(0, "VSRAM") end
  if name == "vsramB" then return memory.read_u16_be(2, "VSRAM") end
  local a = ram[name] or tonumber(name)
  if size == "w" then return memory.read_u16_be(a, bus) elseif size == "l" then return memory.read_u32_be(a, bus) end
  return memory.read_u8(a, bus)
end
savestate.load(env("LS_HAWK_STATE"))
for i = 1, 5 do emu.frameadvance() end
-- LS_PRE_POKE="0xFF1002=4" writes RAM bytes BEFORE the warp (story flags: the room reads them as it loads)
for sym, size, val in env("LS_PRE_POKE", ""):gmatch("([%w_]+)(:?%a?)=(%d+)") do
  if size == ":w" then memory.write_u16_be(ram[sym] or tonumber(sym), tonumber(val), bus)
  else memory.write_u8(ram[sym] or tonumber(sym), tonumber(val), bus) end
end
local room = tonumber(env("LS_WARP_ROOM", "1"))
memory.write_u16_be(ram.g_RmNum1, room - 1, bus)
for f = 0, 120 do
  joypad.set({ ["P2 Start"] = (f >= 2 and f < 6) })
  emu.frameadvance()
end
-- LS_POKE="Player_Y=22,Player_X=24" writes bytes into the RAM once the room is loaded (to put Nigel where the story would have)
-- Name=value writes a byte, Name:w=value a word
for sym, size, val in env("LS_POKE", ""):gmatch("([%w_]+)(:?%a?)=(%d+)") do
  if size == ":w" then memory.write_u16_be(ram[sym] or tonumber(sym), tonumber(val), bus)
  else memory.write_u8(ram[sym] or tonumber(sym), tonumber(val), bus) end
end
local steps = {}
for s in env("LS_HAWK_INPUT", ""):gmatch("[^;]+") do
  local f, sign, key = s:match("^(%d+):([%+%-])(%w+)$")
  if f then steps[#steps + 1] = { tonumber(f), sign == "+", key } end
end
local held, nstep = {}, 1
local cols = {}
for s in env("LS_HAWK_LOG", ""):gmatch("[^,]+") do cols[#cols + 1] = s end
local log = io.open(out .. "/log.csv", "w")
log:write("frame," .. table.concat(cols, ",") .. "\n")
local shots = {}
for f in env("LS_HAWK_SHOT", ""):gmatch("%d+") do shots[tonumber(f)] = true end
for frame = 0, tonumber(env("LS_HAWK_END", "600")) do
  while steps[nstep] and steps[nstep][1] <= frame do held[steps[nstep][3]] = steps[nstep][2] or nil; nstep = nstep + 1 end
  local pad = {}
  for k in pairs(held) do pad["P1 " .. k] = true end
  joypad.set(pad)
  local row = { frame }
  for _, c in ipairs(cols) do row[#row + 1] = read(c) end
  log:write(table.concat(row, ",") .. "\n")
  if shots[frame] then client.screenshot(out .. "/shot_" .. frame .. ".png") end
  emu.frameadvance()
end
log:close()
client.exit()
