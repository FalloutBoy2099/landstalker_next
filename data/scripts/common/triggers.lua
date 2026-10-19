-- Triggers (WaitForCondition): what `wait_trigger(e, n)` waits for.

---------------------------------------------------------------------------------------------------------------
-- triggers (WaitForCondition): the routines the original needs a few special cases for, the rest are programs
-- (data/triggers.yaml) run by the game
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

