-- Drives EmuHawk for the comparisons with the port. Everything comes from environment variables (see run.py):
--   LS_HAWK_OUT    folder for the results
--   LS_HAWK_INPUT  "frame:+Button;frame:-Button;..."  (Up Down Left Right A B C Start), pressed at those frames
--   LS_HAWK_SHOT   "frame,frame,..."  screenshots (shot_<frame>.png)
--   LS_HAWK_LOG    "Symbol,Symbol:w,..."  RAM symbols (ram.lua) written every LS_HAWK_EVERY frames (default 1) to log.csv; ":w" = word, ":l" = long
--   LS_HAWK_END    last frame (default 600)
--   LS_HAWK_STATE  savestate to load at the start; LS_HAWK_SAVE "frame=name,..." savestates written (state_<name>)
local here = os.getenv("LS_HAWK_DIR") or ""
local ram = dofile(here .. "ram.lua")
local out = os.getenv("LS_HAWK_OUT") or "."
local function env(n, d) local v = os.getenv(n); if v == nil or v == "" then return d end return v end

local bus = "M68K BUS"
local function read(sym, size)
  if sym == "vsramA" then return memory.read_u16_be(0, "VSRAM") end
  if sym == "vsramB" then return memory.read_u16_be(2, "VSRAM") end
  local a = ram[sym] or tonumber(sym)
  if size == "w" then return memory.read_u16_be(a, bus) elseif size == "l" then return memory.read_u32_be(a, bus) end
  return memory.read_u8(a, bus)
end

-- input
local steps = {}
for s in env("LS_HAWK_INPUT", ""):gmatch("[^;]+") do
  local f, sign, key = s:match("^(%d+):([%+%-])(%w+)$")
  if f then steps[#steps + 1] = { tonumber(f), sign == "+", key } end
end
table.sort(steps, function(a, b) return a[1] < b[1] end)
local held, nstep = {}, 1

local shots = {}
for f in env("LS_HAWK_SHOT", ""):gmatch("%d+") do shots[tonumber(f)] = true end
local saves = {}
for f, n in env("LS_HAWK_SAVE", ""):gmatch("(%d+)=([%w_]+)") do saves[tonumber(f)] = n end

local cols = {}
for s in env("LS_HAWK_LOG", ""):gmatch("[^,]+") do
  local name, size = s:match("^([%w_]+):?(%a?)$")
  cols[#cols + 1] = { name, size }
end
local every = tonumber(env("LS_HAWK_EVERY", "1"))
local log
if #cols > 0 then
  log = io.open(out .. "/log.csv", "w")
  local h = { "frame" }
  for _, c in ipairs(cols) do h[#h + 1] = c[1] end
  log:write(table.concat(h, ",") .. "\n")
end

if env("LS_HAWK_STATE", "") ~= "" then savestate.load(env("LS_HAWK_STATE")) end
local last = tonumber(env("LS_HAWK_END", "600"))
for frame = 0, last do
  while steps[nstep] and steps[nstep][1] <= frame do
    held[steps[nstep][3]] = steps[nstep][2] or nil
    nstep = nstep + 1
  end
  local pad = {}
  for k in pairs(held) do pad["P1 " .. k] = true end
  joypad.set(pad)
  if log and frame % every == 0 then
    local row = { frame }
    for _, c in ipairs(cols) do row[#row + 1] = read(c[1], c[2]) end
    log:write(table.concat(row, ",") .. "\n")
  end
  if shots[frame] then client.screenshot(out .. "/shot_" .. frame .. ".png") end
  if saves[frame] then savestate.save(out .. "/state_" .. saves[frame] .. ".State") end
  emu.frameadvance()
end
if log then log:close() end
client.exit()
