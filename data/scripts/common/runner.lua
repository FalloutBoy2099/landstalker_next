-- The script runner: the flag rules, start_room (one coroutine per entity) and tick (60 per second).

local DEFAULT_SPEED = 1

---------------------------------------------------------------------------------------------------------------
-- the script runner
---------------------------------------------------------------------------------------------------------------


-- Who is shown or hidden by the story flags (RoomView::ApplyFlagRules). Only the entities a rule talks about are touched.
function apply_flag_rules()
  local hidden, has_onetime, onetime_shows, touched = {}, {}, {}, {}
  local n = #ents
  for _, r in ipairs(flag_rules) do
    local i = r.entity + 1
    if r.kind == "hide" then
      touched[i] = true
      -- (checked on the original: the entity is shown while the flag is in the state `when_set` says, hidden otherwise: the bears of room 425 are only there once
      -- the flag 0 is set and the flag 18 is not, the bear of 424 leaves when the flag 16 is set)
      if flag(r.flag) ~= r.when_set then hidden[i] = true end
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
  apply_flag_rules()   -- who is there when the room opens depends on the story flags
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

