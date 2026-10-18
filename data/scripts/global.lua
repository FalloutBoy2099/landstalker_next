-- landstalker_next: scripting standard library.
--
-- The sprite behaviours of the original game (bytecode) are translated into Lua by tools/behaviours_to_lua.py
-- (data/scripts/generated/behaviours.lua); every command of the bytecode is one of the functions below, with the same
-- timing as the C++ engine (RoomView::BehaviourTick): a script runs as a coroutine, one step per script tick (60 per
-- second); `yield` ends the tick, the commands that "carry on" do not.
--
-- Entities (`ents`, filled by the game when a room is loaded) are plain tables:
--   i (0-based index in the room), kind, x, y, z (cells / height units), orientation (0 NE, 1 SE, 2 SW, 3 NW), speed (1/16
--   cell per tick), visible, base_visible, hostile, gravity, solid, falls, fall_v, behaviour, hbase, hheight, killable.
-- The game provides: floor_height(cx, cy), step(e, noclip), overlaps_player(e), trigger_program(n), flag(n), set_flag(n),
-- clear_flag(n), sfx(n), and the table P (the player: x, y, z, facing, grounded, support).
--
-- Hand-made scripts go in data/scripts/objects, critters, rooms (see scripts/README.md): `script(e)` of an entity table
-- replaces its generated behaviour.

local yield = coroutine.yield

local function tick_wait(e)
  e.guard = 0
  yield()
end

local DEFAULT_SPEED = 1

---------------------------------------------------------------------------------------------------------------
-- timing commands (each is documented with the bytecode command it implements)
---------------------------------------------------------------------------------------------------------------

local function count_of(n) return n == 0 and 256 or n end

-- 0x00 Pause: `n` ticks
function pause(e, n)
  for _ = 1, count_of(n) do tick_wait(e) end
end

-- 0x01 MoveTimed: moves `n` ticks, blocked or not
function move_timed(e, n, noclip)
  n = count_of(n)
  for i = 1, n do
    step(e, noclip)
    if i < n then tick_wait(e) end
  end
end

-- 0x11 MoveRelative: `n` successful steps (waits while blocked), then ends the tick
function move_wait(e, n)
  local count = count_of(n)
  while true do
    if not step(e, false) then
      tick_wait(e)
    else
      count = count - 1
      if count > 0 then tick_wait(e) else break end
    end
  end
  tick_wait(e)
end

-- 0x4F MoveNoClip: walks through anything
function move_noclip(e, n)
  n = count_of(n)
  local dx, dy = ({[0] = 0, 1, 0, -1})[e.orientation & 3], ({[0] = -1, 0, 1, 0})[e.orientation & 3]
  for i = 1, n do
    e.x = e.x + dx * e.speed / 16
    e.y = e.y + dy * e.speed / 16
    if i < n then tick_wait(e) end
  end
end

-- 0x30 / 0x31 MoveUpRelative / MoveDownRelative: `n` ticks of vertical movement (speed/16 per tick); a tick where something is
-- in the way does not count (the platform waits, then goes on)
function move_up_relative(e, n)
  local count, d = count_of(n), e.speed / 16
  while count > 0 do
    e.z = e.z + d
    if hits(e) then e.z = e.z - d else count = count - 1 end
    if count > 0 then tick_wait(e) end
  end
end
function move_down_relative(e, n)
  local count, d = count_of(n), e.speed / 16
  while count > 0 do
    e.z = e.z - d
    if hits(e) then e.z = e.z + d else count = count - 1 end
    if count > 0 then tick_wait(e) end
  end
end

-- 0x3A / 0x3B: elevators and platforms, 1/16 unit per tick at speed 1
function move_up_timed(e, n)
  for _ = 1, count_of(n) do e.z = e.z + e.speed / 16; tick_wait(e) end
end
function move_down_timed(e, n)
  for _ = 1, count_of(n) do e.z = e.z - e.speed / 16; tick_wait(e) end
end

-- 0x3C / 0x3D: up / down to a height (1/16 units)
function move_up_to(e, t)
  local target, d = t / 16, e.speed / 16
  repeat e.z = math.min(target, e.z + d); tick_wait(e) until e.z == target
end
function move_down_to(e, t)
  local target, d = t / 16, e.speed / 16
  repeat e.z = math.max(target, e.z - d); tick_wait(e) until e.z == target
end

-- 0x13 MoveUntilCollision
function move_until_collision(e)
  while step(e) do tick_wait(e) end
end

-- 0x2A MoveRandomTime(range, max)
function move_random_time(e, range, max)
  local count = math.max(1, max - math.random(0, math.max(1, range) - 1) + 1)
  for i = 1, count do
    step(e)
    if i < count then tick_wait(e) end
  end
end

