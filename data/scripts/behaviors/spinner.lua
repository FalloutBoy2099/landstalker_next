-- The spinners (kinds 157 and 170): dash at the player along an axis, bounce off whatever is in the way.

local function spinner(e, c)
  if c.dist <= 18 then
    if e.ai_state == "lunge" then
      e.ai_time = e.ai_time + DT
      local moved = enemy_move(e, e.ai_vx * DT, e.ai_vy * DT, false)
      if not moved or e.ai_time > 1.6 then e.ai_state = "recover"; e.ai_time = 0 end
    else
      e.ai_time = e.ai_time + DT
      if e.ai_time > 0.5 then
        local sp = c.speed * 2.6
        if math.abs(c.dx) >= math.abs(c.dy) then e.ai_vx, e.ai_vy = (c.dx > 0 and 1 or -1) * sp, 0 else e.ai_vx, e.ai_vy = 0, (c.dy > 0 and 1 or -1) * sp end
        e.ai_state = "lunge"
        e.ai_time = 0
      end
    end
  end
end
BOSS_AI[157] = spinner
BOSS_AI[170] = spinner
