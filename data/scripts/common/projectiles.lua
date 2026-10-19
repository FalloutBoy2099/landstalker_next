-- Projectiles of the bosses (RoomView::SpawnProjectile, UpdateProjectiles): fireballs, axes and boomerangs fly straight along a grid axis and hurt the
-- player on contact (combat.rs). They live in the spare entity rows of the room (`slot`, see scripting.rs), so they are drawn like any entity.

-- A projectile of entity type `kind` leaves `owner` with the velocity (vx, vy) cells per second, `lateral` cells to the side.
function spawn_projectile(owner, kind, vx, vy, attack, returns, lateral)
  local slot
  for i = #ents, 1, -1 do
    local s = ents[i]
    if s.slot and s.proj_attack == 0 then slot = s break end
  end
  if not slot then return end
  local len = math.sqrt(vx * vx + vy * vy)
  local ux, uy = 0, 0
  if len > 0 then ux, uy = vx / len, vy / len end
  slot.kind = kind
  slot.hbase, slot.hheight = kind_box(kind)
  slot.x = owner.x + ux * (owner.hbase + 0.3) - uy * lateral
  slot.y = owner.y + uy * (owner.hbase + 0.3) + ux * lateral
  slot.z = owner.z + owner.hheight / 2
  slot.vx, slot.vy = vx, vy
  slot.proj_attack = attack
  slot.proj_returns = returns
  slot.proj_owner = owner.i
  slot.proj_age = 0
  if math.abs(vx) > math.abs(vy) then slot.orientation = vx > 0 and 1 or 3 else slot.orientation = vy > 0 and 2 or 0 end
  slot.visible = true
end

-- aims along the grid axis that separates the boss from the player
-- `sound`: what is heard (the fireball by default; the throws of Zak, of Nole and of Gola's aimed shot are slashes: 0x77, 0x78)
function boss_fire(e, kind, speed, attack, count, returns, sound)
  local dx, dy = P.x - e.x, P.y - e.y
  local vx, vy = 0, 0
  if math.abs(dx) >= math.abs(dy) then vx = (dx >= 0 and 1 or -1) * speed else vy = (dy >= 0 and 1 or -1) * speed end
  for k = 0, count - 1 do
    spawn_projectile(e, kind, vx, vy, attack, returns, (k - (count - 1) / 2) * 0.9)
  end
  sfx(sound or 0x71)   -- SND_Fireball1
end

function retire_projectile(e)
  e.proj_attack = 0
  e.visible = false
end

function projectile_tick(e)
  e.proj_age = e.proj_age + DT
  if e.proj_returns and e.proj_age > 0.9 then
    -- a boomerang: out for 0.9 s, then back to whoever threw it
    local o = ents[e.proj_owner + 1]
    if o then
      local dx, dy = o.x - e.x, o.y - e.y
      local d = math.sqrt(dx * dx + dy * dy)
      local speed = math.sqrt(e.vx * e.vx + e.vy * e.vy)
      if d < 0.6 then retire_projectile(e) return end
      e.vx, e.vy = dx / d * speed, dy / d * speed
    end
  end
  e.x = e.x + e.vx * DT
  e.y = e.y + e.vy * DT
  if math.abs(e.vx) > math.abs(e.vy) then e.orientation = e.vx > 0 and 1 or 3 else e.orientation = e.vy > 0 and 2 or 0 end
  -- walls stop it (a returning axe flies on: it is going home)
  if wall_at(e.x, e.y) and not (e.proj_returns and e.proj_age > 0.9) then retire_projectile(e) return end
  if e.proj_age > 6 then retire_projectile(e) end
end