-- 0x40 MoveDownUntilCollision: down to the floor under it
function move_down_until_collision(e)
  local half = e.hbase
  local floor_h = 0
  for cx = math.floor(e.x - half + 0.01), math.floor(e.x + half - 0.01) do
    for cy = math.floor(e.y - half + 0.01), math.floor(e.y + half - 0.01) do
      local h = floor_height(cx, cy)
      if h <= e.z + 1e-6 and h > floor_h then floor_h = h end
    end
  end
  local d = e.speed / 16
  while true do
    if e.z - d <= floor_h then e.z = floor_h; return end
    e.z = e.z - d
    tick_wait(e)
  end
end

---------------------------------------------------------------------------------------------------------------
-- orientation, visibility, flags
---------------------------------------------------------------------------------------------------------------

function turn(e, dir) e.orientation = dir & 3 end
function turn_wait(e, dir) turn(e, dir); tick_wait(e) end

-- 0x45: appears (a gate closing behind the player) once the player is out of its box
function show_when_clear(e)
  if not e.visible then
    while overlaps_player(e) do tick_wait(e) end
  end
  e.visible = true
end

function wait_flag(e, f, want)
  while flag(f) ~= want do tick_wait(e) end
end

function activate_switch(e)
  turn(e, 2)
  sfx(0x74)
end

function move_to(e, x, y)
  e.x, e.y = x + 0.5, y + 0.5
  if e.gravity then e.z = floor_height(x, y) end
end

function jump_guard(e)
  e.guard = (e.guard or 0) + 1
  if e.guard >= 32 then tick_wait(e) end
end

function repeat_begin(e, n)
  if n ~= 0 then e.repeat_count = n end
end
function repeat_next(e)
  e.repeat_count = (e.repeat_count or 1) - 1
  return e.repeat_count > 0
end

-- not ported yet (the game does nothing, like the C++ engine for most of them)
-- PlaybackInput(n): the player follows a recorded input (the scenes of the story)
function playback(n) request_playback(n) end
-- EB_Pause4s: PlaybackInput(9), the player cannot move for 254 frames (the raft that carries him)
function pause_player() request_pause() end
function rotate_player(dir) P.turn_to = dir & 3 end
-- TriggerTileSwap(n): sets the flag that makes the swap n happen (once)
function tile_swap(n)
  for _, sf in ipairs(swap_flags) do
    if sf.trigger == n and not sf.locked and not flag(sf.flag) then
      set_flag(sf.flag)
      break
    end
  end
end
-- TriggerCutscene(n) (the command 0x4E, n < 256) / StartHiCutscene(p) (0x64, n = 0x100 + p): the story runs an action. A scene that waited for a
-- flag plays once: entering the room again must not start it again (its scripts restart with the room), remembered as a private flag.
function cutscene(e, n)
  if n < 0x100 and WAITS_FIRST[e.behaviour] then
    local played = 4096 + n
    if flag(played) then
      e.co = nil
      while true do coroutine.yield() end
    end
    set_flag(played)
  end
  request_cutscene(e.i, n)
end

---------------------------------------------------------------------------------------------------------------
-- triggers (WaitForCondition): the routines the original needs a few special cases for, the rest are programs
-- (data/text/triggers.txt) run by the game
---------------------------------------------------------------------------------------------------------------

-- somebody (the player, or an enemy) stands on entity e
local function someone_on(e)
  if P.support == e.i then return true end
  local top = e.z + e.hheight
  for _, o in ipairs(ents) do
    -- an enemy, or something that was put there (a crate on a switch)
    if o ~= e and o.visible and not o.carried and not o.dying and (o.hostile or o.pickable or o.gravity) then
      if math.abs(o.x - e.x) < e.hbase and math.abs(o.y - e.y) < e.hbase and math.abs(o.z - top) < 0.15 then return true end
    end
  end
  return false
end

local function killable_enemy_exists()
  for _, o in ipairs(ents) do
    if o.hostile and not o.dying and o.killable then return true end
  end
  return false
end

function wait_trigger(e, n)
  while not trigger(e, n) do tick_wait(e) end
end

