-- Timing and movement commands of the behaviour scripts (one Lua function per command of the original bytecode, with the timing of the C++ engine's
-- RoomView::BehaviourTick). A script is a coroutine run once per script tick (60 per second); `yield` ends the tick, the commands that carry on do not.

local yield = coroutine.yield

-- ends the tick of the script of entity e
function tick_wait(e)
  e.guard = 0
  yield()
end

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

