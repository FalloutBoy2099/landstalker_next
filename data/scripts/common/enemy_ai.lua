-- landstalker_next: the enemies' artificial intelligence (a port of RoomView::UpdateEnemies of the C++ engine).
--
-- The original gives every family of enemies its own state machine (code/gamelogic/ai/enemies). They share one idiom: an enemy
-- PATROLS with a small movement script (walk until something is in the way, turn at random), SEES the player inside a rectangle
-- in front of it (6 cells ahead, 2 behind, 3 to each side) and then CHASES along the grid axes. Some families add their own
-- trick: a jumping LUNGE, floating through walls, hopping, bouncing, lying in wait. The exact distances and odds of every
-- family are not ported: the numbers below are common values (like the C++ engine's).
--
-- Called by tick() (60 times a second) for each enemy that is hostile and awake; the Rust side does the damage (combat.rs).
-- Fields used on the entity table: speed_index (the room file's speed 0..5), gravity, hit_flash, ai_alert, ai_state ("idle",
-- "chase", "lunge", "recover"), ai_time, ai_cooldown, act_t (seconds into an attack swing, -1 none), orientation, kind.
-- The AI shared by the creatures of the game; what is particular to one boss is in scripts/behaviors/<boss>.lua (BOSS_AI[kind]).

DT = 1 / 60
local DIR_X = { [0] = 0, 1, 0, -1 }   -- orientation: 0 NE, 1 SE, 2 SW, 3 NW
local DIR_Y = { [0] = -1, 0, 1, 0 }

local function set_of(list)
  local t = {}
  for _, v in ipairs(list) do t[v] = true end
  return t
end

-- the families (AiKindFor)
local LUNGER = set_of { 4, 5, 6, 26, 27, 28, 23, 24, 25, 38, 39, 40, 159, 112 }
local FLOATER = set_of { 32, 33, 34, 149, 150, 151 }
local HOPPER = set_of { 54, 55, 56 }
local BOUNCER = set_of { 60, 61, 62, 129, 130, 131 }
local MIMIC = set_of { 44, 45, 46 }
local ROLLER = set_of { 73, 74, 141, 142 }
local ORC = set_of { 4, 5, 6 }
BOSS = set_of { 112, 124, 133, 157, 159, 160, 162, 165, 167, 170, 171 }
-- grey and spiked balls, boulders: dangerous, but they follow their script like any other object
BALL = set_of { 19, 20, 73, 74 }

local SIGHT_AHEAD, SIGHT_BEHIND, SIGHT_SIDE = 6.0, 2.0, 3.0   -- cells
local SIGHT_NEAR = 2.2        -- closer than this, an enemy always notices the player
local LOSE_DISTANCE = 11.0    -- a chase is given up beyond this
local LOSE_TIME = 5.0         -- ... or after this long without seeing the player
local LUNGE_MIN, LUNGE_MAX, LUNGE_TIME, LUNGE_HEIGHT = 1.3, 3.8, 0.7, 1.2
local LUNGE_COOLDOWN, RECOVER_TIME = 1.6, 0.5
-- Measured on the ROM: an enemy of speed byte s moves s/16 of a cell every frame, 3.75 cells per second per level; the walkers patrol
-- in bursts of one cell (16 frames) with a pause of 64 frames, and chase without stopping.
local ENEMY_BASE_SPEED = 3.75   -- cells per second at speed 1, 3.75 more per level
local PATROL_PAUSE = 64 / 60    -- seconds between two steps of a patrolling walker
-- the bosses keep the slower speeds they had (not measured yet)
local BOSS_BASE_SPEED = 0.6

local function attack_anim_time(e) return BOSS[e.kind] and 0.9 or 0.5 end

function enemy_speed(e)
  -- Miro is fast, the stone warriors slow
  local factor = 1.0
  if e.kind == 159 then factor = 1.5 elseif e.kind == 112 then factor = 1.3 elseif e.kind == 167 or e.kind == 171 then factor = 0.8 end
  if BOSS[e.kind] then return factor * (BOSS_BASE_SPEED + 0.6 * math.max(0, e.speed_index - 1)) end
  return factor * ENEMY_BASE_SPEED * math.max(1, e.speed_index)
end

function sign(v) return v < 0 and -1 or 1 end

-- a chasing step along one grid axis at a time, like the original (no diagonals)
local function chase_step(e, dx, dy, step)
  local adx, ady = math.abs(dx), math.abs(dy)
  local axis = e.move_axis or 0
  if axis == 0 and ady > adx + 0.4 then axis = 1 elseif axis == 1 and adx > ady + 0.4 then axis = 0 end
  local function try_axis(a)
    local d = a == 0 and dx or dy
    if math.abs(d) < 0.02 then return false end
    local m = sign(d) * math.min(step, math.abs(d))
    if a == 0 then return enemy_move(e, m, 0, false) end
    return enemy_move(e, 0, m, false)
  end
  local moved_axis = -1
  if try_axis(axis) then moved_axis = axis elseif try_axis(1 - axis) then moved_axis = 1 - axis end
  if moved_axis < 0 then return false end
  e.move_axis = moved_axis
  if moved_axis == 0 then e.orientation = dx > 0 and 1 or 3 else e.orientation = dy > 0 and 2 or 0 end
  return true
end

-- the odds of starting a lunge, in thousandths per tick (the C++ numbers are per 60 Hz frame)
local function lunge_odds(e)
  if e.kind == 159 then return 70 end
  if ORC[e.kind] then return 12 end
  return 24
end

function ai_tick(e)
  e.ai_state = e.ai_state or "idle"
  e.ai_time = e.ai_time or 0
  e.ai_time2 = e.ai_time2 or 0
  e.ai_cooldown = e.ai_cooldown or 0
  e.ai_alert = e.ai_alert or 0
  e.act_t = e.act_t or -1
  if e.hit_flash > 0 then return end       -- stunned by the blow
  if boss_tick(e) then return end          -- the bosses have patterns of their own
  if e.speed_index <= 0 then return end    -- statues of enemies
  -- enemies swing when the player is within reach
  if e.act_t >= 0 then
    e.act_t = e.act_t + DT
    if e.act_t >= attack_anim_time(e) then e.act_t = -1 end
  elseif math.sqrt((P.x - e.x) ^ 2 + (P.y - e.y) ^ 2) < 1.8 and e.ai_state ~= "lunge" and not (e.kind >= 60 and e.kind <= 62) then
    e.act_t = 0
  end

  local speed = enemy_speed(e)
  local dx, dy = P.x - e.x, P.y - e.y
  local dist = math.sqrt(dx * dx + dy * dy)
  local dir = e.orientation & 3
  e.ai_cooldown = math.max(0, e.ai_cooldown - DT)
  e.ai_alert = math.max(0, e.ai_alert - DT)
  local floater, hopper, bouncer, roller, mimic, lunger = FLOATER[e.kind], HOPPER[e.kind], BOUNCER[e.kind], ROLLER[e.kind], MIMIC[e.kind], LUNGER[e.kind]

  -- does the enemy see the player? (a rectangle in front of it, or very close)
  local ahead = dx * DIR_X[dir] + dy * DIR_Y[dir]
  local side = math.abs(dx * -DIR_Y[dir] + dy * DIR_X[dir])
  local arena_boss = e.kind == 159 or e.kind == 167 or e.kind == 171 or e.kind == 112   -- always know where the player is
  e.ai_blind = math.max(0, (e.ai_blind or 0) - DT)
  local sees = e.ai_blind <= 0 and (dist < SIGHT_NEAR or e.ai_alert > 0 or (arena_boss and dist < 16) or (ahead > -SIGHT_BEHIND and ahead < SIGHT_AHEAD and side < SIGHT_SIDE))

  local state = e.ai_state
  if state == "lunge" then
    -- a jump towards the player along one axis: it keeps going whatever happens
    e.ai_time = e.ai_time + DT
    local t = math.min(1, e.ai_time / LUNGE_TIME)
    enemy_move(e, e.ai_vx * DT, e.ai_vy * DT, false)
    e.z = enemy_floor(e) + math.sin(math.pi * t) * (ORC[e.kind] and 0.7 or LUNGE_HEIGHT)
    if e.ai_time >= LUNGE_TIME then
      e.z = enemy_floor(e)
      e.ai_state = "recover"
      e.ai_time = 0
    end
  elseif state == "recover" then
    e.ai_time = e.ai_time + DT
    if e.gravity then e.z = enemy_floor(e) end
    if e.ai_time >= RECOVER_TIME then
      e.ai_state = "chase"
      e.ai_cooldown = e.kind == 159 and 0.9 or LUNGE_COOLDOWN
      e.ai_time = 0
    end
  elseif state == "chase" then
    e.ai_time = sees and 0 or e.ai_time + DT   -- time since the player was last seen
    if dist > LOSE_DISTANCE or e.ai_time > LOSE_TIME then
      e.ai_state = "idle"
      e.ai_time = 0
      return
    end
    if floater then
      -- floats straight at the player through walls and furniture
      if dist > 0.1 then
        local m = math.min(speed * DT, dist)
        enemy_move(e, dx / dist * m, dy / dist * m, true)
        if math.abs(dx) > math.abs(dy) then e.orientation = dx > 0 and 1 or 3 else e.orientation = dy > 0 and 2 or 0 end
      end
      e.ai_phase = (e.ai_phase or 0) + DT * 3
      e.z = enemy_floor(e) + 1.0 + 0.25 * math.sin(e.ai_phase)
      return
    end
    if hopper then
      -- short hops towards the player, a pause between two of them
      e.ai_time2 = e.ai_time2 + DT
      local hop, pause = 0.55, 0.8
      local cycle = e.ai_time2 % (hop + pause)
      if cycle < hop and dist > 0.6 then
        chase_step(e, dx, dy, speed * 0.4 * DT)
        e.z = enemy_floor(e) + math.sin(math.pi * cycle / hop) * 0.8
      elseif e.gravity then
        e.z = enemy_floor(e)
      end
      return
    end
    -- a chase that goes nowhere (a wall between the two): after a while the enemy gives up and wanders again
    local px0, py0 = e.x, e.y
    if dist > 0.3 then chase_step(e, dx, dy, speed * DT) end
    if math.abs(e.x - px0) + math.abs(e.y - py0) < 1e-4 then e.ai_stuck = (e.ai_stuck or 0) + DT else e.ai_stuck = 0 end
    if e.ai_stuck > 1.0 then
      e.ai_stuck, e.ai_state, e.ai_time, e.ai_alert, e.ai_blind = 0, "idle", 0, 0, 2.5
      e.orientation = math.random(0, 3)
      return
    end
    if e.gravity then e.z = enemy_floor(e) end
    local max_reach = ORC[e.kind] and 2.3 or LUNGE_MAX
    if (lunger or mimic) and e.ai_cooldown <= 0 and dist > LUNGE_MIN and dist < max_reach and math.random(0, 1999) < lunge_odds(e) then
      -- jump at the player along the axis that separates them most
      local adx, ady = math.abs(dx), math.abs(dy)
      local reach = math.min(dist + 0.4, ORC[e.kind] and 1.9 or 3.2)   -- orcs only hop a short way
      local sp = reach / LUNGE_TIME
      if adx >= ady then
        e.ai_vx, e.ai_vy = sign(dx) * sp, 0
        e.orientation = dx > 0 and 1 or 3
      else
        e.ai_vx, e.ai_vy = 0, sign(dy) * sp
        e.orientation = dy > 0 and 2 or 0
      end
      e.ai_state = "lunge"
      e.ai_time = 0
    end
  else   -- idle
    if floater and not sees then
      e.ai_phase = (e.ai_phase or 0) + DT * 3
      e.z = enemy_floor(e) + 1.0 + 0.25 * math.sin(e.ai_phase)   -- bobs on the spot
      return
    end
    if sees and not bouncer and not roller and (not mimic or dist < 3.0) and (not hopper or dist < 5.0) then
      e.ai_state = "chase"
      e.ai_time = 0
      e.ai_time2 = 0
      if lunger then e.ai_cooldown = math.max(e.ai_cooldown, 0.4) end
      return
    end
    if mimic or hopper then
      if e.gravity then e.z = enemy_floor(e) end
      return   -- waits where it is
    end
    -- patrol script: walk along the facing direction until blocked, then turn at random; now and then stop for a moment.
    -- Rollers and bubbles never stop.
    local patrol_speed = speed
    e.ai_time = e.ai_time - DT
    if e.ai_time <= 0 then
      local walker = not (floater or hopper or bouncer or roller or mimic) or lunger
      -- a walker takes one cell, then waits; the others keep going
      local pause = walker and e.ai_walking
      e.ai_walking = not pause
      e.ai_time = pause and PATROL_PAUSE or (walker and 1.0 / patrol_speed or 1.0 + math.random(0, 199) / 100)
      if not pause and not e.ai_blocked and math.random(0, 99) < 50 then e.orientation = math.random(0, 3) end
      e.ai_blocked = false
    end
    -- stuck for good (inside something): back to where it started
    e.home_x, e.home_y = e.home_x or e.x, e.home_y or e.y
    if math.abs(e.x - (e.last_x or e.x)) + math.abs(e.y - (e.last_y or e.y)) < 1e-4 then e.ai_idle_stuck = (e.ai_idle_stuck or 0) + DT else e.ai_idle_stuck = 0 end
    e.last_x, e.last_y = e.x, e.y
    if e.ai_idle_stuck > 3.0 and not e.ai_pause_now then
      e.x, e.y, e.ai_idle_stuck = e.home_x, e.home_y, 0
    end
    if e.ai_walking then
      local d = e.orientation & 3
      if not enemy_move(e, DIR_X[d] * patrol_speed * DT, DIR_Y[d] * patrol_speed * DT, false) then
        -- blocked: a roller turns back, the others turn at random
        e.orientation = roller and (d + 2) & 3 or (d + math.random(1, 3)) & 3   -- never the same way again
        e.ai_blocked = true
        if not roller then e.ai_time = math.min(e.ai_time, 0.05) end
      end
    end
    if e.gravity then e.z = enemy_floor(e) end
  end
end

-- What the enemy shows: its walk (0), hurt (1), attacking (2), spinning attack (3); and how far through the pose it is.
function enemy_pose(e)
  if e.dying or e.hit_flash > 0 then
    e.pose, e.pose_p = 1, 0
  elseif e.act_t and e.act_t >= 0 then
    e.pose, e.pose_p = 2, e.act_t / attack_anim_time(e)
  elseif e.ai_state == "lunge" then
    local loop = e.kind == 157 or e.kind == 170   -- spinners keep spinning
    e.pose = loop and 3 or 2
    e.pose_p = loop and e.ai_time * 2 or e.ai_time / LUNGE_TIME
  else
    e.pose = 0
  end
end

-- A dying enemy flickers (the game draws it so), then becomes what it leaves: an item box (one chance in N, see
-- data/enemies.yaml) or a bag with its gold; or nothing.
function dying_tick(e)
  e.die_t = e.die_t + DT
  if e.die_t < (e.die_dur or 0.45) then return end
  local gold = e.pending_gold or 0
  local d = DROPS[e.kind]
  local item = -1
  if d and (d.one_in <= 1 or math.random(0, d.one_in - 1) == 0) then item = d.item end
  e.dying = false
  e.hostile = false
  e.enemy = false
  e.killable = false
  e.dead = true
  if item < 0 and gold <= 0 then
    e.visible, e.base_visible = false, false
    return
  end
  -- the same table becomes the drop: it falls to the floor and waits to be picked up
  e.kind = item >= 0 and 96 or 10
  e.hbase, e.hheight = kind_box(e.kind)
  e.solid = false
  e.gravity = true
  e.falls, e.fall_v = true, 0
  e.speed_index = 0
  e.behaviour = 0
  e.co = nil
  e.orientation = 2
  e.pickup_gold = item >= 0 and 0 or gold
  e.pickup_item = item
  e.pickup_age = 0
  e.visible, e.base_visible = true, true
end