function trigger(e, n)
  if n == 0 then
    -- first dungeon: boulder trap. Once one of the six blocks falls, all of them fall.
    local any = false
    for i = 1, math.min(6, #ents) do any = any or ents[i].falls end
    if not any then return false end
    for i = 1, math.min(6, #ents) do
      if not ents[i].falls then ents[i].falls = true; ents[i].fall_v = 0 end
    end
    sfx(0x59)
    return true
  elseif n == 1 then
    return someone_on(e)
  elseif n == 6 or n == 0x1E then
    return not someone_on(e)
  elseif n == 2 then
    return not killable_enemy_exists()
  elseif n == 0x62 then
    return killable_enemy_exists()
  elseif n == 3 or n == 4 or n == 5 then
    local o = ents[n + 3]   -- the 6th / 7th / 8th entity (1-based in Lua)
    local hostile = o and o.hostile and not o.dying
    return not hostile or flag(0x0A * 8 + 3)
  end
  return trigger_program(n)
end

---------------------------------------------------------------------------------------------------------------
-- the script runner
---------------------------------------------------------------------------------------------------------------

dofile_relative("generated/behaviours.lua")
dofile_relative("ai.lua")

-- Who is shown or hidden by the story flags (RoomView::ApplyFlagRules). Only the entities a rule talks about are touched.
function apply_flag_rules()
  local hidden, has_onetime, onetime_shows, touched = {}, {}, {}, {}
  local n = #ents
  for _, r in ipairs(flag_rules) do
    local i = r.entity + 1
    if r.kind == "hide" then
      touched[i] = true
      if flag(r.flag) == r.when_set then hidden[i] = true end
    elseif r.kind == "onetime" then
      touched[i] = true
      has_onetime[i] = true
      if flag(r.show_flag) == r.show_set and not (flag(r.hide_flag) == r.hide_set) then onetime_shows[i] = true end
    elseif r.kind == "hide_from" then
      for k = i, n do
        touched[k] = true
        if flag(r.flag) then hidden[k] = true end
      end
    end
  end
  for i in pairs(touched) do
    local e = ents[i]
    if e then
      local v
      if has_onetime[i] then v = onetime_shows[i] or false else v = e.base_visible end
      if hidden[i] then v = false end
      e.visible = v
    end
  end
end

-- Called by the game once the room's entities are in `ents`: starts every script.
function start_room()
  take_rules_dirty()
  for _, e in ipairs(ents) do
    e.speed = e.speed or DEFAULT_SPEED
    e.guard = 0
    -- an enemy whose script waits for a story event (an ambush, a boss) sleeps until the script turns it hostile
    if e.hostile and e.behaviour ~= 0 and SCRIPTED[e.behaviour] and not BALL[e.kind] then e.dormant = true end
    if os.getenv("LS_WAKE") then e.dormant = false; if e.hostile then e.visible = true end end   -- test aid: the sleeping enemies are awake
    local f = e.script_fn or B[e.behaviour]
    if f and e.behaviour ~= 0 then
      e.co = coroutine.create(f)
      e.co_started = false
    else
      e.co = nil
    end
  end
end

-- One script tick of every entity.
local trace = tonumber(os.getenv("LS_TRACE_ENT") or "")
local ticks = 0
function tick()
  ticks = ticks + 1
  if take_rules_dirty() then apply_flag_rules() end
  if trace and ticks % 15 == 0 and ents[trace + 1] then
    local e = ents[trace + 1]
    log(string.format("entity %d: x %.2f y %.2f z %.2f orientation %d speed %d | player x %.2f y %.2f z %.2f", trace, e.x, e.y, e.z, e.orientation, e.speed, P.x, P.y, P.z))
  end
  for _, e in ipairs(ents) do
    local px, py = e.x, e.y
    local co = e.co
    if e.proj_attack > 0 then
      projectile_tick(e)
    elseif e.dying then
      dying_tick(e)
    elseif e.hostile and not e.dormant and not BALL[e.kind] then
      -- awake enemies are driven by the AI, not by their script (RoomView::UpdateBehaviours)
      if e.enemy and e.visible and not e.dead then ai_tick(e) end
    elseif co and coroutine.status(co) ~= "dead" then
      local ok, err
      if not e.co_started then
        e.co_started = true
        ok, err = coroutine.resume(co, e)
      else
        ok, err = coroutine.resume(co)
      end
      if not ok then
        log("script error on entity " .. e.i .. " (behaviour " .. tostring(e.behaviour) .. "): " .. tostring(err))
        e.co = nil
      end
    end
    if e.hit_flash > 0 then e.hit_flash = math.max(0, e.hit_flash - 1 / 60) end
    if e.enemy then enemy_pose(e) end
    if e.pickup_gold > 0 or e.pickup_item >= 0 then
      -- a drop vanishes after a while
      e.pickup_age = (e.pickup_age or 0) + 1 / 60
      if e.pickup_age > 25 then e.visible, e.base_visible, e.pickup_gold, e.pickup_item = false, false, 0, -1 end
    end
    e.moved = e.x ~= px or e.y ~= py
  end
  falling_tick()
end

-- Entities whose gravity was switched on (0x38) drop until they rest on the floor or on another object.
function falling_tick()
  for _, e in ipairs(ents) do
    if e.falls then
      local floor_h = floor_height(math.floor(e.x), math.floor(e.y))
      e.fall_v = (e.fall_v or 0) + 0.04
      e.z = e.z - e.fall_v
      if e.z <= floor_h then
        e.z = floor_h
        e.falls = false
        e.fall_v = 0
      end
    end
  end
end
