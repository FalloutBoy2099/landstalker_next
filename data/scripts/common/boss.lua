-- Boss patterns (RoomView::UpdateBoss): what every boss shares. The pattern of each boss is in scripts/behaviors/<boss>.lua, which sets BOSS_AI[kind] =
-- function(e, c); `c` is what the shared part works out for it: c.dx, c.dy (to the player), c.dist, c.speed (cells per second), c.shot (the attack of
-- its projectiles), c.attack (its stats), and the helpers c.face_player(), c.away_step(step), c.toward_step(step), c.rnd(n). The patterns follow the way the
-- bosses play (Ifrit keeps his distance and shoots, Gola fires rings of fireballs, Nole walks up and throws returning axes...), like the C++ engine's.

BOSS_AI = {}

-- the bosses that raise an arm before they shoot (the attack pose starts as the telegraph begins and runs 0.9 s, the shot is in the middle)
local TELEGRAPH = { [124] = true, [133] = true, [160] = true, [162] = true, [165] = true }

-- Returns true if the entity was handled here.
function boss_tick(e)
  local pattern = BOSS_AI[e.kind]
  if not pattern then return false end
  local stats_attack = e.attack or 0
  local dx, dy = P.x - e.x, P.y - e.y
  local dist = math.sqrt(dx * dx + dy * dy)
  local speed = enemy_speed(e)
  e.ai_shots = e.ai_shots or 0
  e.ai_cooldown = (e.ai_cooldown or 0) - DT
  e.ai_state = e.ai_state or "idle"
  e.ai_time = e.ai_time or 0
  e.act_t = e.act_t or -1
  local c = { dx = dx, dy = dy, dist = dist, speed = speed, shot = math.max(1, stats_attack // 2), attack = stats_attack }
  function c.face_player()
    if math.abs(dx) >= math.abs(dy) then e.orientation = dx > 0 and 1 or 3 else e.orientation = dy > 0 and 2 or 0 end
  end
  -- backs off / closes in along the axis that separates them most
  function c.away_step(step)
    if math.abs(dx) >= math.abs(dy) then return enemy_move(e, dx > 0 and -step or step, 0, false) end
    return enemy_move(e, 0, dy > 0 and -step or step, false)
  end
  function c.toward_step(step)
    if math.abs(dx) >= math.abs(dy) then return enemy_move(e, dx > 0 and step or -step, 0, false) end
    return enemy_move(e, 0, dy > 0 and step or -step, false)
  end
  function c.rnd(n) return math.random(0, n - 1) / 100 end
  if e.act_t >= 0 then
    e.act_t = e.act_t + DT
    if e.act_t >= 0.9 then e.act_t = -1 end
  elseif TELEGRAPH[e.kind] and e.ai_cooldown > 0 and e.ai_cooldown < 0.45 then
    e.act_t = 0
  end
  pattern(e, c)
  if e.gravity then e.z = floor_height(math.floor(e.x), math.floor(e.y)) end
  return true
end
